class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.13/guepard-cli-v0.27.13-macos-arm64.tar.gz"
  version "0.27.13"
  sha256 "bb653db14f07b8ef2f667024ac8959400be88a0b12d847477efe9426be11d07a"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.13/guepard-cli-v0.27.13-macos-amd64.tar.gz"
      sha256 "b0d4a3042de1d317cd1454b751d591bf5861978a31270ca1e1ab34bef074b8c4"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.13/guepard-cli-v0.27.13-linux-arm64.tar.gz"
      sha256 "5dd70049b42a1c7d4dd667578c3be4b2a2f182ddec29edbbf7e89dcc076b5783"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.13/guepard-cli-v0.27.13-linux-amd64.tar.gz"
      sha256 "aa24a1f8b4c409c282547dc22e42030937dd4fd65c46962f6d3d76ae9248c133"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
