class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.30.0/guepard-cli-0.30.0-macos-arm64.tar.gz"
  version "0.30.0"
  sha256 "2aeec51f379719e8c1a38dc53e95bebe2d532564a3e7d3c6389119c13991cd61"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.30.0/guepard-cli-0.30.0-macos-amd64.tar.gz"
      sha256 "e61010ab14df6779bbd45186eb0cadaf95ad9decc6a8738426a7d7c063666e10"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.30.0/guepard-cli-0.30.0-linux-arm64.tar.gz"
      sha256 "2ca35ee5d379aaa41d5bef1466a32186d7cf06959d310d482fc1dfbea02447d7"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.30.0/guepard-cli-0.30.0-linux-amd64.tar.gz"
      sha256 "a38ee245f8f0fa64f62d8ecf9ff620594f78984eaec7c95a6fa4c9bd619447b9"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
