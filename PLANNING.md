# Becho - Planning

## What does it tries to solve?

In certain situations, writing shell script is easier and will perform faster
than using other programming languages. In those cases, if a interface is
needed, building it will require the use of the `echo` command.

The echo command only displays a line of text, which means that to build a
decent interface you will have to use it together with other sofwares such
as `fmt` and `fold`. Also, it only handle colors with escape sequences. This
is the perfect formula to make your project unmaintanable really fast.

Becho is software designed to solve that issue, allowing you to print and
format a piece of text with just a few flags. With it, you will be able
to substitute the use of `echo`, `fmt` and `fold`.

## How it should receive input?

`becho` can accept a piece of text as an argument and will print it to the
standart output. Without any flags, it does not format the text, so it will
behave like `echo`. An example of this use would be:

```bash
becho "foo"
```

Alternatively, Becho should also accept an argument comming from a pipeline,
which will allow it to use text from other sources, for example: if `cat` is
used, `becho` will be able to format and/or print the contents of a file. An
example of this usage is:

```bash
cat foo.txt | becho
```

## What are the options flag it can accept?

### Escape Sequences


By default, `becho` would interpret escape sequences, which will make it easier
to handle text. The flag `--no-escape` could be used to do not allow this
behavior and print the escape sequence instead.


### Bold

By using the flag `--bold`, `becho` will print the text in bold text.
An example of this usage is:

```bash
becho --bold foo
```

### Foreground And Background Colors

The flags `--foreground-color` and `--background-colors` will change the
foreground color and background color, respectively. In both flags, a color
value must be passed.

The color value can be:
  + the name of a color from the 4-bits color pallete. Those values are:
    + `black`
    + `dark_red`
    + `red`
    + `dark_green`
    + `green`
    + `dark_yellow`
    + `yellow`
    + `dark_blue`
    + `blue`
    + `dark_magenta`
    + `magenta`
    + `dark_cyan`
    + `cyan`
    + `white`
    + `normal`, this is the default value and refers to your terminal's default
      colors.
  + The value of a color from the 8-bits color pallete, which is a value from
    0 to 256. Values from 0 to 16 also corresponds to the 4-bits color pallete,
    and can be used instead of using one of the names mentioned in the previous
    item.
Invalid colors values are color values that does not correspond to the scope
of both color palletes listed previously. When an invalid color is used,
`becho` will not modify that color.

Terminals that does not supports some of the color palletes will not be
able to see some or all colors.

Some examples are:

```bash
becho --foreground-color red foo
becho --foreground-color dark_yellow foo
```


### Left And Right Indentation

The flag `--left-indentation` can be used to defined a string that will
be placed in the start of every line of the text. A similar flag,
`--right-indentation`, will do the same, but for in the end of every line
instead. Those flags can be used to indent the text.

As an example, consider that the file `foo.txt` contains the following
text:

```
barbar bar barbar
barbar bar barbar
barbar bar barbar
```

`cat` can be used to pipe that file's contents to `becho`:

```bash
cat foo.txt | becho --left-indentation="  "
```

This command will output:

```
  barbar bar barbar
  barbar bar barbar
  barbar bar barbar
```

Another example can be:

```bash
cat foo.txt | becho --left-indentation="  " --right-indentation="<<"
```

This command will output

```
  barbar bar barbar<<
  barbar bar barbar<<
  barbar bar barbar<<
```

## Prefix And Suffix

Similar to the indentations flags, there will the flags `--left-prefix` and
`--right-prefix` to define a prefix string and right prefix string,
respectively.

Using the same file, `foo.txt` as the previous section, the command:

```bash
cat foo.txt | becho --left-prefix=">->> "
```

would output:

```
>->> barbar bar barbar
     barbar bar barbar
     barbar bar barbar
```

And the command:

```bash
cat foo.txt | becho --right-prefix=" <-<<"
```

would output:

```
barbar bar barbar <<-<
barbar bar barbar
barbar bar barbar
```

Those flags are really useful when creating bullet list in the terminal.

Another flag, `--with-repeat`, can be used to repeat the prefixes in each
line of the text instead of using spaces, not only in the first, so the
previous command, now with this flag:

```bash
cat foo.txt | becho --left-prefix=">->> " --with-repeat
```

would output:

```
>->> barbar bar barbar
>->> barbar bar barbar
>->> barbar bar barbar
```

This behavior is similar to the one made by the command `fmt` with the prefix
flag.

## Alignments

By default, `becho` uses left-alignment, but using the flag `--alignment`, one
of the folling alignments can be used:

  + `center` - centers the text.
  + `right` - makes the text go to the right.

## Width

By using the flag `--width`, you can change the length of each line of text
`becho` will consider. By default, `becho` will consider the `--width` the
same number of columns of your terminal.

`becho` will organize the words to fit in that space by considering each
word's length, however this behavior can be changed by using the flag
`--consider-characters` which make it use each character as reference, which
can lead to broken words as it will wrap it.

If `becho` is considering the word's length, the value of `--width` can not be
less than the longest word you have in the text, otherwise it would not be able
to print that word entirely.

## Trimming

`becho` would maintain any space or tab character leave in the text, which
will allow you to make any kind of spacing inside your text, no trimming will
be made by it.
