# Termpose

(Unofficial spec)

## 1. Type system

Termpose only has a single, complex, type: the **s-list**.

“Complex” here is used to mean that it is made of separate parts while being a
whole, rather than a single value. It is not a commentary on complexity.

An s-list has three fields, _all of which are optional_:

| Name  | Contents                        |
|:------|:--------------------------------|
| Label | A character string.             |
| Head  | An s-list.                      |
| Tail  | An ordered sequence of s-lists. |

The **label** of an s-list is a string of zero or more characters, as defined in
the character encoding used by the file. Parsers SHOULD support Unicode, and MAY
support more encodings.

A field which is not present in an s-list is said to be _nil_.

When an s-list has all of its fields _nil_, it is said to be a _nil s-list_.

> **Non-normative commentary**
>
> Termpose was designed by inspiration from s-expressions, but is not strictly
> compatible: s-expressions are strictly _pairs_, with _lists_ simply sugar, as
> in `(a b c d)` being the equivalent to `(a (b (c d)))`.
>
> S-expressions are always binary trees. In contrast, Termpose represents
> arbitrary trees with ordered child nodes, so `(a b c)`, `(a c b)`, and
> `(a (b c))` are all different structures:
>
> ```
>      A          A          A
>   1/ 2\      1/ 2\        1|
>   B   C      C    B        B
>                           1|
>                            C
> ```

## 2. Label syntaxes

While it is possible to describe s-lists without a label in termpose directly,
most of the time s-lists are created through their label. There are three ways
to write labels in termpose:

### 2.1. Bare labels

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

### 2.3. Escaped bare labels

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

### 2.4 Quoted labels

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

There is a shorthand form of quoted label, discussed in §3.N.

Note how an empty label (of length zero) is not the same as a _nil_ label.
