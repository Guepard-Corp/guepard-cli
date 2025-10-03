class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.26.19/guepard-cli-v0.26.19-macos-arm64.tar.gz"
  version "0.26.19"
  sha256 "0fad9015e7c702a27431a630bbe957664ad484bfad7d82321e8ba9d69c8defb2"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.26.19/guepard-cli-v0.26.19-macos-amd64.tar.gz"
      sha256 "e51583ea6966e4a4c7f4af92a798b547186dfcd1b397a8dde6ae0166d3d6cba7"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.26.19/guepard-cli-v0.26.19-linux-arm64.tar.gz"
      sha256 "3589a31cee62813cb76b530884dde3ab99570de12d1fa255a498cabbaf945fe3"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.26.19/guepard-cli-v0.26.19-linux-amd64.tar.gz"
      sha256 "bfee54d8f6ac956618c26373fd6c547b8e3d64cbe87e0185aedeb99d58749ce9"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
