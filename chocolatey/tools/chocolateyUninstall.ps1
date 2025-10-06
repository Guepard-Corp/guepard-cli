# Guepard CLI Chocolatey Package Uninstallation Script

$ErrorActionPreference = 'Stop'

$packageName = 'guepard'
$binDir = Join-Path $env:ChocolateyInstall "bin"
$exePath = Join-Path $binDir "guepard.exe"

if (Test-Path $exePath) {
    Remove-Item $exePath -Force
    Write-Host "Guepard CLI uninstalled successfully!"
} else {
    Write-Host "Guepard CLI was not found in the expected location."
}
