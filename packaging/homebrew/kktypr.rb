class Kktypr < Formula
  desc "Terminal touch typing academy and developer code syntax trainer in pure Rust"
  homepage "https://github.com/kk376/kktypr"
  license any_of: ["MIT", "Apache-2.0"]
  version "0.3.0"

  on_macos do
    url "https://github.com/kk376/kktypr/archive/refs/tags/v0.3.0.tar.gz"
    sha256 "34ca5fc0e33d069368a0fd0a13cac242b31c2aa7c075859274e87735aef39d56"
    depends_on "rust" => :build

    def install
      system "cargo", "install", *std_cargo_args
      man1.install "docs/kktypr.1" if File.exist?("docs/kktypr.1")
      doc.install "README.md" if File.exist?("README.md")
    end
  end

  on_linux do
    url "https://github.com/kk376/kktypr/releases/download/v0.3.0/kktypr-0.3.0-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "b082bdf9fc7c7543f49a7fd350cc715cf741f4ea6974e5d100c25b3c788b8097"

    def install
      bin.install "kktypr"
      man1.install "kktypr.1" if File.exist?("kktypr.1")
      doc.install "README.md" if File.exist?("README.md")
    end
  end

  test do
    assert_match "kktypr 0.3.0", shell_output("#{bin}/kktypr --version")
  end
end
