class Guepard < Formula
  desc "Guepard CLI (guepard) - Git, but for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.2/guepard-cli-v0.25.2-macos-arm64.tar.gz"
  version "0.25.2"
  sha256 "568929d4a8ada6e38c9eaa0222eafc946bead757b8fc90a8bb1fd22e111cde69"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.2/guepard-cli-v0.25.2-macos-amd64.tar.gz"
      sha256 "3322235013aca5021b83d52cce49a1b57e6e7780727428372a411f1c406d89f7"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.2/guepard-cli-v0.25.2-linux-arm64.tar.gz"
      sha256 "e44b5c47f070dcde1f088e4b43ef54f31e9e1acbe1c336e4645b7fcc65a9ffdf"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.2/guepard-cli-v0.25.2-linux-amd64.tar.gz"
      sha256 "4815b4ab32ab9ed9824577446fd38ad2e0a6a170c672e46e2242c4ea564bf7aa"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
