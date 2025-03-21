class CryptoCli < Formula
  desc "A command-line tool for crypto operations"
  homepage "https://github.com/yourusername/crypto-cli"
  url "https://github.com/yourusername/crypto-cli/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "YOUR_TARBALL_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    # If you have any additional files like man pages or completions
    # man.install "man/crypto.1"
    # bash_completion.install "completions/crypto.bash"
  end

  test do
    assert_match "Crypto CLI 0.1.0", shell_output("#{bin}/crypto --version")
  end
end 