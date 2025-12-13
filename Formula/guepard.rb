class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.0/guepard-cli-0.29.0-macos-arm64.tar.gz"
  version "0.29.0"
  sha256 "73b6c9a384b253066a3f61efc24437f3f082c3da72075d34522dda23f2da520c"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.0/guepard-cli-0.29.0-macos-amd64.tar.gz"
      sha256 "e0d0cc1545cb38684782c25c4a827de244579a23da79fc13dfa926621e320884"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.0/guepard-cli-0.29.0-linux-arm64.tar.gz"
      sha256 "000fb0a8867b0d6d9bcef6d08a70e1c1160b298b6f4eeda43bce8da59551b56a"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.0/guepard-cli-0.29.0-linux-amd64.tar.gz"
      sha256 "af4ad40106f40b1d390512a08f2a0a70c2967f77df32e0621f942fbc42501047"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
