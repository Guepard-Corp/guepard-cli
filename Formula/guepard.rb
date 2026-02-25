class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.8/guepard-cli-0.29.8-macos-arm64.tar.gz"
  version "0.29.8"
  sha256 "150fe29d40968928088f711f82688608bca7b6ed3cb48e5da315c49c88a914d4"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.8/guepard-cli-0.29.8-macos-amd64.tar.gz"
      sha256 "0c3b649708cee69efe73bf23d2ecd59440443b36c9eca211b2226700862b4fc5"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.8/guepard-cli-0.29.8-linux-arm64.tar.gz"
      sha256 "1bb058c4d3c876a1d4d73946b6aceeff9b91c1ec87cb6352042aeee6c1fb2444"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.8/guepard-cli-0.29.8-linux-amd64.tar.gz"
      sha256 "bbe83220acf3ad85f5ca05bd42511c45727f2139d6c52bad5ce265b66f9708dc"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
