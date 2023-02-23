# Becho - Planning


## What does it tries to solve?

Shell scripting is a language that is perfect when writing instructions that
mostly uses system commands. The main disavantage of using it is when you
have to create an interface to use with it.

Commands like `echo`, `fmt` and `fold` would be used, but they would require
complex pipelines, treatments and weird escape sequences to make some
interesting interface which would make your project unmaintainable really
quickly, and if you have to mantain a lot of projects including scripts, it can
be hard to replan your functions to make different types of interface.

`becho` is a terminal utility designed to help you trim, treat, format and
print a text by only using flags. It can be versatile enough to:
  + color text.
  + create bullet list.
  + indent text.
  + fix text's case.
  + remove trimming spacing.


## How it should receive input?

`becho` can accept fragments of text to be handle as arguments:

```bash
becho "hello" "world"
```

When dealing with multiple fragments, `becho` will consider them as just one
block of text by concatenating them. By default, the previous command would be
read by `becho` as the text:

```
helloworld
```

Alternatively, `becho` can get those fragments of text from a pipe line,
which allows you to pipe them from another command, for example, `cat`:

```bash
cat foo.txt | becho
```

When receiving from a pipe line, `becho` will consider it as just one fragment,
instead of considered each of the received words as a fragment.

By default, `becho` can remove indentation spacing:

Previously you would call a command like `echo` like:

```bash
echo "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Donec placerat convallis ornare. Curabitur in tincidunt risus, \
a sodales nibh. Aenean nulla orci, consectetur vitae posuere mattis, \
dapibus in turpis. In dignissim ex eget libero malesuada consectetur."
```

With `becho`, you can reorganize your code and keep the same indentation level
as you were:

```bash
becho "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
       Donec placerat convallis ornare. Curabitur in tincidunt risus,
       a sodales nibh. Aenean nulla orci, consectetur vitae posuere mattis,
       dapibus in turpis. In dignissim ex eget libero malesuada consectetur."
```

What about this one? This also works:

```bash
becho "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Donec placerat convallis ornare. Curabitur in tincidunt risus,
a sodales nibh. Aenean nulla orci, consectetur vitae posuere mattis,
dapibus in turpis. In dignissim ex eget libero malesuada consectetur."
```

And this one too:

```bash
becho "
  Lorem ipsum dolor sit amet, consectetur adipiscing elit.
  Donec placerat convallis ornare. Curabitur in tincidunt risus,
  a sodales nibh. Aenean nulla orci, consectetur vitae posuere mattis,
  dapibus in turpis. In dignissim ex eget libero malesuada consectetur.
"
```

`becho` should remove indentation characters that uses spaces and tabs and extra
newlines in the start and end of the text.

## What is the expected output?

After receiving the fragments of text as argument and creating the one block
of text it will handle, `becho` will treat that text according with the defined
properties and, in the end, will output it back to the standart output.

When creating its output, `becho` will consider the following properties:
  + a left indentation.
  + a prefix.
  + your text.
  + a suffix.
  + a right indentation.
  + a width.
  + a foreground color.
  + a background color.
  + the text's case.

All those properties can be changed by using specific flags, which will be
mentioned later in this document.


## What are the options flag it can accept?


### Separator

You can change the separator `becho` uses when concatenating text fragments
given as arguments by using the flag `--separator`. For example:

```
becho --separator " | " hello world
```

Would output:

```
hello | world
```

The separator will not affect texts comming from the standart input as they
are considered as just one text.

By default, the separator is an empty string.

### Escape Sequences

By default, `becho` will not interpret escape sequences, so characters like `\n`
or `\t` will be printed instead of interpreted. This behavior can be disabled by
using the flag `--escape`. Using it, the previous characters would be interpreted
instead of printed.


### Bold

You can specify that you want the output to use bold text by using the flag
`-b` or `--bold` or.

### Crossed Out

You can specify that you want the output to use crossed out text by using the
flag `-x` or `--crossed-out`.

### Dimmed

You can specify that you want the output to use dimmed color by using the
flag `-d` or `--dim`.

### Italic

You can specify that you want the output to use italic text by using the
flag `-i` or `--italic`.

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
    0 to 255. As curiosity, values from 0 to 15 corresponds to the 4-bits color
    pallete, and can be used instead of using one of the names mentioned in the
    previous item.

Invalid color values are color values that does not correspond to the scope
of both color palletes mentioned previously. When an invalid color is used,
`becho` will just not modify that color.

Terminals that does not supports some of the color palletes will not be
able to see some or all colors.

By default, those flags have the value `normal`.


### Left And Right Indentation

You can use:
+ the flag `-l` or `--left-indentation` to defined a string that will
  be used in the start of every line of the text.
+ the flag `-r` or `--right-indentation` to do the same, but for the end of
  every line instead.

Those flags can be used to indent the text or put a symbol in the start/end of
every line.

As an example, consider a file `foo.txt` that contains the following
text inside of it:

```
barbar bar barbar
barbar bar barbar
barbar bar barbar
```

By using `cat`, that content can be piped to `becho`:

```bash
cat foo.txt | becho -l "  "
```

This time, the flag `-l` was defined, so it will output:

```
  barbar bar barbar
  barbar bar barbar
  barbar bar barbar
```

Note that the string defined was used at the start of each line. The same
example can be done with the `-r` property, but to make it visible, it will be
used a symbol this time. When running the command:

```bash
cat foo.txt | becho -r " <<"
```

It will output:

```
barbar bar barbar <<
barbar bar barbar <<
barbar bar barbar <<
```

Note that this time that symbol was used at the end of each line.

By default, the value of those flags are an empty string.

### Prefix And Suffix

Similar to the indentations flags, there will:
  + the flag `-p` or `--prefix` to define a prefix string.
  + the flag `-s` or `--suffix` to define a sufix string.

The difference between prefix/suffix and indentation is that a prefix is only
show once, instead of being repeated in every line.

Please, consider the same file `foo.txt` from the last section:

```bash
cat foo.txt | becho -p ">->> "
```

This time, the `-p` flag was used. It would output:

```
>->> barbar bar barbar
     barbar bar barbar
     barbar bar barbar
```

Similarly, the same behavior be reproduced in the right side by using the
flag `-s`:

```bash
cat foo.txt | becho -s " <-<<"
```

It would output:

```
barbar bar barbar <<-<
barbar bar barbar
barbar bar barbar
```

Each prefix can be aligned vertically using another flag:
  + to adjust the alignment of the prefix, use the flag `--alignment-prefix`.
  + to adjust the alignment of the suffix, use the flag `--alignment-suffix`.

Those alignment flags use the `top` value by default, but can use `center` and
`bottom` too. Follow examples:

```bash
cat foo.txt | becho --alignment-suffix center -s " <-<<"
```

would output:

```
barbar bar barbar
barbar bar barbar <<-<
barbar bar barbar
```

and:

```bash
cat foo.txt | becho --alignment-suffix bottom -s " <-<<"
```

would output:

```
barbar bar barbar
barbar bar barbar
barbar bar barbar <-<<
```

The prefix flags are really useful when creating bullet list in the terminal,
for example:

```bash
messages=(
  "Visit my friend."
  "Go shopping"
  "Do yoga"
)
for message in "${message[@]}"; do
  becho -p "* " "${message}"
done
```

Would output:

```
* Visit my friend.
* Go shopping
* Do yoga
```

The color of a prefix can not be changed by a flag, but you can use the own
`becho` to color that prefix and use its output as the prefix. If you want,
for example, to colorize the previous example's prefix in red, you
would use:

```bash
becho -p "$(becho -f red "*") " "${message}"
```

As it will be discussed in a next section, you will be able to define
a width for your text to break. When using a prefix/sufix, `becho` does
not wraps your text around it. For example, if you output a lorem ipsum
text with the prefix ">->>", it would behavior something like this:

```
>->> Lorem ipsum dolor sit amet, consectetur adipiscing elit.
     Donec placerat convallis ornare. Curabitur in tincidunt risus,
     a sodales nibh. Aenean nulla orci, consectetur vitae posuere mattis,
     dapibus in turpis. In dignissim ex eget libero malesuada consectetur.
```

This behavior can be changed by using the `--wrap-around` flag. Using
it in the same example would output:

```
>->> Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Donec placerat convallis ornare. Curabitur in tincidunt risus,
a sodales nibh. Aenean nulla orci, consectetur vitae posuere mattis,
dapibus in turpis. In dignissim ex eget libero malesuada consectetur.
```

Note that, now, the text wraps around the prefix. Same behavior happens in
the right side if you use a suffix.

As a matter of curiosity, an output can have both a prefix and suffix and both
in same or different alignments:

```bash
becho -p ">->>" --alignment-suffix bottom -s "<-<<" "hello" "world"
```

would output

```
>->> helloworld <-<<
```

You can not have a more than one prefix or suffix even in different alignments.

If you use an escape sequence in a prefix/suffix, it will be printed instead
of be interpreted.

### Cases


`becho` can be used to handle the case of your text. The case of your text can
be defined by using the flag `-c` or `--case`. It uses `normal` as its default
value, but can be changed to:
  + `upper_case`.
  + `lower_case`.
  + `snake_case`.
  + `camel_case`.
  + `kebab_case`.
  + `pascal_case`.

### Width


The width property is the most important one, and will allow you to fold
your text in a desired width. In this section you will also see the
composition of the output containing all the elements mentioned previously.

That composition is as following:

```
left_indentation prefix text suffix right_indentation
```
> Composition with all the elements (case 1).

By default, `becho` will consider that the sum of those element characters must
be equivalent to its defined width property or less than it. If it exceeds the
width property, a line break will be inserted. The default width value is
80, but can be changed with the flag `--width`.

Considering the default behavior of prefix/suffix, `becho` will consider
the size of prefix/suffix in all of the lines, however if the `--wrap-around`
flag was used, `becho` will have to analyze each line and provide a different
behavior for each one: now, it must consider prefix and/or suffix only when
it will be actually used, as a space will not be placed to substitute them in
each line. This implies that these other forms of composition can be found:

```
left_indentation prefix text right_indentation
```
> Composition with just prefix (case 2).

```
left_indentation text suffix right_indentation
```
> Composition with just suffix (case 3).

```
left_indentation text right_indentation
```
> Composition without prefix/suffix (case 4).

As an example, take a look in the following lorem ipsum text that is using
the `--wrap-around` with some width defined:

```
(case 2) >> >->> Lorem ipsum dolor sit amet, consectetur adipiscing      <<
(case 4) >> elit. Donec placerat convallis ornare. Curabitur in          <<
(case 4) >> tincidunt risus, a sodales nibh. Aenean nulla orci,          <<
(case 3) >> consectetur vitae posuere mattis, dapibus in turpis. In <-<< <<
(case 4) >> dignissim ex eget libero malesuada onsectetur.               <<
```

In this example, the cases 2, 3 and 4 could be found, which ilustrates some
of the different compositions `becho` will have to maintain.

Now, considering only the text part of one of those lines, `becho` will try
to fit your text inside it. By default, `becho` tries to fit a whole word in
a line so it can be readable. This behavior implies that the minimum size of
a text per line must be the size of the longest word present in that line.

This behavior can be disabled by using the flag `--wrap-by-character` which
will make `becho` try to fit characters inside of a line. Using it, the minimum
size for the text is only one character.

Considering all these scenarios, if `becho` detects that your text can not fit
in that space, considering if that line uses a prefix/suffix, it will throw an
error and show you a good spacing.

What about special sequences like `\n` and `\t`? As describe in the start of
this document, `becho` can interpret them if the flag `--escape` is used, how it
will handle the line break and the spacing of a tab character?

When interpreting those special sequences:
  + if `becho` finds a `line break`, it will break the line in that point an
    will start analyzing the next one inserted.
  + if `becho` find a `tab`, it will substitute it by 2 spaces. You can not
    change how much spacing characters `becho` will use for a `tab`. The best
    solution in this case is to use spaces to ensure the desired width.
  + others characters will be printed normally.

If `becho` is not interpreting those sequences, it will consider each of them
as part of the word they are closed to or as separate words if they are stand
alone.

For example:
If `becho` is considering special sequences, when it receive:

```
Hello,\n\x1b[1;31mworld\x1b[0m\t.
```

It will transform it to:

```
Hello,
world  .
```

If it its not interpreting special sequences, it will consider that same text
as the words: `Hello,` and `\n\x1b[1;31mworld\x1b[0m\t.`.


### Alignments

You can define the alignment that `becho` will use for each line of your text.
This alignment only affects the text element of its output. By default, `becho`
uses `left` alignment, but it can be changed using the flag `--alignment`. Other
values are: `center` and `right`.

Alignments other than `left` will only be visible if a spacing will be left
in the right side of the line when `becho` wraps your text. This means that
defining an alignment for the text while using the flag `--wrap-by-character` is
redudant as `becho` will not have space to make that alignment visible.


## Trimming

By default, `becho` will only remove spaces that you left when indent the
text in your code. Spaces inside of the text itself will be preserved but can
be removed with flags:
  + `--trim-start` - removes extra spacing in the start of the text.
  + `--trim-end` - removes extra spacing in the end of the text.
  + `--trim-middle` - removes extra spacing in the middle of the text.
  + `--trim-all` - same as using all the previous flags together: removes all
    the extra spaces from the text.

Spaces and tabs are considered as spacing and will be removed by those flags.


## Repetitions

You can use the flag `--repeat` to specify how many time you want your output
to be printed. By default, its value is 1.


## Line Breaks

By default, `becho` will print new line character in the end of the output.
That can be removed by using the flag `--no-end-line-break`.
