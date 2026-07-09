param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Whatcolor"
)

$ErrorActionPreference = 'Stop'

$repo = 'zyrridian/whatcolor'
$apiUrl = "https://api.github.com/repos/$repo/releases/latest"
$headers = @{ 'User-Agent' = 'whatcolor-installer' }

Write-Host 'Finding latest release...'
$release = Invoke-RestMethod -Uri $apiUrl -Headers $headers
$asset = $release.assets | Where-Object { $_.name -eq 'whatcolor.exe' } | Select-Object -First 1

if (-not $asset) {
    throw 'Could not find whatcolor.exe in the latest release.'
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$exePath = Join-Path $InstallDir 'whatcolor.exe'

Write-Host "Downloading to $exePath..."
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $exePath -Headers $headers

Write-Host ''
Write-Host 'Installed.'
Write-Host "Run it with: $exePath"