#!/usr/bin/env bash

main() {
  local -r cpu_temperatures=(
    45°C
    50°C
    63°C
    71°C
  )
  becho -bf dark_red "CPU Temperatures"
  becho "[ $(becho -t " | " ${cpu_temperatures[@]}) ]"
}

main
