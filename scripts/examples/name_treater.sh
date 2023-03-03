#!/usr/bin/env bash

main() {
  becho "[$(becho -bf dark_blue ?)] What is your name?"
  becho -n "$(becho -bf dark_green "¦ ")"
  read name
  becho "Your name treated is $(becho -bic title -f dark_red ${name})."
}

main
