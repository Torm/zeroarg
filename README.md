# Zeroarg

Zeroarg is a zero-setup commandline argument parser. It determines operands,
attributes and flags purely from syntax. However, it makes stronger assumptions
and is stricter about syntax than other parsers.

## Usage

Use the `parse_arguments` function to get a `Vec` of operands, flags and attributes.

## Features

- Requires zero setup: operands, attributes and flags are determined entirely from
  syntax.
- Approximately supports traditional syntax, but differs slightly in disallowing
  some common patterns. For example, `=` is always required for values, even for
  short options.
- Simple conventions that require just a small parser.

## Rules & assumptions

- There is no distinction between short and long options.
- Options within an argument can be separated by `+`.
- Options can be prefixed by `--`, `+`.
- Short options can additionally be compounded and prefixed by `-`.
- Attribute values must always be preceded by `=`.

## Supported syntax

#### Operand

```
operand
```

#### Flag

```
--flag
+flag
```

#### Short flag

A short flag consists of a single character. It can be prefixed by a single `-`.

```
-f
```

#### Multiple flags

Multiple flags in the same argument can be delimited by `+`. In such cases, the
argument does not require a prefix.

```
--flag1+flag2
+flag1+flag2
flag1+flag2
```

#### Multiple short flags

```
-abc
```

#### Attribute

```
--attribute=value
+attribute=value
attribute=value
```

#### Short attribute

A short attribute consists of a single character. It can be prefixed by a single `-`.

```
-a=value
```

#### Trailing attribute

```
--flag1+flag2+attribute=value
+flag1+flag2+attribute=value
flag1+flag2+attribute=value
```

#### Trailing short attribute

Here, `a` and `b` are flags and `c` is an attribute.

```
-abc=value
```

#### Operand delimiter

Any argument after an operand delimiter is parsed as an operand.

```
-
--
+
```
