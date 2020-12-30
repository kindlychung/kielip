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

svg_file="$1"

if [[ -z "$svg_file" ]]; then
    echo "You must specify an svg file."
    exit 1
fi

for i in 16 32 64 128 256; do
    resvg -w $i -h $i "$svg_file" "AppDir/usr/share/icons/hicolor/${i}x${i}/apps/${svg_file%.*}.png"
done
cp "$svg_file" "AppDir/usr/share/icons/hicolor/scalable/apps/${svg_file}"
