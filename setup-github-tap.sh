#!/bin/bash

# Automated GitHub tap setup
GITHUB_USERNAME="Guepard-Corp"
TAP_REPO_NAME="homebrew-guepard-cli"

echo "🔧 Setting up GitHub repository..."

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is not installed. Please install it first:"
    echo "   brew install gh"
    echo "   gh auth login"
    exit 1
fi

# Create the repository
echo "📦 Creating GitHub repository: ${TAP_REPO_NAME}"
gh repo create ${GITHUB_USERNAME}/${TAP_REPO_NAME} \
    --public \
    --description "Homebrew tap for Guepard CLI" \
    --clone

# Copy files
echo "📁 Copying tap files..."
cp -r homebrew-tap/* ${TAP_REPO_NAME}/

# Commit and push
cd ${TAP_REPO_NAME}
git add .
git commit -m "Initial Homebrew tap for Guepard CLI"
git push origin main

echo "✅ Homebrew tap repository created successfully!"
echo ""
echo "🎉 You can now install gfs via Homebrew:"
echo "   brew tap ${GITHUB_USERNAME}/guepard-cli"
echo "   brew install gfs"
