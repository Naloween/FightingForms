#!/bin/sh
printf '\033c\033]0;%s\a' FightingForms_client
base_path="$(dirname "$(realpath "$0")")"
"$base_path/FightingForms_LinuxClient.x86_64" "$@"
