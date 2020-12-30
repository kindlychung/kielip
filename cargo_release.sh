#!/usr/bin/env bash
HELP_OPT_REGEX="^(--help|-h)$"
read -r -d '' help_msg <<EOM
cargo_release.sh: Description
------
Usage:
    cargo_release.sh <binary_name>
    cargo_release.sh --help
EOM

if [[ "$1" =~ $HELP_OPT_REGEX ]]; then
    echo "$help_msg" >&2
    exit 1
fi

binary_name="$1"
binary_path="target/release/$binary_name"
target_path="AppDir/usr/bin/"
cargo build --release && strip "$binary_path" && cp "$binary_path" "$target_path"
