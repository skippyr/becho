#!/usr/bin/env bash

sleep_randomly() {
  local -r random_value=$((${RANDOM} % 7 + 1))
  sleep 0.${random_value}
}

log_message() {
  local -r status=$1
  local -r action=$2
  local -r text=$(echo $@ | sed "s/$1 $2 //")
  local color=""

  case $(becho -c lower ${status}) in
    ok)
      color=green
      ;;
    fail)
      color=red
      ;;
    warn)
      color=yellow
      ;;
  esac

  becho -l "  $(becho -bf dark_red "*") " -w 80 "$(becho -b [)$(becho -bf ${color} ${status})$(becho -b ]) $(becho -i ${action}) $(becho -b ${text}) ..."
  sleep_randomly
}

main() {
  local -r system_name="example linux"
  local -r system_version=2.0.5
  local -r system_author="Sherman Rofeman"
  becho "$(becho -i -f dark_red -c title ${system_name}) $(becho -bf dark_green ${system_version}) by $(becho -f dark_blue ${system_author})."
  sleep 3

  local -r messages=(
    "OK initiating operating system"
    "OK initiating basic services"
    "OK using DHCP to stablish LAN connection"
    "OK downloading latest software updates"
    "FAIL discovering existing operating systems"
    "OK verifying kernel status"
    "WARN verifying available drivers compatibility"
  )

  for message in "${messages[@]}"; do
    log_message ${message}
  done
}

main
