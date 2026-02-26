class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.10/guepard-cli-0.29.10-macos-arm64.tar.gz"
  version "0.29.10"
  sha256 "3ad79cf60352a5ee07f12164a2d4e3b12a3910ad56faad9b26f29660ff85f9f3"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.10/guepard-cli-0.29.10-macos-amd64.tar.gz"
      sha256 "bf310d9c0cb36334b67bd5c074c21ebf9bd578ed179b9daaed79d821f84881d3"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.10/guepard-cli-0.29.10-linux-arm64.tar.gz"
      sha256 "e5b9a708c4eacb881fa8471407c01d990069bbd60a9e1fee623c26ede2508c96"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.10/guepard-cli-0.29.10-linux-amd64.tar.gz"
      sha256 "b74d9025dfd5259c4157466df04d5b7eb5c22f85b83d1627e58d5436b3ac12f6"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
