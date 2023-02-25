# Becho - Planning

This document contains the next features planned for `becho`.

## Indentation

An indentation could be added to all lines of the text by
using the flags:
  + `-l` or `--left_indentation` to indent the start of each line.
  + `-r` or `--right_indentation` to indent the end of each line.

Those flags are very useful to create text that is indented.

## Prefix/Suffix

Unique symbols could be place in the start or end of a line by
using the flags:
  + `-p` or `--prefix` to add a prefix at the start of the line.
  + `-s` or `--suffix` to add a suffix at the end of the line.

Diffently of the indentation flags, a prefix/suffix can only
appear once in the side of the text.

The vertical alignment of prefix and suffix could be adjusted
with the flags `--alignment-prefix` and `--alignment-suffix` respectively. Those flags would have the value `top` as default
but could accept `center` and `bottom` as well.

Prefix/suffix are useful to create bullet list.

## Width

The total width of the text can be changed by using the flag
`--width`. Its default value would be the number of columns in
the terminal.

The width flag would throw an error if the text can not fit
inside that width with all the elements already calculated for
the output.

## Repeat

The flag `--repeat` can be used to define how many times the
output would be shown. By default its value is 1.

## No End Line Break

By default, `becho` will output with a line break in the end
of the line, the flag `-n` or `--no-end-line-break` would remove it.
