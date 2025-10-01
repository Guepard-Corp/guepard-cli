# Guepard CLI Chocolatey Package Uninstallation Script

$ErrorActionPreference = 'Stop'

$packageName = 'gfs'
$binDir = Join-Path $env:ChocolateyInstall "bin"
$exePath = Join-Path $binDir "gfs.exe"

if (Test-Path $exePath) {
    Remove-Item $exePath -Force
    Write-Host "Guepard CLI uninstalled successfully!"
} else {
    Write-Host "Guepard CLI was not found in the expected location."
}
