# Termpose

(Unofficial spec)

## 1. Types

Termpose has two types: **atoms** and **s-lists**.

Atoms are strings of zero or more characters, as defined in the character
encoding used by the file. Parsers SHOULD support Unicode, and MAY support more
encodings.

S-Lists are:
- _one_ atom (the **head atom**) and
- _zero or more_ atoms (the **tail atoms**) in source-defined order.

> **Non-normative commentary**
>
> Termpose was designed by inspiration from s-expressions, but is not strictly
> compatible: s-expressions are strictly _pairs_, with _lists_ simply sugar, as
> in `(a b c d)` being the equivalent to `(a (b (c d)))`.
>
> S-expressions are always binary trees. In contrast, Termpose represents
> arbitrary trees, so `(a b c)` and `(a (b c))` are two different structures:
>
> ```
>    A       A
>  /  \      |
> B   C      B
>            |
>            C
> ```

## 2. Atom syntaxes

### 2.1. Bare atoms

Bare atoms are character strings containing any character except for:

- whitespace,
- the escape character,
- parenthesis,
- colons (0x3A),
- double-quotes (0x22).

Whitespace MUST include spaces (0x20), newlines (0x0A), carriage-returns (0x0D),
and tabs (0x09).

The escape character is the backslash (0x5C).

Parenthesis are both the opening (0x28) and closing (0x29) parenthesis.

Bare atoms can be used and will be parsed as is.

For example, the following are all bare atoms:

- `elephants`
- `tea-shop`
- `les_éléphants`
- `филҳо`
- `戰爭大象`
- `~!@#$%^&*`

### 2.2. Escaping

A backlash invokes an alternative interpretation of the character following that
backslash. The backslash is called the **escape character**, the combination of
an escape character and its following character is called an **escape sequence**
or simply **escape**, and the result is called an **escaped character**.

The followings escapes MUST be implemented:

| Escape sequence (input) | Escaped character (result) |
|:-----------------------:|:--------------------------:|
|          `\\`           |      a backlash (`\`)      |
|          `\"`           |    a double-quote (`"`)    |
|          `\n`           |      a newline (0x20)      |
|          `\t`           |         a tab (0x09)       |

Escapes not present in this table MUST halt parsing with an error.

> **Non-normative commentary**
>
> Note that no other escaping is provided for at this time. Notably, to express
> a null byte, it is completely legal to include it as-is in an atom, and §2.1.
> mandates that a parser handles this. However, in C-based parsers, this may
> cause issues. Implementations may wish to provide their own escapes in
> addition to the ones defined here to help in those cases, but should be aware
> a future version of the spec may add escapes that could conflict with these.

### 2.3. Escaped bare atoms

With the escapes given in §2.2. above, bare atoms can include some forbidden
characters provided they are suitably escaped. Escaped bare atoms may start with
an escape, or may even be composed entirely of escapes.

For example, this are all valid escaped bare atoms:

- `all\\is\\ashes`
- `unbalanced\"`
- `life\nunlife`
- `data\tforever`
- `\"magical\"`
- `\\\\`

### 2.4 Quoted atoms

An atom enclosed in double-quotes (0x22) can contain whitespace, parenthesis,
and/or colons without restriction nor escaping. Quoted atoms can also contain
escape sequences as defined in §2.2.

All bare atoms and escaped bare atoms may be quoted without effect.

For example, these are all _single_ valid quoted atoms:

- `"habitual"`
- `"home sweet home"`
- `"this is a double quote: \""`
- `"Incredibles (2)"`
- `"	 (a single tab)"`
- `"\t (a single tab)"`
- ```
"We must
always
take sides"
```
- `""` (an empty atom)

There is a shorthand form of quoted atom, discussed in §3.N.
