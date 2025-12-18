class Guepard < Formula
  desc "Guepard CLI - Git for Data"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.2/guepard-cli-0.29.2-macos-arm64.tar.gz"
  version "0.29.2"
  sha256 "5489911c8cfbdf44c89e6ed5c9325018ae32784a33902a5a296095a4f7ef3cb5"
  license "Guepard (c) 2025"

  on_macos do
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.2/guepard-cli-0.29.2-macos-amd64.tar.gz"
      sha256 "972ecf4cfa8947ede4cc23f7c63b9ac13e6a65acdfa916d93a6284659ba08126"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.2/guepard-cli-0.29.2-linux-arm64.tar.gz"
      sha256 "aa57ccab745ce53d893004458667c9a70cc596a448edcdb7a2dfb54fbe436a9b"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.29.2/guepard-cli-0.29.2-linux-amd64.tar.gz"
      sha256 "afe63e14825335068e9868ff98f2c46dbb0e36b228ae9d82624c89a2eef357ab"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
