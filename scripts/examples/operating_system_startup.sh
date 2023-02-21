#!/usr/bin/env bash

log_status() {
  local -r message=$1
  local -r status=$(echo $1 | cut -f 1 -d " ")
  local color="normal"

  case ${status} in
    OK)
      color="green"
      ;;
    FAILED)
      color="red"
      ;;
    WARNING)
      color="yellow"
      ;;
  esac
  local -r action=$(echo $1 | cut -f 2 -d " ")
  local -r remaining=$(echo $1 | sed "s/${status} ${action} //")
  becho "[$(becho -bf ${color} ${status})] ${action} $(becho -b "${remaining}")"
}

get_random_small_interval() {
  echo "0.$((${RANDOM} % 5))"
}

main() {
  becho "Initializing $(becho -b "operating system") ..."
  sleep 3
  messages=(
    "OK Initializing operating system ..."
    "OK Detecting hardware ..."
    "OK Checking for drivers ..."
    "OK Initializing system services ..."
    "FAILED Checking status of wireless connection ..."
    "OK Checking status of cabled connection ..."
    "OK Creating directory structure ..."
    "WARNING Checking for updates ..."
    "OK Unziping files ..."
    "OK Creating test user ..."
    "WARNING Getting latest news from upstream ..."
    "OK Cleaning up ..."
    "WARNING Checking for audio drivers ... "
    "OK Initializing user interface ..."
  )
  for message in "${messages[@]}"; do
    log_status "${message}"
    sleep $(get_random_small_interval)
  done
  sleep 3
}

main
