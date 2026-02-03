class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.4/guepard-cli-0.29.4-macos-arm64.tar.gz"
  version "0.29.4"
  sha256 "41cfb6cd9257653f76e41c766ae751511b62d733f424e4df9a745ee6cc62d868"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.4/guepard-cli-0.29.4-macos-amd64.tar.gz"
      sha256 "00271b9c286d22b45406112bf144e943509175ac307cd81c97ee62161e71b4d7"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.4/guepard-cli-0.29.4-linux-arm64.tar.gz"
      sha256 "a4d331c6a261825e1b1d05cdd69bfa940b005eb6e3f5bd585708ca9916cb3065"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.4/guepard-cli-0.29.4-linux-amd64.tar.gz"
      sha256 "3e2c99d1d9c3092bad666760fd52c715d4548a8ac3fb7f756ae5dd47f879d146"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
