#!/usr/bin/env bash

main() {
  becho -w 80 -l "    $(becho -bf dark_yellow "=>") " "\
Lorem ipsum dolor sit amet, consectetur $(becho -bf dark_red adipiscing elit. \
Morbi non ipsum at nibh maximus imperdiet eget eu nulla). \
$(becho -g dark_yellow -f black Pellentesque aliquam nibh ligula. In \
euismod eget) ipsum et fermentum. Donec non condimentum nibh. Aliquam interdum \
arcu neque, eu tempor dolor gravida sed. $(becho -ui Proin ultrices tortor \
risus). Nunc eleifend tempus libero. Etiam interdum nibh sed venenatis \
maximus. Mauris vehicula orci a ipsum porta, eu convallis ligula malesuada. \
Integer lobortis arcu sit amet pharetra placerat. Donec velit leo, viverra \
eget molestie sit amet, semper ac velit.\
"
}

main
