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
    output="${i}x${i}.png"
    echo "Export png: $output ..."
    resvg -w $i -h $i "$svg_file" "$output"  
    i2=$(( i * 2 ))
    output="${i}x${i}@2x.png"
    echo "Export retina png: $output ..."
    resvg -w $i2 -h $i2 "$svg_file" "$output"  
    # "AppDir/usr/share/icons/hicolor/${i}x${i}/apps/${svg_file%.*}.png"
done
# cp "$svg_file" "AppDir/usr/share/icons/hicolor/scalable/apps/${svg_file}"
