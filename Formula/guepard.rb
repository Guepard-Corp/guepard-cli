class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.6/guepard-cli-0.29.6-macos-arm64.tar.gz"
  version "0.29.6"
  sha256 "2768230bd0bb324e23088276437d61c340843e69dd4cb0910f4f5cc830211ac7"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.6/guepard-cli-0.29.6-macos-amd64.tar.gz"
      sha256 "4599bae0740a5c5bbdbe4c627525b5dc9ee6be383aee67d53947fd2cd462a9dc"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.6/guepard-cli-0.29.6-linux-arm64.tar.gz"
      sha256 "155c41a75e6634de1ed6d5b0c5b440ccf35888b77de100da91b29f7941e20229"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.6/guepard-cli-0.29.6-linux-amd64.tar.gz"
      sha256 "b286bd2ccb8c3688a15590c84af5f91315cbcf67ae1e2d67acccc2f1c9b668c9"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
