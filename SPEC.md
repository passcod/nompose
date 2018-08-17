# Termpose

### Abstract

TODO

### Status

- Unofficial
- Draft

### Authors

TBC

### Table of Contents

1. [Type system](#1-type-system)
   - [Note on semantics](#note-on-semantics)
2. [Syntax overview](#2-syntax-overview)
3. [Label literals](#3-label-literals)
   1. [Bare labels](#31-bare-labels)
   2. [Escaping](#32-escaping)
   3. [Escaped bare labels](#33-escaped-bare-labels)
   4. [Quoted labels](#34-quoted-labels)
   5. [Multiline labels](#35-multiline-labels)
4. [Indenting](#4-indenting)
5. [Parenthesis](#5-parenthesis)
6. [Colons](#6-colons)

## 1. Type system

Termpose only has a single composite type: the **s-list**.

An s-list has two fields:

| Name  | Contents                        |
|:------|:--------------------------------|
| Head  | Either an s-list or a label.    |
| Tail  | An ordered sequence of s-lists. |

A **label** is a string of zero or more characters, as defined in the character
encoding used by the file. Parsers MUST support ASCII-7, SHOULD support Unicode,
and MAY support more encodings.

There is a special case of s-list where its head is absent and its tail empty:
the **nil s-list**, or just **nil** for short. This is the only time an s-list's
head can be absent.

#### Note on semantics

Termpose ascribes no semantics to data nor layout.

Notably, while the head/tail mechanisms are critical to expressing the termpose
data model, an application may choose to interpret an s-list's child s-lists as
a contiguous array rather than two distinct slots.

Similarly, labels are character strings. Interpreting those into meaningful
types is entirely left to the application to do as and if it pleases. It is not
a termpose concern.

## 2. Syntax overview

A termpose document contains literal labels, which define s-lists with a head
of that label, laid out into structures through three mechanisms, named after
the character kinds they employ to describe structure:

 - whitespace, using lines' or s-lists' surrounding spacing and indenting;
 - parens, with opening, closing, implied, or nested parentheses; and
 - colons, which is a shorthand syntax for some common structures.

Labels can be written in two ways:

 - as literals inside of a line; or
 - as a multiline quote, through the indenting mechanism.

Here is an example of a termpose document:

```
mon
   name "courageous leafward"
   affinity creation
   description "
      Plants healing bombs.
      Standard attack.
      Watch out, it's fragile!
   stride 2
   stamina 3
   recovery 2
   health 50
   abilities
      move
      strike drain:2 effect( damage:2 )
      bomb drain:3 effect( heal:5 slow:1 )
```

## 3. Label literals

There are three ways to write labels in termpose:

### 3.1. Bare labels
[§3.1]: #31-bare-labels

Bare labels are character strings containing any character except for:

 - whitespace,
 - the escape character,
 - parenthesis,
 - colons (0x3A),
 - double-quotes (0x22).

Whitespace MUST include spaces (0x20), newlines (0x0A), carriage-returns (0x0D),
and tabs (0x09).

The escape character is the backslash (0x5C).

Parenthesis are both the opening (0x28) and closing (0x29) parenthesis.

Bare labels can be used and will be parsed as is.

For example, the following are all bare labels:

 - `elephants`
 - `tea-shop`
 - `les_éléphants`
 - `филҳо`
 - `戰爭大象`
 - `~!@#$%^&*`

### 3.2. Escaping
[§3.2]: #32-escaping

A backlash invokes an alternative interpretation of the character following
that backslash. The backslash is called the **escape character**, the character
following it is called the **escape literal**, the combination of the two is
called an **escape sequence** or simply **escape**, and the result is called an
**escaped character**.

The followings escapes MUST be implemented:

| Escape sequence (input) | Escaped character (result) |
|:-----------------------:|:--------------------------:|
|          `\n`           |     a line feed (0xOA)     |
|          `\r`           |  a carriage return (0xOD)  |
|          `\t`           |         a tab (0x09)       |

Escapes not present in this table must be interpreted as outputting the escape
literal as-is. Whitespace escape literals are never allowed in bare labels.

The table below shows some characters that are not formal escapes but are often
used as such:

| Escape sequence (input) | Escaped character (result) |
|:-----------------------:|:--------------------------:|
|          `\\`           |      a backlash (`\`)      |
|          `\"`           |    a double-quote (`"`)    |
|          `\(`           |   an opening paren (`(`)   |
|          `\)`           |    a closing paren (`)`)   |
|          `\:`           |        a colon (`:`)       |

### 3.3. Escaped bare labels

With the escapes given in [§3.2] above, bare labels can include
previously-forbidden characters provided they are suitably escaped, to the
exception of whitespace, which is always forbidden in bare labels.

Escaped bare labels may start with an escape, or may even be composed entirely
of escapes.

For example, these are all valid escaped bare labels:

 - `all\\is\\ashes`
 - `unbalanced\"`
 - `life\nunlife`
 - `data\tforever`
 - `\"magical\"`
 - `\\\\`

### 3.4 Quoted labels

A label enclosed in double-quotes (0x22) can contain whitespace, parenthesis,
and/or colons without restriction nor escaping. Quoted labels can also contain
escape sequences as defined in [§3.2].

All bare labels and escaped bare labels may be quoted without effect.

For example, these are all _single_ valid quoted labels:

 - `"habitual"`
 - `"home sweet home"`
 - `"this is a double quote: \""`
 - `"Incredibles (2)"`
 - `"	 (a single tab)"`
 - `"\t (a single tab)"`
 - `""` (an empty label)

### 3.5 Multiline labels

TODO

 - ```
   "We must
   always
   take sides"
   ```

## 4. Indenting

Indent syntax describes structure through relative indentation.

An s-list followed by another s-list at a higher level of indentation has that
latter s-list as its first **tail** s-list.

Subsequent s-lists at that same indentation level are each others' siblings.

| Termpose | Data |
|:---------|:-----|
| `one` <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;two</code> | <ul><li>head: `one`</li><li>tail: <ol><li><ul><li>head: `two`</li><li>tail: _empty_</li></ul></li></ol></li></ul> |
| `one` <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;two</code> <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;three</code> | <ul><li>head: `one`</li><li>tail: <ol><li><ul><li>head: `two`</li><li>tail: _empty_</li></ul></li><li><ul><li>head: `three`</li><li>tail: _empty_</li></ul></li></ol></li></ul> |
| `one` <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;two</code> <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;three</code> <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;four</code> | <ul><li>head: `one`</li><li>tail: <ol><li><ul><li>head: `two`</li><li>tail: _empty_</li></ul></li><li><ul><li>head: `three`</li><li>tail: _empty_</li></ul></li><li><ul><li>head: `four`</li><li>tail: _empty_</li></ul></li></ol></li></ul> |
