#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]

#[macro_use]
extern crate nom;
use nom::Needed;

#[cfg(test)]
mod tok_tests;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Indent<'a>(&'a str);

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tag(String);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Open {
    Paren,
    Colon,
    Quote,
    Implied,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Close {
    Paren,
    Implied,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Token<'a> {
    Indent(Indent<'a>),
    Tag(Tag),
    Open(Open),
    Close(Close),
    Sigspace,
}

impl<'a> Token<'a> {
    pub fn tag(t: &str) -> Self {
        Tag(t.into()).into()
    }
}

impl<'a> From<Tag> for Token<'a> {
    fn from(tag: Tag) -> Self {
        Token::Tag(tag)
    }
}

impl<'a> From<Open> for Token<'a> {
    fn from(o: Open) -> Self {
        Token::Open(o)
    }
}

impl<'a> From<Close> for Token<'a> {
    fn from(c: Close) -> Self {
        Token::Close(c)
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Line<'a>(Vec<Token<'a>>);

const NEWLINE: &str = "\r\n";
const SPACING: &str = " \t";

named!(newline<&str, &str>, is_a!(NEWLINE));
named!(spacing<&str, &str>, is_a!(SPACING));

named!(simple_tag<&str, Tag>, map!(
    is_not!(" \t\r\n\":("),
    |tag| Tag(tag.into())
));

named!(escaped_tag_inner<&str, &str>, alt!(complete!(is_not!("\"\\\n")) | eof!()));
named!(escaped_tag<&str, Tag>, map!(
    escaped_transform!(call!(escaped_tag_inner), '\\', alt!(
        tag!("\\") => { |_| "\\" } |
        tag!("\"") => { |_| "\"" }
    )),
    Tag
));

named!(quoted_tag<&str, Tag>, delimited!(
    tag!("\""),
    escaped_tag,
    tag!("\"")
));

named!(a_tag<&str, Tag>, alt!(quoted_tag | simple_tag));

named!(multitag<&str, Vec<Tag> >, many1!(do_parse!(
    eat_separator!(SPACING) >>
    tag: a_tag >>
    (tag)
)));

named!(open<&str, Open>, alt!(
    tag!("(") => { |_| Open::Paren } |
    tag!(":") => { |_| Open::Colon }
));

named!(close<&str, Close>, map!(tag!(")"), |_| Close::Paren));

named!(line<&str, Line>, map!(do_parse!(
    i: eat_separator!(SPACING) >>
    toks: many0!(alt!(
        multitag => { |mt: Vec<Tag>| mt.into_iter().map(Token::Tag).collect() } |
        open => { |o: Open| vec![o.into()] } |
        close => { |c: Close| vec![c.into()] } |
        spacing => { |_| vec![Token::Sigspace] }
    )) >>
    trailq: opt!(do_parse!(
        tag!("\"") >>
        s: opt!(is_not!(NEWLINE)) >>
        (s)
    )) >>
    (i, toks, trailq)
), |(i, aroar, trailq)| {
    let mut toks = vec![];

    if !i.is_empty() {
        toks.push(Token::Indent(Indent(i)));
    }

    for ar in aroar {
        for tok in ar {
            toks.push(tok);
        }
    }

    if let Some(q) = trailq {
        toks.push(Open::Quote.into());
        if let Some(s) = q {
            toks.push(Token::tag(s));
        }
    }

    Line(toks)
}));

named!(pub lines<&str, Vec<Line> >, map!(do_parse!(
    nls: many0!(alt!(tag!("\r") | tag!("\r\n") | tag!("\n"))) >>
    lines: separated_list_complete!(newline, line) >>
    (nls, lines)
), |(nls, mut lines)| {
    for _ in 0..nls.len() {
        lines.insert(0, Line(vec![]));
    }

    lines
}));

fn main() {
    println!("open close {:?}", line("foo(bar) baz\n"));
    println!("open close {:?}", line("foo (bar) baz\n"));
}
