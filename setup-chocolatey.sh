#!/bin/bash

# Chocolatey Package Setup Script for Guepard CLI
# This script helps you publish your CLI to Chocolatey

set -e

echo "🍫 Setting up Chocolatey package for Guepard CLI"
echo "==============================================="

# Check if choco is installed
if ! command -v choco &> /dev/null; then
    echo "❌ Chocolatey is not installed. Please install it first:"
    echo "   # Run in PowerShell as Administrator:"
    echo "   Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"
    exit 1
fi

echo "📋 Chocolatey package configuration created:"
echo "   - chocolatey/gfs.nuspec"
echo "   - chocolatey/tools/chocolateyInstall.ps1"
echo "   - chocolatey/tools/chocolateyUninstall.ps1"
echo ""
echo "📝 Next steps to publish to Chocolatey:"
echo ""
echo "1. Update chocolateyInstall.ps1 with correct SHA256:"
echo "   # Replace PLACEHOLDER_WINDOWS_AMD64_SHA256 with actual checksum"
echo ""
echo "2. Build the package:"
echo "   choco pack chocolatey/gfs.nuspec"
echo ""
echo "3. Test the package locally:"
echo "   choco install gfs --source . --force"
echo ""
echo "4. Publish to Chocolatey:"
echo "   choco push gfs.0.25.1.nupkg --source https://push.chocolatey.org/"
echo ""
echo "🎉 Users can then install with:"
echo "   choco install gfs"
echo ""
echo "⚠️  Note: You need a Chocolatey account and API key to publish."
echo "   Sign up at: https://chocolatey.org/account/Register"
