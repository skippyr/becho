#!/usr/bin/env bash

abort() {
  local -r exit_code=$1
  becho -bf red "Procedure aborted. Nothing has changed."
  exit ${exit_code}
}

confirm() {
  local -r message=$1
  read -p "${message}" confirm
  local -r confirm=$(becho -c lowercase ${message})
  case ${confirm} in
    y)
      ;;
    *)
      abort 1
      ;;
  esac
}

title() {
  local -r title=$1
  becho -bf red -c uppercase "${title}"
}

confirm_installation() {
  title "Confirm"
  becho -l "  " "This script will try to install a system in your disk. This can
  lead to loss of data, make sure you make a backup before running it."
  confirm -l "  " "Do you confirm the installation? [y/N] "
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
      becho -l "  " "[$(becho -bf green OK)] Package
      $(becho -bf yellow ${package}) is installed. Skipping..."
    else
      becho -l "  " "[$(becho -bf red FAILED)] Package
      $(becho -bf yellow ${package}) is missing. Installing..."
    fi
  done
}

main() {
  confirm_installation
  install_packages
}

main

