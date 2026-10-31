#!/bin/sh
# Downloads the latest soundex binary for Linux and registers it with installed browsers.
set -e
case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) f=soundex-linux-x64 ;;
  *) echo "unsupported platform: $(uname -sm)" >&2; exit 1 ;;
esac
d="${XDG_DATA_HOME:-$HOME/.local/share}/soundex"
mkdir -p "$d"
curl -fsSL "https://github.com/ParthibanPillay/soundex/releases/latest/download/$f" -o "$d/soundex"
chmod +x "$d/soundex"
"$d/soundex" install
