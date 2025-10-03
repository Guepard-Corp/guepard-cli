class Guepard < Formula
  desc "Guepard CLI - Git-like filesystem for databases"
  homepage "https://www.guepard.run"
  url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v${VERSION}/guepard-cli-v${VERSION}-macos-arm64.tar.gz"
  version "${VERSION}"
  sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
  license "Guepard (c) 2025"

  on_macos do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v${VERSION}/guepard-cli-v${VERSION}-macos-arm64.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v${VERSION}/guepard-cli-v${VERSION}-macos-amd64.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v${VERSION}/guepard-cli-v${VERSION}-linux-arm64.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
    on_intel do
      url "https://github.com/Guepard-Corp/guepard-cli/releases/download/v${VERSION}/guepard-cli-v${VERSION}-linux-amd64.tar.gz"
      sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
    end
  end

  def install
    bin.install "guepard"
  end

  test do
    system "#{bin}/guepard", "--version"
  end
end
