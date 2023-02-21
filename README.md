# Becho

> 🚧 This software is under development. Expect changes and missing features.

"Becho" is a terminal utility written in Rust that aims to be a replacement for
the UNIX `echo` command by providing more useful flags to:

  + align text to left, center or right.
  + style text with colors, bold and blinking.
  + choose left and right indentation of the text.
  + add prefix to the first line or all lines of text.
  + add sufix to the first line or all lines of text.
  + wrap text after a certain number of characters by
    considering only characters or words.


## Installation


  + use `rustup` to install the Rust tools in your system. You can get
    instructions about it in the [official Rust home page](https://www.rust-lang.org/).

  + Clone this repository:

    ```bash
    git clone https://github.com/skippyr/becho
    ```

    While in development, you can access other branches by using:

    ```bash
    git checkout <NAME_OF_THE_BRANCH>
    ```

  + Use `cargo build -r` to compile and create a release of the sofware at
    `target/release/becho`. You can use that file to test the sofware, but if
    you want it available as a command in the terminal, you need to add it
    to your system's `${PATH}` variable.

    Alternatively, you can use:

    ```bash
    cargo run --
    ```

    To build and run the sofware.

## Usage Instructions


You can find instructions about this software by using its
man page file located at `manuals/becho.man`:

```bash
man manuals/becho.man
```

## Testing

After installing it, what about using the example scripts that
are at `scripts/examples`? You can analyze them and execute
to test this software in different environments.

## License

This project is under the MIT license.
