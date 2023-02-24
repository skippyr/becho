# Becho

`becho` is a terminal utility designed to help you treat, style and print text
into the standart output. It is my rethink of the `echo` command with more
features.


## Development

This project still under development. However, some features have already been
implement and can be tested by you.

I plan to continue adding more features, but those will be replaned before being
added.

## Installation

`becho` is written in Rust and for to be used, must be installed from source.
To do it, if you are using MacOS or Linux, follow these steps:

  + install [`rust`](https://www.rust-lang.org), `git` and `make`. If you
    are using a MacOS, the last two dependencies can be installed in the
    `development tools` kit. If you are using Linux, your distribution's package
    manager can have those, but you can the installation manually if not.
  + clone this repository using `git`:

    ```bash
    git clone --depth 1 https://github.com/skippyr/becho
    ```

    Use the flag `--depth` with value `1` to specify that you want to download
    only the latest commit.
  + access the repository's directory and use `make` to build and install
    the `becho` and its manual in your system.

    ```bash
    make install
    ```
  + you can the use `becho --help` or `man becho` to obtain usage instructions.
  + if you want to uninstall `becho`, go back to the repository's directory
    and use `make` again, but with a different command:

    ```bash
    make uninstall
    ```

`make` were configured to install `becho` in MacOS and Linux as they use
pratically the same directory structure. In Windows, you may have to separate
the `becho`'s binary in a separate directory and add that directory to your
`${PATH}` environment variable by using your system's settings manually. You
can avoid this, by using it inside of `WSL` (windows subsystem for linux).

Alternatively, if you know how to use a Docker container, there is instructions
to build an image with `becho` already installed in this repository in the
`dockerfile`.

## What it can do?

`becho` can do basically what `echo` is capable with some features that you
may find useful in other utilities like `tr` by just using flags.

It can accept a text to be handled by concatenating the command line arguments
with a separator or from the standart input, through a pipe line. It will do its
magic with the properties you will define with flags and output it back in the
standart output.


### Separator

For example, using the command line arguments:

```bash
becho hello world
```

`becho` will interpret those arguments as the `helloworld`. By default,
`becho` will consider an empty string as the separator to be used when
concatenating the arguments, but it can be changed by using the flag
`-t` or `--separator`, so the command:

```bash
becho -t " | " hello world
```

Now, that will be interpreted as the text `hello | world`. This feature
is really useful to place a separator in the elements of an array.

When receiving text from a pipe line, it will consider it as just one
argument, so the separator flag will not affect it.

For example:

```bash
cat text.txt | becho
```

If you want to use a separator flag in this case, prefer using a subcommand
instead:

```bash
becho -t " | " $(cat text.txt)
```

### Escapes

By default, `becho` will not interpret escape sequences that are in the
arguments of a command line. If you want, there is the flag `-e` or `--escape`
that will make `becho` escape new line and tab sequences. A tab character will
be replaced by two spaces.

### Colors

`becho` is capable of easily setting colors for the foreground and background
by using the flag `-f` or `--foreground-color` and `-g` or `--background-color`
respectively:

Those flags accept the name of a color from the 4-bits pallete or a value
from the 8-bits pallete, which is a value from 0 to 255.

For example:

```bash
becho -f red "foo"
```

You can use a dim variant of the color you chose by using the flag `-d` or
`--dim`.

### Cases

`becho` can change the cases of your text for a lot of variants of cases by
using the flag `-c` or `--case`. For example:

```bash
becho -c snake "hello world"
```

### Bold

You can specify that you want the output to use bold text by using the
flag `-b` or `--bold`.

### Underline

You can specify that you want the output to use underlined text by using the
flag `-u` or `--underline`.

### Italic

You can specify that you want the output to use italic text by using the
flag `-i` or `--italic`.

## More Information

The instructions you saw in this document are just as examples, if want more
details, please refer to the manual of `becho` after you install it.
