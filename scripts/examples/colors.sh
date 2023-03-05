#!/usr/bin/env bash

main() {
  local -r text="Here are dragons!"

  for color in $(seq 0 255); do
    becho "(${color}) $(becho -f ${color} ${text})   $(becho -g ${color} ${text})"
  done
}

main
