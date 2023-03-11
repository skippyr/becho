#!/usr/bin/env bash

main() {
  local -r text="Here are dragons!"
  local -r colors=$(($(tput colors) - 1))

  becho "Use $(becho -bf cyan "Ctrl + c") to exit."
  for color in $(seq 0 ${colors}); do
    local counter="($(becho -bf red ${color})/$(becho -bf yellow ${colors})) "
    becho "${counter}$(becho -f ${color} ${text})   $(becho -g ${color} ${text})"
    becho -e "\033[2A"
    sleep 0.2
  done

  becho
  becho
  becho "You terminal could support colors from values 0 to ${colors}."
  becho "That was $(becho -bf yellow cool), don't you think?"
}

main
