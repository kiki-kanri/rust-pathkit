#!/bin/bash

cd "$(realpath "$(dirname "$(readlink -f "$0")")")" || exit 1

git config --replace-all core.filemode true
find . -name target -prune -o \( -type f -exec chmod 600 {} + \)
find . -name target -prune -o \( -type d -exec chmod 700 {} + \)
find . -name target -prune -o \( -name '*.sh' -type f -exec chmod 700 {} + \)
