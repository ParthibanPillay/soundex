# Soundex

Shows what you're playing on YouTube Music as your Discord status. Linux and Windows; Chrome, Chromium, Brave, Edge, Helium and Firefox.

## Install

**1. Run the installer.** It downloads the binary and registers it with your browsers. Nothing else to do.

Linux:
```sh
curl -fsSL https://raw.githubusercontent.com/ParthibanPillay/soundex/main/install.sh | sh
```

Windows (PowerShell):
```powershell
irm https://raw.githubusercontent.com/ParthibanPillay/soundex/main/install.ps1 | iex
```

**2. Add the extension.** Download `soundex-extension.zip` from the [latest release](https://github.com/ParthibanPillay/soundex/releases/latest) and unzip it.
- Chrome, Chromium, Brave, Edge, Helium: open `chrome://extensions` (or `brave://`, `edge://`), enable Developer mode, "Load unpacked", pick the folder.
- Firefox (140+): open `about:debugging` → This Firefox → Load Temporary Add-on, pick `manifest.json`.

Then open music.youtube.com with Discord running. Click the Soundex toolbar icon to hide/show the status.

To update, run step 1 again.

## Build from source

```sh
cargo build --release
```
