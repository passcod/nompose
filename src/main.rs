#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(non_ascii_literal))]

#[macro_use]
extern crate nom;

mod lexer;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
}

fn main() {
    println!("{:#?}", lexer::lex("a\n b\n c\n").unwrap());
}
