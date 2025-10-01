# Guepard CLI Chocolatey Package Installation Script

$ErrorActionPreference = 'Stop'

$packageName = 'gfs'
$url = 'https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.1/guepard-cli-0.25.1-windows-amd64.zip'
$checksum = '8abf4cf1877ff6472fbbae15abcae1c55f657f02d895f66b368393e3842a7901'
$checksumType = 'sha256'

# Download and extract
$tempDir = Join-Path $env:TEMP $packageName
if (Test-Path $tempDir) {
    Remove-Item $tempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tempDir | Out-Null

Write-Host "Downloading Guepard CLI..."
$zipFile = Join-Path $tempDir "guepard-cli.zip"
Invoke-WebRequest -Uri $url -OutFile $zipFile

# Verify checksum
$fileHash = Get-FileHash -Path $zipFile -Algorithm SHA256
if ($fileHash.Hash -ne $checksum) {
    throw "Checksum verification failed. Expected: $checksum, Got: $($fileHash.Hash)"
}

Write-Host "Extracting Guepard CLI..."
Expand-Archive -Path $zipFile -DestinationPath $tempDir -Force

# Install to chocolatey bin directory
$binDir = Join-Path $env:ChocolateyInstall "bin"
if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir | Out-Null
}

$exePath = Join-Path $tempDir "gfs.exe"
if (Test-Path $exePath) {
    Copy-Item $exePath $binDir -Force
    Write-Host "Guepard CLI installed successfully!"
    Write-Host "You can now use 'gfs' command from any terminal."
} else {
    throw "gfs.exe not found in downloaded package"
}

# Cleanup
Remove-Item $tempDir -Recurse -Force
