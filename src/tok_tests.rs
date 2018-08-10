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
fn line_quoted_open_paren() {
    assert_eq!(
        line("foo bar: \"baz( qux\"\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::tag("bar"),
                Open::Colon.into(),
                Token::tag("baz( qux"),
            ])
        ))
    );
}

#[test]
fn line_quoted_close_paren() {
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
fn line_open_close_two_tags() {
    assert_eq!(
        line("foo (bar baz) qux\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::Sigspace,
                Open::Paren.into(),
                Token::tag("bar"),
                Token::tag("baz"),
                Close::Paren.into(),
                Token::tag("qux"),
            ])
        ))
    );
}

#[test]
fn line_open_close_two_tags_last_space() {
    assert_eq!(
        line("foo (bar baz ) qux\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::Sigspace,
                Open::Paren.into(),
                Token::tag("bar"),
                Token::tag("baz"),
                Token::Sigspace,
                Close::Paren.into(),
                Token::tag("qux"),
            ])
        ))
    );
}

#[test]
fn line_open_close_two_tags_last_nospace_after() {
    assert_eq!(
        line("foo (bar baz )qux\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::Sigspace,
                Open::Paren.into(),
                Token::tag("bar"),
                Token::tag("baz"),
                Token::Sigspace,
                Close::Paren.into(),
                Token::tag("qux"),
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
                Token::indent(" "),
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
                Token::indent("   "),
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
                Token::indent(" \t  "),
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
                Token::indent("\t"),
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
                Token::indent("\t\t\t"),
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
            Line(vec![Token::indent("   \t  "), Token::tag("\"Foo\"")])
        ))
    );
}

#[test]
fn bare_escaped_tags() {
    assert_eq!(
        line("foo\\\\bar baz\\\"qux\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo\\bar"),
                Token::tag("baz\"qux"),
            ])
        ))
    );
}

#[test]
fn all_tag_forms() {
    assert_eq!(
        line("foo \"bar \" baz\\\"qux \"zooo\\\\oool\"\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("foo"),
                Token::tag("bar "),
                Token::tag("baz\"qux"),
                Token::tag("zooo\\oool"),
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
                Line(vec![Token::indent("\t\t"), Token::tag("indent")]),
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
                Line(vec![Token::indent("\t\t"), Token::tag("indent")]),
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
                Line(vec![Token::indent("  ")]),
                Line(vec![Token::tag("root")]),
                Line(vec![Token::indent("\t\t"), Token::tag("indent")]),
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
                Line(vec![Token::indent("  ")]),
                Line(vec![Token::indent("  "), Token::tag("root")]),
                Line(vec![Token::indent("\t\t"), Token::tag("indent")]),
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
                Line(vec![Token::indent("  "), Token::tag("root")]),
                Line(vec![Token::indent("\t\t"), Token::tag("indent")]),
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
                Line(vec![Token::indent("  "), Token::tag("root")]),
                Line(vec![Token::indent("\t\t"), Token::tag("indent")]),
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
                Line(vec![Token::indent("  "), Token::tag("root")]),
                Line(vec![Token::indent("\t\t"), Token::tag("indent")]),
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

#[test]
fn trailing_quote() {
    assert_eq!(
        lines("foo\"\n"),
        Ok((
            "\n",
            vec![Line(vec![Token::tag("foo"), Open::Quote.into()])]
        ))
    );
}

#[test]
fn trailing_quote_and_spaces() {
    assert_eq!(
        line("open\"   \n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Open::Quote.into(),
                Token::tag("   "),
            ])
        ))
    );
}

#[test]
fn trailing_quote_and_tab() {
    assert_eq!(
        line("open\"\t\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Open::Quote.into(),
                Token::tag("\t"),
            ])
        ))
    );
}

#[test]
fn trailing_quote_and_space() {
    assert_eq!(
        line("open\" \n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Open::Quote.into(),
                Token::tag(" "),
            ])
        ))
    );
}

#[test]
fn trailing_quote_and_mixed_whitespace() {
    assert_eq!(
        line("open\" \t \n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Open::Quote.into(),
                Token::tag(" \t "),
            ])
        ))
    );
}

#[test]
fn trailing_quote_and_word() {
    assert_eq!(
        line("open\"ash\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Open::Quote.into(),
                Token::tag("ash"),
            ])
        ))
    );
}

#[test]
fn quoted_tag_and_trailing_quote() {
    assert_eq!(
        line("open\"ash\" bash\"\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Token::tag("ash"),
                Token::tag("bash"),
                Open::Quote.into(),
            ])
        ))
    );
}

#[test]
fn quoted_tag_and_trailing_quote_and_word() {
    assert_eq!(
        line("open\"ash\" bash\"dash\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Token::tag("ash"),
                Token::tag("bash"),
                Open::Quote.into(),
                Token::tag("dash"),
            ])
        ))
    );
}

#[test]
fn quoted_tag_and_trailing_quote_and_escapes() {
    assert_eq!(
        line("open\"ash\" bash\"dash\\\"rash\\\\balderdash\n"),
        Ok((
            "\n",
            Line(vec![
                Token::tag("open"),
                Token::tag("ash"),
                Token::tag("bash"),
                Open::Quote.into(),
                Token::tag("dash\"rash\\balderdash"),
            ])
        ))
    );
}

#[test]
fn big_example() {
    assert_eq!(
        lines(
            "
mon
  name leafward
  affinity creation
  description \"
    Plants healing bombs.
    Standard attack.
    Watch out, it's fragile!
  stride 2
  stamina 3
  recovery 2
  health 50
  abilities
    move
    strike drain:2 damage:standard:2
    bomb drain:3 effect:heal:standard:2 grenadeTimer:2 grenadeHealth:20 range:2 radius:2
"
        ),
        Ok((
            "\n",
            vec![
                Line(vec![]),
                Line(vec![Token::tag("mon")]),
                Line(vec![
                    Token::indent("  "),
                    Token::tag("name"),
                    Token::tag("leafward"),
                ]),
                Line(vec![
                    Token::indent("  "),
                    Token::tag("affinity"),
                    Token::tag("creation"),
                ]),
                Line(vec![
                    Token::indent("  "),
                    Token::tag("description"),
                    Token::Sigspace,
                    Open::Quote.into(),
                ]),
                Line(vec![
                    Token::indent("    "),
                    Token::tag("Plants"),
                    Token::tag("healing"),
                    Token::tag("bombs."),
                ]),
                Line(vec![
                    Token::indent("    "),
                    Token::tag("Standard"),
                    Token::tag("attack."),
                ]),
                Line(vec![
                    Token::indent("    "),
                    Token::tag("Watch"),
                    Token::tag("out,"),
                    Token::tag("it's"),
                    Token::tag("fragile!"),
                ]),
                Line(vec![
                    Token::indent("  "),
                    Token::tag("stride"),
                    Token::tag("2"),
                ]),
                Line(vec![
                    Token::indent("  "),
                    Token::tag("stamina"),
                    Token::tag("3"),
                ]),
                Line(vec![
                    Token::indent("  "),
                    Token::tag("recovery"),
                    Token::tag("2"),
                ]),
                Line(vec![
                    Token::indent("  "),
                    Token::tag("health"),
                    Token::tag("50"),
                ]),
                Line(vec![Token::indent("  "), Token::tag("abilities")]),
                Line(vec![Token::indent("    "), Token::tag("move")]),
                Line(vec![
                    Token::indent("    "),
                    Token::tag("strike"),
                    Token::tag("drain"),
                    Open::Colon.into(),
                    Token::tag("2"),
                    Token::tag("damage"),
                    Open::Colon.into(),
                    Token::tag("standard"),
                    Open::Colon.into(),
                    Token::tag("2"),
                ]),
                Line(vec![
                    Token::indent("    "),
                    Token::tag("bomb"),
                    Token::tag("drain"),
                    Open::Colon.into(),
                    Token::tag("3"),
                    Token::tag("effect"),
                    Open::Colon.into(),
                    Token::tag("heal"),
                    Open::Colon.into(),
                    Token::tag("standard"),
                    Open::Colon.into(),
                    Token::tag("2"),
                    Token::tag("grenadeTimer"),
                    Open::Colon.into(),
                    Token::tag("2"),
                    Token::tag("grenadeHealth"),
                    Open::Colon.into(),
                    Token::tag("20"),
                    Token::tag("range"),
                    Open::Colon.into(),
                    Token::tag("2"),
                    Token::tag("radius"),
                    Open::Colon.into(),
                    Token::tag("2"),
                ]),
            ]
        ))
    );
}
