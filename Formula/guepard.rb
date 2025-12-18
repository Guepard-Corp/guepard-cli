class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.3/guepard-cli-0.29.3-macos-arm64.tar.gz"
  version "0.29.3"
  sha256 "5edf2b24ec04513334bc6e63a5da0e884518a585936232aaead4f1c8f4e7019f"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.3/guepard-cli-0.29.3-macos-amd64.tar.gz"
      sha256 "bd1b12c1e176ba538a72d2bc91301954c269d71ebe8cc3fb4a5ca7a9433ec6e7"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.3/guepard-cli-0.29.3-linux-arm64.tar.gz"
      sha256 "8919b6f12538b2806eea033ac35d0335df21b0ac5c83ec8e10923da2b08e1c59"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.3/guepard-cli-0.29.3-linux-amd64.tar.gz"
      sha256 "2c2cd0076620466ecd5eebde8514cca583b9e474d3450c7a5c1534b819793c78"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
