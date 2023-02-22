# Becho - Planning


## What does it tries to solve?

Shell scripting is a language that is perfect when writing instructions that
mostly uses system commands. The main disavantage of using it is when you
have to create an interface to use with it.

Commands like `echo`, `fmt` and `fold` would be used, but they would require
complex pipelines, treatments and weird escape sequences to make some
interesting interface which would make your project unmaintainable realy
quickly, and if you have to mantain a lot of projects including scripts, it can
be hard to replan your functions to make different types of interface.

`becho` is a terminal utility designed to help you print and format a piece of
text with just a few flags.


## How it should receive input?

`becho` can accept fragments of text to be handle as arguments:

```bash
becho "hello" "world"
```

When dealing with multiple fragments, `becho` will consider them as just one
block of text by concatenating them with a line break. The previous command
would be read by `becho` as the text:

```
hello
world
```

Alternatively, `becho` can get those fragments of text from a pipe line,
which allows you to pipe them from another command, for example, `cat`:

```bash
cat foo.txt | becho
```

When receiving from a pipeline, `becho` will consider it as just one fragment,
instead of considered each of the received words as a fragment.


### What is the expected output?

After receiving the fragments of text as argument and creating the one block
of text it will handle, `becho` will treat that text according with the defined
properties and, in the end, will output it back to the standart output.

When creating its output, `becho` will consider the following properties:
  + a left indentation.
  + a left prefix.
  + your text.
  + a right prefix.
  + a right indentation.
  + a width for all the previous elements to fit.
  + a foreground color.
  + a background color.

All those properties can be changed by using specific flags, which will be
mentioned later in this document.

By saying it, `becho` can be versatile enough to:
  + color text.
  + create bullet list.
  + indent text.


## What are the options flag it can accept?


### Escape Sequences

By default, `becho` will interpret escape sequences, so characters like `\n`
or `\t` would affect its output. This behavior can be disabled by using the
flag `--no-escape`. Using it, the previous characters would be printed instead
of interpreted.


### Bold

You can specify that you want the output to use bold text by using the flag
`--bold` or `-b`.


### Foreground And Background Colors

You can specify a color value to be used in the foreground by using the flag
`--foreground-color` or `-f`. The same can be done for the background by using
the flag `--background-color` or `-g`.

The color value can be:
  + the name of a color from the 4-bits color pallete. The available names are:
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
    0 to 256. As curiosity, values from 0 to 16 corresponds to the 4-bits color
    pallete, and can be used instead of using one of the names mentioned in the
    previous item.

Invalid colors values are color values that does not correspond to the scope
of both color palletes listed previously. When an invalid color is used,
`becho` will just not modify that color.

Terminals that does not supports some of the color palletes will not be
able to see some or all colors.

By default, those flags have the value `normal`.


### Left And Right Indentation

The flag `--left-indentation` can be used to defined a string that will
be placed in the start of every line of the text. A similar flag,
`--right-indentation`, will do the same, but for the end of every line
instead. Those flags can be used to indent the text or put a symbol in the
start of every line.

As an example, consider a file `foo.txt` that contains the following
text inside of it:

```
barbar bar barbar
barbar bar barbar
barbar bar barbar
```

By using `cat`, that content can be piped to `becho`:

```bash
cat foo.txt | becho --left-indentation "  "
```

This time, the flag `--left-indentation` was defined, so it will output:

```
  barbar bar barbar
  barbar bar barbar
  barbar bar barbar
```

Note that the string used was used at the start of each line. The same example
can be done with the `--right-indentation` property, but to make it visible,
it will be used a symbol this time. When running the command:

```bash
cat foo.txt | becho --right-indentation="<<"
```

It will output:

```
barbar bar barbar<<
barbar bar barbar<<
barbar bar barbar<<
```

Note that this time that symbol was used at the end of each line. Those flags
can used to indent a text and place a symbol by its side.

By default, the value of those flags are an empty string.

## Prefix And Suffix

Similar to the indentations flags, there will the flags `--left-prefix` and
`--right-prefix` to define a prefix string and right prefix string,
respectively.

The difference between them is that a prefix is only show once, instead of
being repeated in every line.

Please, conside the same file `foo.txt` from the last section:

```bash
cat foo.txt | becho --left-prefix=">->> "
```

This time, the `--left-prefix` flag was used. It would output:

```
>->> barbar bar barbar
     barbar bar barbar
     barbar bar barbar
```

Similarly, the same behavior be reproduced in the right side by using the
flag `--right-prefix`:

```bash
cat foo.txt | becho --right-prefix=" <-<<"
```

would output:

```
barbar bar barbar <<-<
barbar bar barbar
barbar bar barbar
```

Each prefix can be aligned vertically using another flag. To adjust the
alignment of the `--left-prefix` use the flag `--alignment-left-prefix`, and
for the `--right-prefix`, `--alignment-right-prefix`.

Those alignment flags use the `top` value by default, but can use `center` and
`bottom` too.

The prefix flags are really useful when creating bullet list in the terminal,
for example:

```bash
messages=(
  "Visit my friend."
  "Go shopping"
  "Do yoga"
)
for message in "${message[@]}"; do
  becho --left-prefix="* " "${message}"
done
```

Would output:

```
Visit my friend.
Go shopping
Do yoga
```

The color of a prefix can not be changed by a flag, but you can use the own
`becho` to color that prefix and use its output as the prefix. If you want,
for example, to colorize the previous example's left prefix in red, you
would use:

```bash
becho --left-prefix="$(becho --foreground-color red "*") " "${message}"
```


## Width


The width property is the most important one, and will allow you to fold
your text in a desired width.


## Alignments

