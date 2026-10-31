# Downloads the latest soundex binary and registers it with installed browsers.
$ErrorActionPreference = "Stop"
$d = "$env:LOCALAPPDATA\soundex"
New-Item -ItemType Directory -Force $d | Out-Null
Invoke-WebRequest "https://github.com/ParthibanPillay/soundex/releases/latest/download/soundex-windows-x64.exe" -OutFile "$d\soundex.exe"
& "$d\soundex.exe" install
