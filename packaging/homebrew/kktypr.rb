class Kktypr < Formula
  desc "Terminal touch typing academy and developer code syntax trainer in pure Rust"
  homepage "https://github.com/kk376/kktypr"
  license any_of: ["MIT", "Apache-2.0"]
  version "0.1.1"

  on_macos do
    url "https://github.com/kk376/kktypr/archive/refs/tags/v0.1.1.tar.gz"
    sha256 "40ee0e16f102d948868206be1d131c022da1491a1528a192c1c9ee11e72d3e36"
    depends_on "rust" => :build

    def install
      system "cargo", "install", *std_cargo_args
      man1.install "docs/kktypr.1" if File.exist?("docs/kktypr.1")
      doc.install "README.md" if File.exist?("README.md")
    end
  end

  on_linux do
    url "https://github.com/kk376/kktypr/releases/download/v0.1.1/kktypr-0.1.1-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "5279d5ef2d70634e226a3fa21a82e2b16c01b698a50514c9c862153a336d8971"

    def install
      bin.install "kktypr"
      man1.install "kktypr.1" if File.exist?("kktypr.1")
      doc.install "README.md" if File.exist?("README.md")
    end
  end

  test do
    assert_match "kktypr 0.1.1", shell_output("#{bin}/kktypr --version")
  end
end
