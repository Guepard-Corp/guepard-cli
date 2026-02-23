class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.7/guepard-cli-0.29.7-macos-arm64.tar.gz"
  version "0.29.7"
  sha256 "e84247f81bf1184ec5b8c5155e1d643cfe5d3fa3943340afeebf99b7187497ca"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.7/guepard-cli-0.29.7-macos-amd64.tar.gz"
      sha256 "94666c436ec9fe16c81e3c91d5b482cd54696fcf6d578da2d3a43cbea365fce0"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.7/guepard-cli-0.29.7-linux-arm64.tar.gz"
      sha256 "1739e1a2b018019a1e504892369bdd619c7ccfc7bd7056c575af002e0440b83a"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.7/guepard-cli-0.29.7-linux-amd64.tar.gz"
      sha256 "d8ab45a005258fd5dba175876493344aff3f7df251e7ebac4d018f152904f9ee"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
