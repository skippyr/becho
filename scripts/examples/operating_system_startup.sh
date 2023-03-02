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
  esac

  becho "$(becho -b [$(becho -f ${color} ${status})]) ${action} $(becho -b ${text})"
  sleep_randomly
}

main() {
  local -r messages=(
    "OK initiating operating system"
    "OK initiating basic services"
    "OK using DHCP to stablish LAN connection"
    "OK downloading latest software updates"
    "FAIL discovering existing operating systems"
  )
  for message in "${messages[@]}"; do
    log_message ${message}
  done
}

main
