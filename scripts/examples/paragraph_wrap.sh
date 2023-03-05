#!/usr/bin/env bash

main() {
  becho -w 80 -l "    $(becho -bf yellow "=>") " "\
Lorem ipsum dolor sit amet, consectetur $(becho -bf red adipiscing elit. \
Morbi non ipsum at nibh maximus imperdiet eget eu nulla). \
$(becho -g yellow -f black Pellentesque aliquam nibh ligula. In \
euismod eget) ipsum et fermentum. Donec non condimentum nibh. Aliquam interdum \
arcu neque, eu tempor dolor gravida sed. $(becho -ui -f green Proin ultrices \
tortor risus). Nunc eleifend tempus libero. Etiam interdum nibh sed venenatis \
maximus. Mauris vehicula orci a $(becho -xf magenta ipsum porta, eu convallis \
ligula malesuada). Integer lobortis arcu sit $(becho -i amet pharetra \
placerat). Donec velit leo, viverra eget molestie sit amet, semper ac velit.\
"
}

main
