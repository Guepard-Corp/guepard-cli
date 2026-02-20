class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.5/guepard-cli-0.29.5-macos-arm64.tar.gz"
  version "0.29.5"
  sha256 "a430bb1adb6df2ef93d4acfb289d878905e84eb715cf890494d3be96ef98b608"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.5/guepard-cli-0.29.5-macos-amd64.tar.gz"
      sha256 "4af6f0403b6cefa3330624670a38f50d2ceada711d0c8ef1a478f6d7d07b1495"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.5/guepard-cli-0.29.5-linux-arm64.tar.gz"
      sha256 "dab055e12513fa4b12aa6638383b48ed9e31446634098eb161da07a7a6478382"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.5/guepard-cli-0.29.5-linux-amd64.tar.gz"
      sha256 "86c16a51d7ab266243baaa6cd84a1eec3cc9af00f8153a65ca207641af6ca83d"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
