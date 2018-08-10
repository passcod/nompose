# nompose

[![Travis CI](https://flat.badgen.net/travis/passcod/nompose/next)][build]
[![License: Artistic 2.0](https://flat.badgen.net/badge/license/Artistic%202.0/purple)][artistic]

Two-pass [termpose] parser.

1. Lex into token lists
2. Parse into tree(s)

Goals:

 - Use [nom]
 - Zero unsafe
 - Tested

[artistic]: ./LICENSE
[build]: https://travis-ci.org/passcod/nompose
[nom]: https://github.com/Geal/nom
[termpose]: https://github.com/makoConstruct/termpose
