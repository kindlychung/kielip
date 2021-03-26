#!/usr/bin/env bash
HELP_OPT_REGEX="^(--help|-h)$"
read -r -d '' help_msg <<EOM
make_icons.sh: Make png icons from a svg and put them in appropriate locations.
------
Usage:
    make_icons.sh <svg_file>
    make_icons.sh --help
EOM

if [[ "$1" =~ $HELP_OPT_REGEX ]]; then
    echo "$help_msg" >&2
    exit 1
fi

cargo bundle --release