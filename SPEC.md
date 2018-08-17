# Termpose

(Unofficial spec)

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

A termpose document contains literal labels, which define s-lists with a head of
that label, laid out into structures through three mechanisms:

 - indenting, where a line's leading whitespace describes structure in context;
 - parenthesis, where `(` and `)` describe structure; and
 - colons (`:`), which enable a shorthand syntax for some common structures.

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

A backlash invokes an alternative interpretation of the character following that
backslash. The backslash is called the **escape character**, the combination of
an escape character and its following character is called an **escape sequence**
or simply **escape**, and the result is called an **escaped character**.

The followings escapes MUST be implemented:

| Escape sequence (input) | Escaped character (result) |
|:-----------------------:|:--------------------------:|
|          `\\`           |      a backlash (`\`)      |
|          `\"`           |    a double-quote (`"`)    |
|          `\n`           |     a line feed (0xOA)     |
|          `\r`           |  a carriage return (0xOD)  |
|          `\t`           |         a tab (0x09)       |

Escapes not present in this table MUST either:

 - halt parsing with an error, OR
 - trigger a warning AND be ignored such that the character following the escape
   character is output instead of the escape sequence.

> **Non-normative commentary**
>
> Note that no other escaping is provided for at this time. Notably, to express
> a null byte, it is completely legal to include it as-is in an atom, and §2.1.
> mandates that a parser handles this. However, in C-based parsers, this may
> cause issues. Implementations may wish to provide their own escapes in
> addition to the ones defined here to help in those cases, but should be aware
> a future version of the spec may add escapes that could conflict with these.

### 3.3. Escaped bare labels

With the escapes given in §2.2. above, bare labels can include some forbidden
characters provided they are suitably escaped. Escaped bare labels may start with
an escape, or may even be composed entirely of escapes.

For example, this are all valid escaped bare labels:

 - `all\\is\\ashes`
 - `unbalanced\"`
 - `life\nunlife`
 - `data\tforever`
 - `\"magical\"`
 - `\\\\`

### 3.4 Quoted labels

A label enclosed in double-quotes (0x22) can contain whitespace, parenthesis,
and/or colons without restriction nor escaping. Quoted labels can also contain
escape sequences as defined in §2.2.

All bare labels and escaped bare labels may be quoted without effect.

For example, these are all _single_ valid quoted labels:

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
 - `""` (an empty label)

### 3.5 Multiline labels

TODO

## 4. Indenting

Indent syntax describes s-lists through labels and their relative indentation.

A character string, as defined in §2, on its own describes an s-list with a
label of that string.

| Termpose | Data |
|:---------|:-----|
| `label` | <ul><li>head: `label`</li><li>tail: _empty_</li></ul> |

Two s-lists at the same level of indentation are siblings:

| Termpose | Data |
|:---------|:-----|
| `one` <br> `two` | <ol><li><ul><li>head: `one`</li><li>tail: _empty_</li></ul></li><li><ul><li>head: `two`</li><li>tail: _empty_</li></ul></li></ol> |

An s-list followed by another s-list at a higher level of indentation has that
latter s-list as its first **tail** s-list. Subsequent s-lists at that same
indentation level are added to the first s-list's tail:

| Termpose | Data |
|:---------|:-----|
| `one` <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;two</code> | <ul><li>head: `one`</li><li>tail: <ol><li><ul><li>head: `two`</li><li>tail: _empty_</li></ul></li></ol></li></ul> |
| `one` <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;two</code> <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;three</code> | <ul><li>head: `one`</li><li>tail: <ol><li><ul><li>head: `two`</li><li>tail: _empty_</li></ul></li><li><ul><li>head: `three`</li><li>tail: _empty_</li></ul></li></ol></li></ul> |
| `one` <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;two</code> <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;three</code> <br> <code>&nbsp;&nbsp;&nbsp;&nbsp;four</code> | <ul><li>head: `one`</li><li>tail: <ol><li><ul><li>head: `two`</li><li>tail: _empty_</li></ul></li><li><ul><li>head: `three`</li><li>tail: _empty_</li></ul></li><li><ul><li>head: `four`</li><li>tail: _empty_</li></ul></li></ol></li></ul> |
