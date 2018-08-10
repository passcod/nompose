use super::*;

#[test]
fn simple_tag_partial_read() {
    assert_eq!(
        simple_tag("Hello, world!"),
        Ok((" world!", Tag("Hello,".into())))
    );
}

#[test]
fn escaped() {
    assert_eq!(
        escaped_tag("Foo\\\"bar \\\\baz \n"),
        Ok(("\n", Tag("Foo\"bar \\baz ".into())))
    );
}

#[test]
fn quoted() {
    assert_eq!(
        quoted_tag("\"Foo\\\"bar \\\\baz \"\n"),
        Ok(("\n", Tag("Foo\"bar \\baz ".into())))
    );
}

#[test]
fn multitag_1() {
    assert_eq!(multitag("a\n"), Ok(("\n", vec![Tag("a".into())])));
}

#[test]
fn multitag_2() {
    assert_eq!(
        multitag("\"ash\"neo\n"),
        Ok(("\n", vec![Tag("ash".into()), Tag("neo".into())]))
    );
}

#[test]
fn multitag_3() {
    assert_eq!(
        multitag("\"ash\" neo pun\n"),
        Ok((
            "\n",
            vec![Tag("ash".into()), Tag("neo".into()), Tag("pun".into())]
        ))
    );
}

#[test]
fn multitag_4() {
    assert_eq!(
        multitag("\"ash\" neo pun \"back\\\"slash\"\n"),
        Ok((
            "\n",
            vec![
                Tag("ash".into()),
                Tag("neo".into()),
                Tag("pun".into()),
                Tag("back\"slash".into()),
            ]
        ))
    );
}

#[test]
fn open_paren() {
    assert_eq!(open("("), Ok(("", Open::Paren)));
}

#[test]
fn open_colon() {
    assert_eq!(open(":"), Ok(("", Open::Colon)));
}

#[test]
fn close_paren() {
    assert_eq!(close(")"), Ok(("", Close::Paren)));
}

#[test]
fn line_single() {
    assert_eq!(line("foo\n"), Ok(("\n", Line(vec![Token::tag("foo")]))));
}

#[test]
fn line_multi() {
    assert_eq!(
        line("foo bar baz\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::tag("bar"),
                Token::tag("baz"),
            ])
        ))
    );
}

#[test]
fn line_colon() {
    assert_eq!(
        line("foo:\n"),
        Ok(("\n", Line(vec![Token::tag("foo"), Open::Colon.into()])))
    );
}

#[test]
fn line_colon_multi() {
    assert_eq!(
        line("foo: bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Open::Colon.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
fn line_trailing_colon() {
    assert_eq!(
        line("foo bar:\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::tag("bar"),
                Open::Colon.into(),
            ])
        ))
    );
}

#[test]
fn line_quoted_colon() {
    assert_eq!(
        line("foo bar: \"baz: qux\"\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::tag("bar"),
                Open::Colon.into(),
                Token::tag("baz: qux"),
            ])
        ))
    );
}

#[test]
fn line_quoted_paren() {
    assert_eq!(
        line("foo bar: \"baz) qux\"\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::tag("bar"),
                Open::Colon.into(),
                Token::tag("baz) qux"),
            ])
        ))
    );
}

#[test]
fn line_open_paren() {
    assert_eq!(
        line("foo(bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Open::Paren.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
#[ignore = "FIXME should pass"]
fn line_open_close() {
    assert_eq!(
        line("foo(bar) baz\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Open::Paren.into(),
                Token::tag("bar"),
                Close::Paren.into(),
                Token::tag("baz"),
            ])
        ))
    );
}

#[test]
#[ignore = "FIXME should pass"]
fn line_open_close_sigspace() {
    assert_eq!(
        line("foo (bar) baz\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::Sigspace,
                Open::Paren.into(),
                Token::tag("bar"),
                Close::Paren.into(),
                Token::tag("baz"),
            ])
        ))
    );
}

#[test]
fn line_sigspace_two_opens_quoted_paren() {
    assert_eq!(
        line("foo (bar: \"baz) qux\"\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::Sigspace,
                Open::Paren.into(),
                Token::tag("bar"),
                Open::Colon.into(),
                Token::tag("baz) qux"),
            ])
        ))
    );
}

#[test]
fn line_sigspace_leading_colon() {
    assert_eq!(
        line("foo :bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::Sigspace,
                Open::Colon.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
fn line_space_indent() {
    assert_eq!(
        line(" foo :bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::Indent(Indent(" ")),
                Token::tag("foo"),
                Token::Sigspace,
                Open::Colon.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
fn line_spaces_indent() {
    assert_eq!(
        line("   foo :bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::Indent(Indent("   ")),
                Token::tag("foo"),
                Token::Sigspace,
                Open::Colon.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
fn line_mixed_indent() {
    assert_eq!(
        line(" \t  foo :bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::Indent(Indent(" \t  ")),
                Token::tag("foo"),
                Token::Sigspace,
                Open::Colon.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
fn line_tabs_indent() {
    assert_eq!(
        line("\tfoo :bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::Indent(Indent("\t")),
                Token::tag("foo"),
                Token::Sigspace,
                Open::Colon.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
fn line_tab_indent() {
    assert_eq!(
        line("\t\t\tfoo :bar\n"),
        Ok((
            "\n",
            Line(vec![
                Token::Indent(Indent("\t\t\t")),
                Token::tag("foo"),
                Token::Sigspace,
                Open::Colon.into(),
                Token::tag("bar"),
            ])
        ))
    );
}

#[test]
fn line_quoted_quotes() {
    assert_eq!(
        line("   \t  \"\\\"Foo\\\"\"\n"),
        Ok((
            "\n",
            Line(vec![
                Token::Indent(Indent("   \t  ")),
                Token::tag("\"Foo\""),
            ])
        ))
    );
}

#[test]
fn lines_triple() {
    assert_eq!(
        lines("foo\nbar\nbaz\n"),
        Ok((
            "\n",
            vec![
                Line(vec![Token::tag("foo")]),
                Line(vec![Token::tag("bar")]),
                Line(vec![Token::tag("baz")]),
            ]
        ))
    );
}

#[test]
fn lines_char() {
    assert_eq!(lines("1\n"), Ok(("\n", vec![Line(vec![Token::tag("1")])])));
}

#[test]
fn lines_single() {
    assert_eq!(
        lines("one\n"),
        Ok(("\n", vec![Line(vec![Token::tag("one")])]))
    );
}

#[test]
fn lines_single_two() {
    assert_eq!(
        lines("tw o\n"),
        Ok(("\n", vec![Line(vec![Token::tag("tw"), Token::tag("o")])]))
    );
}

#[test]
fn lines_double_two() {
    assert_eq!(
        lines("th r\nee\n"),
        Ok((
            "\n",
            vec![
                Line(vec![Token::tag("th"), Token::tag("r")]),
                Line(vec![Token::tag("ee")]),
            ]
        ))
    );
}

#[test]
fn lines_second_indent() {
    assert_eq!(
        lines("root\n\t\tindent\n"),
        Ok((
            "\n",
            vec![
                Line(vec![Token::tag("root")]),
                Line(vec![Token::Indent(Indent("\t\t")), Token::tag("indent")]),
            ]
        ))
    );
}

#[test]
fn lines_initial_blank() {
    assert_eq!(
        lines("\nroot\n\t\tindent\n"),
        Ok((
            "\n",
            vec![
                Line(vec![]),
                Line(vec![Token::tag("root")]),
                Line(vec![Token::Indent(Indent("\t\t")), Token::tag("indent")]),
            ]
        ))
    );
}

#[test]
fn lines_initial_just_indent() {
    assert_eq!(
        lines("  \nroot\n\t\tindent\n"),
        Ok((
            "\n",
            vec![
                Line(vec![Token::Indent(Indent("  "))]),
                Line(vec![Token::tag("root")]),
                Line(vec![Token::Indent(Indent("\t\t")), Token::tag("indent")]),
            ]
        ))
    );
}

#[test]
fn lines_initial_just_indent_then_second_indent() {
    assert_eq!(
        lines("  \n  root\n\t\tindent\n"),
        Ok((
            "\n",
            vec![
                Line(vec![Token::Indent(Indent("  "))]),
                Line(vec![Token::Indent(Indent("  ")), Token::tag("root")]),
                Line(vec![Token::Indent(Indent("\t\t")), Token::tag("indent")]),
            ]
        ))
    );
}

#[test]
fn lines_initial_blank_then_second_indent() {
    assert_eq!(
        lines("\n  root\n\t\tindent\n"),
        Ok((
            "\n",
            vec![
                Line(vec![]),
                Line(vec![Token::Indent(Indent("  ")), Token::tag("root")]),
                Line(vec![Token::Indent(Indent("\t\t")), Token::tag("indent")]),
            ]
        ))
    );
}

#[test]
fn lines_initial_indent() {
    assert_eq!(
        lines("  root\n\t\tindent\n"),
        Ok((
            "\n",
            vec![
                Line(vec![Token::Indent(Indent("  ")), Token::tag("root")]),
                Line(vec![Token::Indent(Indent("\t\t")), Token::tag("indent")]),
            ]
        ))
    );
}

#[test]
fn lines_multi_blank_initials() {
    assert_eq!(
        lines("\n\n\n  root\n\t\tindent\n"),
        Ok((
            "\n",
            vec![
                Line(vec![]),
                Line(vec![]),
                Line(vec![]),
                Line(vec![Token::Indent(Indent("  ")), Token::tag("root")]),
                Line(vec![Token::Indent(Indent("\t\t")), Token::tag("indent")]),
            ]
        ))
    );
}

#[test]
fn newlines_lf() {
    assert_eq!(
        lines("a\nb\n"),
        Ok((
            "\n",
            vec![Line(vec![Token::tag("a")]), Line(vec![Token::tag("b")])]
        ))
    );
}

#[test]
fn newlines_cr() {
    assert_eq!(
        lines("a\rb\r"),
        Ok((
            "\r",
            vec![Line(vec![Token::tag("a")]), Line(vec![Token::tag("b")])]
        ))
    );
}

#[test]
fn newlines_crlf() {
    assert_eq!(
        lines("a\r\nb\r\n"),
        Ok((
            "\r\n",
            vec![Line(vec![Token::tag("a")]), Line(vec![Token::tag("b")])]
        ))
    );
}

#[test]
fn newlines_lfcr() {
    assert_eq!(
        lines("a\n\rb\n\r"),
        Ok((
            "\n\r",
            vec![Line(vec![Token::tag("a")]), Line(vec![Token::tag("b")])]
        ))
    );
}
