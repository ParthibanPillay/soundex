# Soundex

Shows what you're playing on YouTube Music as your Discord status. Linux, macOS and Windows; Chrome, Chromium, Brave, Edge and Helium.

## Install

Grab the files from the [latest release](https://github.com/ParthibanPillay/soundex/releases/latest): `soundex-extension.zip` plus the binary for your OS.

| OS | Binary |
|---|---|
| Linux (x64) | `soundex-linux-x64` |
| macOS, Apple Silicon (M1+) | `soundex-macos-arm64` |
| macOS, Intel | `soundex-macos-x64` |
| Windows (x64) | `soundex-windows-x64.exe` |

**1. Register the binary.** Put it somewhere permanent (the browser launches it from there), then run `install` once.

Linux:
```sh
chmod +x soundex-linux-x64
./soundex-linux-x64 install
```

macOS (use `soundex-macos-x64` on Intel):
```sh
chmod +x soundex-macos-arm64
xattr -d com.apple.quarantine soundex-macos-arm64   # allow the unsigned binary
./soundex-macos-arm64 install
```

Windows (PowerShell):
```powershell
.\soundex-windows-x64.exe install
```

**2. Load the extension.** Unzip `soundex-extension.zip`, open your browser's extensions page (`chrome://extensions`, `brave://extensions`, `edge://extensions`), enable Developer mode, click "Load unpacked" and pick the unzipped folder.

**3. Play something.** Open music.youtube.com with Discord running.

Click the Soundex toolbar icon to hide/show the status.

## Build from source

```sh
cargo build --release
```
