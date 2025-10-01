#!/bin/bash

# Homebrew Tap Setup Script for Guepard CLI
# This script helps you set up a GitHub repository for your Homebrew tap

set -e

TAP_NAME="guepard-cli"
GITHUB_USERNAME="Guepard-Corp"  # Change this to your GitHub username
TAP_REPO_NAME="homebrew-guepard-cli"

echo "🚀 Setting up Homebrew tap for Guepard CLI"
echo "=========================================="

# Check if we're in the right directory
if [ ! -f "homebrew-tap/Formula/gfs.rb" ]; then
    echo "❌ Error: homebrew-tap/Formula/gfs.rb not found. Please run this script from the project root."
    exit 1
fi

echo "📁 Creating GitHub repository setup instructions..."
echo ""
echo "To set up your Homebrew tap, follow these steps:"
echo ""
echo "1. Create a new GitHub repository:"
echo "   Repository name: ${TAP_REPO_NAME}"
echo "   Description: Homebrew tap for Guepard CLI"
echo "   Visibility: Public"
echo "   Initialize with README: No"
echo ""
echo "2. Clone the repository:"
echo "   git clone https://github.com/${GITHUB_USERNAME}/${TAP_REPO_NAME}.git"
echo ""
echo "3. Copy the tap files:"
echo "   cp -r homebrew-tap/* ${TAP_REPO_NAME}/"
echo ""
echo "4. Commit and push:"
echo "   cd ${TAP_REPO_NAME}"
echo "   git add ."
echo "   git commit -m 'Initial Homebrew tap for Guepard CLI'"
echo "   git push origin main"
echo ""
echo "5. Test the installation:"
echo "   brew tap ${GITHUB_USERNAME}/${TAP_NAME}"
echo "   brew install gfs"
echo ""
echo "6. Update your main project's README.md to include Homebrew installation instructions:"
echo ""
echo "   ### Homebrew (macOS)"
echo "   \`\`\`bash"
echo "   brew tap ${GITHUB_USERNAME}/${TAP_NAME}"
echo "   brew install gfs"
echo "   \`\`\`"
echo ""

# Create a script to automate the GitHub setup
cat > setup-github-tap.sh << 'EOF'
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
EOF

chmod +x setup-github-tap.sh

echo "📝 Created automated setup script: setup-github-tap.sh"
echo "   Run: ./setup-github-tap.sh"
echo ""
echo "⚠️  Note: Make sure you have GitHub CLI installed and authenticated:"
echo "   brew install gh"
echo "   gh auth login"
