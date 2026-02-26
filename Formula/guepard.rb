class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.9/guepard-cli-0.29.9-macos-arm64.tar.gz"
  version "0.29.9"
  sha256 "26e08f0b04770d1bbd918549ba9d80dcd23fc3443b61f6d98677b51b1af109aa"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.9/guepard-cli-0.29.9-macos-amd64.tar.gz"
      sha256 "862afc85e6809b40e6ca273adeef473a0a3a4c027f3e9efa0f345e985aac513e"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.9/guepard-cli-0.29.9-linux-arm64.tar.gz"
      sha256 "ac0ce5847ade8f4ecc14bd4b1accd0f258c7f3c8cc0973a5121613037d666dda"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.9/guepard-cli-0.29.9-linux-amd64.tar.gz"
      sha256 "9eae28e7c04d8839132f738c21ee68d01b6baa6c25f91e531713d12348613d99"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
