param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Whatcolor"
)

$ErrorActionPreference = 'Stop'

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$exePath = Join-Path $InstallDir 'whatcolor.exe'
$downloadUrl = 'https://github.com/zyrridian/whatcolor/releases/latest/download/whatcolor.exe'

Write-Host "Downloading to $exePath..."
curl.exe -L --fail --silent --show-error $downloadUrl -o $exePath

Write-Host ''
Write-Host 'Installed.'
Write-Host "Run it with: $exePath"