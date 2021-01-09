#!/usr/bin/env bash
HELP_OPT_REGEX="^(--help|-h)$"
read -r -d '' help_msg <<EOM
make_appimage_desktop_file.sh: Description
------
Usage:
    make_appimage_desktop_file.sh <arg>
    make_appimage_desktop_file.sh --help
EOM

if [[ "$1" =~ $HELP_OPT_REGEX ]]; then
    echo "$help_msg" >&2
    exit 1
fi

binary_name="$1"
if [[ -z "$binary_name" ]]; then
    echo "You must specify a binary name."
    exit 1
fi
binary_desktop_path="AppDir/usr/share/applications/${binary_name}.desktop"

echo -e "[Desktop Entry]" >"$binary_desktop_path"
echo -e "Type=Application" >>"$binary_desktop_path"
echo -e "Name=$binary_name" >>"$binary_desktop_path"
echo -e "Exec=$binary_name" >>"$binary_desktop_path"
echo -e "Icon=$binary_name" >>"$binary_desktop_path"
echo -e "StartupWMClass=$binary_name" >>"$binary_desktop_path"
echo -e "Categories=Development;" >>"$binary_desktop_path"
