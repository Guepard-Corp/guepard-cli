class Gfs < Formula
  desc "Guepard CLI - Git-like filesystem for databases"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.1/guepard-cli-0.25.1-macos-arm64.tar.gz"
  version "0.25.1"
  sha256 "680dd71aed7c9d522a53316970d102cddfcebf7128ebda228eda1580dc5c1853"
  license "Guepard (c) 2025"

  on_macos do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.1/guepard-cli-0.25.1-macos-arm64.tar.gz"
      sha256 "680dd71aed7c9d522a53316970d102cddfcebf7128ebda228eda1580dc5c1853"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.1/guepard-cli-0.25.1-macos-amd64.tar.gz"
      sha256 "8f63bfc072bb82ce5e1abb220fec176758cc3e68626e30ffaba6881741d41949"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.1/guepard-cli-0.25.1-linux-arm64.tar.gz"
      sha256 "PLACEHOLDER_LINUX_ARM64_SHA256"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.1/guepard-cli-0.25.1-linux-amd64.tar.gz"
      sha256 "PLACEHOLDER_LINUX_AMD64_SHA256"
    end
  end

  def install
    bin.install "gfs"
  end

  test do
    system "#{bin}/gfs", "--version"
  end
end
