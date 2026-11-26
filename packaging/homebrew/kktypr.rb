class Kktypr < Formula
  desc "Terminal touch typing academy and developer code syntax trainer in pure Rust"
  homepage "https://github.com/kk376/kktypr"
  license any_of: ["MIT", "Apache-2.0"]
  version "0.2.0"

  on_macos do
    url "https://github.com/kk376/kktypr/archive/refs/tags/v0.2.0.tar.gz"
    sha256 "7a13817bab89b2ea68ee9a398b61045a1d7f19dffeccf43c98a44b98546d745a"
    depends_on "rust" => :build

    def install
      system "cargo", "install", *std_cargo_args
      man1.install "docs/kktypr.1" if File.exist?("docs/kktypr.1")
      doc.install "README.md" if File.exist?("README.md")
    end
  end

  on_linux do
    url "https://github.com/kk376/kktypr/releases/download/v0.2.0/kktypr-0.2.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "425916f0198a0ec697b3bf7b32163608f6472045083612a05db06e265c26dab8"

    def install
      bin.install "kktypr"
      man1.install "kktypr.1" if File.exist?("kktypr.1")
      doc.install "README.md" if File.exist?("README.md")
    end
  end

  test do
    assert_match "kktypr 0.2.0", shell_output("#{bin}/kktypr --version")
  end
end
