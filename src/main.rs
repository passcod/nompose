#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(non_ascii_literal))]

#[macro_use]
extern crate nom;

use lexer::{lex, Line, Token};
use std::fmt::{self, Debug};
use std::sync::{Arc, RwLock};

mod lexer;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Node {
    pub name: String,
    pub indent: String,
    pub line: usize,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: String, indent: String, line: usize) -> Self {
        if line == 0 {
            panic!("Tried to manually create the root node");
        }

        Self {
            name,
            indent,
            line,
            children: Vec::with_capacity(0),
        }
    }

    pub fn is_root(&self) -> bool {
        self.line == 0
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: "".into(),
            indent: "".into(),
            line: 0,
            children: Vec::with_capacity(0),
        }
    }
}

#[derive(Clone, Default)]
pub struct Protonodule {
    node: Node,
    children: Vec<Protonode>,
    parent: Option<Protonode>,
}

impl Protonodule {
    pub fn new(node: Node, parent: Protonode) -> Self {
        Self {
            node,
            children: vec![],
            parent: Some(parent),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn last_child(&self) -> Protonode {
        self.children.last().unwrap().clone()
    }

    pub fn add_child(&mut self, protonode: Protonode) {
        self.children.push(protonode);
    }

    pub fn parent(&self) -> Option<Protonode> {
        self.parent.clone()
    }

    pub fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }

    pub fn finalise(&self) -> Node {
        let mut node = self.node.clone();
        node.children = self.children.iter().map(|p| p.finalise()).collect();
        node
    }
}

impl Debug for Protonodule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Protonodule")
            .field("node", &self.node)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Clone, Default)]
pub struct Protonode(Arc<RwLock<Protonodule>>);
// This node, children nodes, parent node

impl Protonode {
    fn new(node: Node, parent: Self) -> Self {
        if node.is_root() {
            panic!("Tried to add root node down the tree")
        }

        Protonode(Arc::new(RwLock::new(Protonodule::new(node, parent))))
    }

    pub fn add_node(&mut self, node: Node) {
        let mut proto = self.0.write().unwrap();
        proto.add_child(Self::new(node, self.clone()));
    }

    pub fn child(&self) -> Option<Self> {
        let proto = self.0.read().unwrap();
        if proto.is_empty() {
            None
        } else {
            Some(proto.last_child())
        }
    }

    pub fn parent(&self) -> Option<Self> {
        self.0.read().unwrap().parent()
    }

    pub fn finalise(&self) -> Node {
        self.0.read().unwrap().finalise()
    }

    pub fn is_root(&self) -> bool {
        self.parent().is_none()
    }
}

impl Debug for Protonode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.read().unwrap().debug(f)
    }
}

#[derive(Clone, Default)]
pub struct Termpose {
    pub tokens: Vec<Line>,
    pub node: Protonode,
    current_line: usize,
    indent_stack: Vec<String>,
    multiline_open: bool,
    just_stepped_in: bool,
}

impl Debug for Termpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Termpose")
            .field("node", &self.node)
            .field("current_line", &self.current_line)
            .field("indent_stack", &self.indent_stack)
            .field("multiline_open", &self.multiline_open)
            .field("just_stepped_in", &self.just_stepped_in)
            .finish()
    }
}

impl Termpose {
    /// Create a new Termpose and start it off from a string
    pub fn new_from_str(input: &str) -> Result<Self, nom::Err<&str>> {
        let mut pose = Self::default();
        pose.load_str(input)?;
        Ok(pose)
    }

    /// Lex a string and load it in
    pub fn load_str<'lex>(&mut self, input: &'lex str) -> Result<(), nom::Err<&'lex str>> {
        self.load(lex(input)?);
        Ok(())
    }

    /// Load a list of lexed Lines
    pub fn load(&mut self, toks: Vec<Line>) {
        for tok in toks {
            self.tokens.push(tok);
        }
    }

    fn step_in(&mut self) {
        if self.just_stepped_in { return; }
        self.just_stepped_in = true;

        self.node = self.node.child()
            .unwrap_or_else(|| panic!("Tried to step into nothing! {:#?}", self));
    }

    fn step_out(&mut self) -> bool {
        self.just_stepped_in = false;
        match self.node.parent() {
            Some(n) => {
                self.node = n;
                true
            }
            None => false,
        }
    }

    fn current_indent(&self) -> String {
        self.indent_stack.last().cloned().unwrap_or_else(|| "".into())
    }

    pub fn finalise(&mut self) -> Node {
        let curr = self.node.clone();
        while self.step_out() {}
        let fin = self.node.finalise();
        self.node = curr;
        fin
    }

    /// Process one Line
    pub fn turn(&mut self) -> Result<bool, String> {
        if self.current_line >= self.tokens.len() {
            return Ok(false);
        }

        #[cfg_attr(feature = "cargo-clippy", allow(indexing_slicing))]
        let line = self.tokens[self.current_line].clone().0;
        let length = line.len();
        self.current_line += 1;

        let mut tag_i = 0;
        let mut preceding_sigspace = false;

        for (i, token) in line.iter().enumerate() {
            #[cfg(debug_assertions)] {
                let debug = format!("{:?}", token);
                let variant = debug.split('(').next().unwrap();
                println!("[{}/{}]\ttag_i: {},  \tjust_stepped_in: {},  \tprev_sigspace: {} \tprocessing: {}", i, length, tag_i, self.just_stepped_in, preceding_sigspace, variant);
            }

            let reset_sigspace = preceding_sigspace;
            match token {
                Token::Indent(s) => {
                    let ci = self.current_indent();
                    if ci != s.0 {
                        if s.0.len() > ci.len() {
                            println!("dive");
                            // todo: check that ci is a substring of s.0 (otherwise abort!)
                            self.indent_stack.push(s.0[ci.len()..].into());
                            self.step_in();
                        } else if s.0.len() < ci.len() {
                            println!("rise");
                            // todo: check that s.0 is a substring of ci (otherwise abort!)
                            self.indent_stack.pop();
                            self.step_out();
                        } else {
                            return Err("wrong indent despite being at same level".into());
                        }
                    } else {
                        println!("same");
                    }
                }
                Token::Open(_) => {
                    self.indent_stack.push("".into());
                    self.step_in();
                }
                Token::Close(_) => {
                    if let Some(peek) = self.indent_stack.last() {
                        if peek != "" {
                            return Err("tried to close an indent with punctuation".into());
                        }
                    } else {
                        return Err("extra close".into());
                    }

                    self.indent_stack.pop();
                    self.step_out();
                }
                Token::Tag(t) => {
                    tag_i += 1;

                    let node = Node::new(
                        t.0.clone(),
                        self.current_indent(),
                        self.current_line,
                    );

                    if tag_i == 2 {
                        self.step_in();
                    }

                    self.node.add_node(node);
                    self.just_stepped_in = false;
                }
                Token::Sigspace => {
                    preceding_sigspace = true;
                }
            }

            if reset_sigspace {
                preceding_sigspace = false;
            }
        }

        #[cfg(debug_assertions)]
        println!("[{}/{}]\ttag_i: {},  \tjust_stepped_in: {},  \tprev_sigspace: {}", length, length, tag_i, self.just_stepped_in, preceding_sigspace);
        Ok(true)
     }
}

fn main() {
    let mut pose = Termpose::new_from_str("
root
    a lot of alots
allowed hallows
    wand
    cape
    rock
").unwrap();
    while pose.turn().unwrap() {
        println!("{:#?}\n\n", pose.finalise());
    }
}
