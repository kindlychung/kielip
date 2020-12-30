#!/usr/bin/env bash
HELP_OPT_REGEX="^(--help|-h)$"
read -r -d '' help_msg <<EOM
make_appimage.sh
------
Usage:
    make_appimage.sh 
    make_appimage.sh --help
EOM

if [[ "$1" =~ $HELP_OPT_REGEX ]]; then
    echo "$help_msg" >&2
    exit 1
fi

if [[ ! -d AppDir ]]; then
    linuxdeploy.app --appdir AppDir
fi
./cargo_release.sh kielip &&
    ./make_appimage_desktop_file.sh kielip &&
    ./make_appimage_icons.sh kielip.svg &&
    VERSION=0.1.0 linuxdeploy.app --appdir AppDir --output appimage
