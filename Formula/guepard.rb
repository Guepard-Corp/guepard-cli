class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-macos-arm64.tar.gz"
  version "0.27.17"
  sha256 "c32041855705f9d7b616d5e5e54117cd46adbb30b1e37e7360ca3dd1498f29ba"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-macos-amd64.tar.gz"
      sha256 "f28c051eab075dda7f708cef79bca8868f22f7be064a9c629cdbcec94c9712b4"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-linux-arm64.tar.gz"
      sha256 "fb6ecd29a106233a06694a4c41f66fdddfe77fff9be4d0f15b9b60957e784810"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-linux-amd64.tar.gz"
      sha256 "b21d4a014b92c3c0669992e9faae79274d82caa373689e4bb5c76039ac4e560f"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
