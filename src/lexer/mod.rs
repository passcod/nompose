use nom::{self, Needed};
use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Indent(pub String);

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tag(pub String);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Open {
    Paren,
    Colon,
    Quote,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Close {
    Paren,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Token {
    Indent(Indent),
    Tag(Tag),
    Open(Open),
    Close(Close),
    Sigspace,
}

impl Token {
    pub fn indent(i: &str) -> Self {
        Indent(i.into()).into()
    }

    #[cfg(test)]
    pub fn tag(t: &str) -> Self {
        Tag(t.into()).into()
    }
}

impl From<Indent> for Token {
    fn from(i: Indent) -> Self {
        Token::Indent(i)
    }
}

impl From<Tag> for Token {
    fn from(tag: Tag) -> Self {
        Token::Tag(tag)
    }
}

impl From<Open> for Token {
    fn from(o: Open) -> Self {
        Token::Open(o)
    }
}

impl From<Close> for Token {
    fn from(c: Close) -> Self {
        Token::Close(c)
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Line(pub Vec<Token>);

const NEWLINE: &str = "\r\n";
const SPACING: &str = " \t";

named!(newline<&str, &str>, is_a!(NEWLINE));
named!(spacing<&str, &str>, is_a!(SPACING));

named!(simple_tag<&str, Tag>, map!(
    is_not!(" \t\r\n\":()"),
    |tag| Tag(tag.into())
));

named!(escaped_tag_inner<&str, &str>, alt!(complete!(is_not!("\"\\\r\n")) | eof!()));
named!(escaped_tag<&str, Tag>, map!(
    escaped_transform!(call!(escaped_tag_inner), '\\', escapes),
    Tag
));

named!(escapes<&str, &str>, alt!(
    tag!("\\") => { |_| "\\" } |
    tag!("\"") => { |_| "\"" } |
    tag!("h") => { |_| "☃" }
));

const BARE_ESCAPED_NOTS: &str = " \t\r\n\"\\:()";
named!(bare_escaped_tag_inner<&str, &str>, alt!(complete!(is_not!(" \t\r\n\"\\:()")) | eof!()));
named!(bare_escaped_str<&str, String>, escaped_transform!(call!(bare_escaped_tag_inner), '\\', escapes));

named!(bare_escaped_tag<&str, Tag>, map!(
    do_parse!(
        first: none_of!(BARE_ESCAPED_NOTS) >>
        rest: bare_escaped_str >>
        (first, rest)
    ),
    |(first, rest)| Tag(format!("{}{}", first, rest))
));

named!(bare_escaped_tag_with_starting_escape<&str, Tag>, map!(
    do_parse!(
        tag!("\\") >>
        escape: escapes >>
        rest: opt!(alt!(bare_escaped_tag | bare_escaped_tag_with_starting_escape)) >>
        (escape, rest)
    ),
    |(escape, rest)| Tag(format!("{}{}", escape, rest.unwrap_or_else(|| Tag("".into()))))
));

named!(quoted_tag<&str, Tag>, delimited!(
    tag!("\""),
    escaped_tag,
    tag!("\"")
));

named!(a_tag<&str, Tag>, alt!(
    quoted_tag |
    bare_escaped_tag_with_starting_escape |
    bare_escaped_tag |
    simple_tag
));

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
        s: opt!(escaped_tag) >>
        (s)
    )) >>
    (i, toks, trailq)
), |(i, aroar, trailq)| {
    let mut toks = vec![];

    if !i.is_empty() {
        toks.push(Token::indent(i));
    }

    for ar in aroar {
        for tok in ar {
            toks.push(tok);
        }
    }

    if let Some(q) = trailq {
        toks.push(Open::Quote.into());
        if let Some(t) = q {
            if !t.0.is_empty() {
                toks.push(t.into());
            }
        }
    }

    Line(toks)
}));

named!(lines<&str, Vec<Line> >, map!(do_parse!(
    nls: many0!(alt!(tag!("\r") | tag!("\r\n") | tag!("\n"))) >>
    lines: separated_list_complete!(newline, line) >>
    (nls, lines)
), |(nls, mut lines)| {
    for _ in 0..nls.len() {
        lines.insert(0, Line(vec![]));
    }

    lines
}));

named!(termpose<&str, Vec<Line> >, complete!(lines));

pub fn lex(input: &str) -> Result<Vec<Line>, nom::Err<&str>> {
    if !input.ends_with('\n') {
        return Err(nom::Err::Incomplete(Needed::Size(1)));
    }

    let (rest, done) = termpose(input)?;
    if rest != "\n" { unreachable!() }
    Ok(done)
}
