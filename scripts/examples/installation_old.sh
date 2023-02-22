#!/usr/bin/env bash

red_and_bold() {
  local -r text=$1
  echo -e "\x1b[1;31m${text}\x1b[0m"
}

green_and_bold() {
  local -r text=$1
  echo -e "\x1b[1;32m${text}\x1b[0m"
}

yellow_and_bold() {
  local -r text=$1
  echo -e "\x1b[1;33m${text}\x1b[0m"
}

abort() {
  local -r exit_code=$1
  red_and_bold "Procedure aborted. Nothing has changed."
  exit ${exit_code}
}

confirm() {
  local -r message=$1
  read -p "${message}" confirm
  local -r confirm=$(echo ${confirm} | tr [:upper:] [:lower:] | xargs)
  case ${confirm} in
    y)
      ;;
    *)
      abort 1
      ;;
  esac
}

title() {
  local -r title=$(echo "$1" | tr [:lower:] [:upper:])
  red_and_bold "${title}"
}

folded() {
  local -r text=$1
  echo "${text}" | fmt
}

confirm_installation() {
  title "Confirm"
  folded "  This script will try to install a system in your disk. This can \
lead to loss of data, make sure you make a backup before running it."
  confirm "  Do you confirm the installation? [y/N] "
}

is_installed() {
  local -r package=$1
  pacman -Qi ${package} 2>/dev/null
}

install_packages() {
  title "Installation Of Packages"
  local -r packages=(
    base
    linux
    linux-firmware
    firefox
    figlet
  )
  for package in ${packages[@]}; do
    if [[ $(is_installed ${package}) ]]; then
      echo "  [$(green_and_bold OK)] Package $(yellow_and_bold ${package}) is \
installed. Skipping..."
    else
      echo "  [$(red_and_bold FAILED)] Package $(yellow_and_bold ${package}) \
is missing. Installing..."
    fi
  done
}

main() {
  confirm_installation
  install_packages
}

main

