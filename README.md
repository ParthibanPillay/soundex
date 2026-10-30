# Soundex

Shows what you're playing on YouTube Music as your Discord status. Linux, macOS and Windows; Chrome, Chromium, Brave, Edge and Helium.

1. Download the binary for your OS and `soundex-extension.zip` from the latest release, and unzip the extension.
2. Run the binary once with `install`: `./soundex-linux-x64 install` (macOS: `chmod +x` first; Windows: `soundex-windows-x64.exe install`). Keep the binary where it is, the browser launches it from there.
3. In your browser open the extensions page, enable Developer mode, "Load unpacked", pick the unzipped extension folder.
4. Open music.youtube.com with Discord running and play something.

Click the Soundex toolbar icon to hide/show the status. Build from source with `cargo build --release`.
