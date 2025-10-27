class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.19/guepard-cli-0.27.19-macos-arm64.tar.gz"
  version "0.27.19"
  sha256 "e711af8a5478bc3a2d7493295b88550d084f40671e77cda5805e5cdc3e47927f"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.19/guepard-cli-0.27.19-macos-amd64.tar.gz"
      sha256 "95c1514690d84289f3461d4c61cd6bd6d1206b1b4fd79f51d54dd6e3924a45be"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.19/guepard-cli-0.27.19-linux-arm64.tar.gz"
      sha256 "8bf9575ff133928aff02f56b930c149f343da18b96c3753a97663d432b125870"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.19/guepard-cli-0.27.19-linux-amd64.tar.gz"
      sha256 "f741ce3423831e0832b6c4e3a7eaf54e1b9f4c6fc4ded7c38e619272ebdce587"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
