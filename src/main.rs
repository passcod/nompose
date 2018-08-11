#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(non_ascii_literal))]

#[macro_use]
extern crate nom;

use lexer::{lex, Line, Token};
use std::fmt;
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
    pub fn new(name: String, indent: String, line: usize, size: usize) -> Self {
        if line == 0 {
            panic!("Tried to manually create the root node");
        }
        Self {
            name,
            indent,
            line,
            children: Vec::with_capacity(size),
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
#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
pub struct Protonode(Arc<RwLock<(Node, Vec<Protonode>, Option<Protonode>)>>);
// This node, children nodes, parent node

impl Protonode {
    fn new(node: Node, parent: Self) -> Self {
        if node.is_root() {
            panic!("Tried to add root node down the tree")
        }
        Protonode(Arc::new(RwLock::new((node, vec![], Some(parent)))))
    }

    pub fn add_node(&mut self, node: Node) {
        let mut proto = self.0.write().unwrap();
        proto.1.push(Self::new(node, self.clone()));
    }

    pub fn step_in(&mut self) -> Self {
        let proto = self.0.read().unwrap();
        if proto.1.is_empty() {
            panic!("Tried to step into nothing")
        }
        proto.1.last().unwrap().clone()
    }

    pub fn step_out(&mut self) -> Option<Self> {
        self.0.read().unwrap().2.clone()
    }
}

impl fmt::Debug for Protonode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let proto = self.0.read().unwrap();
        f.debug_struct("Protonode")
            .field("node", &proto.0)
            .field("children", &proto.1)
            .finish()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Termpose {
    tokens: Vec<Line>,
    pub node: Protonode,
    current_line: usize,
    indent_stack: Vec<String>,
    multiline_open: bool,
    rewound: bool,
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
        if self.rewound {
            panic!("rewound")
        }

        self.load(lex(input)?);
        Ok(())
    }

    /// Load a list of lexed Lines
    pub fn load(&mut self, toks: Vec<Line>) {
        if self.rewound {
            panic!("rewound")
        }

        for tok in toks {
            self.tokens.push(tok);
        }
    }

    fn step_in(&mut self) {
        if self.rewound {
            panic!("rewound")
        }

        self.node = self.node.step_in();
    }

    fn step_out(&mut self) -> bool {
        match self.node.step_out() {
            Some(n) => {
                self.node = n;
                true
            }
            None => false,
        }
    }

    fn current_indent(&self) -> String {
        self.indent_stack.concat()
    }

    /// If you rewind in the middle of a parse you'll corrupt the state
    pub fn rewind(&mut self) {
        self.rewound = true;
        while self.step_out() {}
    }

    /// Process one Line
    pub fn turn(&mut self) -> Result<(), String> {
        if self.rewound {
            panic!("rewound")
        }

        if self.current_line >= self.tokens.len() {
            return Err("at the end of the road".into());
        }

        #[cfg_attr(feature = "cargo-clippy", allow(indexing_slicing))]
        let line = self.tokens[self.current_line].clone();
        self.current_line += 1;

        for token in &line.0 {
            match token {
                Token::Indent(s) => {
                    let ci = self.current_indent();
                    if ci != s.0 {
                        if s.0.len() > ci.len() {
                            // dive
                            // todo: check that ci is a substring of s.0 (otherwise abort!)
                            self.indent_stack.push(s.0.clone());
                            self.step_in();
                        } else if s.0.len() < ci.len() {
                            // rise
                            // todo: check that s.0 is a substring of ci (otherwise abort!)
                            self.indent_stack.pop();
                            self.step_out();
                        } else {
                            return Err("wrong indent despite being at same level".into());
                        }
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
                    self.node.add_node(Node::new(
                        t.0.clone(),
                        self.indent_stack
                            .last()
                            .cloned()
                            .unwrap_or_else(|| "".into()),
                        self.current_line,
                        0,
                    ));
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn main() {
    let mut pose = Termpose::new_from_str("a\n b\n c\n").unwrap();
    println!("\n\n{:#?}", pose.node);
    pose.turn().unwrap();
    println!("\n\n{:#?}", pose.node);
    pose.turn().unwrap();
    println!("\n\n{:#?}", pose.node);
    pose.turn().unwrap();
    println!("\n\n{:#?}", pose.node);
    pose.rewind();
    println!("\n\n{:#?}", pose.node);
}
