# Packaging kktypr

This directory contains distribution package definitions, build recipes, and metadata for Linux, macOS, BSD, and Windows package managers.

## Package Matrix

| Distribution / Repository | Recipe Location | Primary Tool | Target Output |
| :--- | :--- | :--- | :--- |
| **Arch Linux / AUR** | [`arch/PKGBUILD`](arch/PKGBUILD) | `makepkg` | `.pkg.tar.zst` |
| **Debian / Ubuntu** | [`debian/`](debian/) | `dpkg-buildpackage` / `cargo-deb` | `.deb` |
| **Fedora / RHEL / Copr** | [`rpm/kktypr.spec`](rpm/kktypr.spec) | `rpmbuild` / `cargo-generate-rpm` | `.rpm` |
| **Nix / NixOS** | [`nix/default.nix`](nix/default.nix), [`flake.nix`](../flake.nix) | `nix build` | Nix store path |
| **Void Linux** | [`void/template`](void/template) | `xbps-src` | `.xbps` |
| **Alpine Linux** | [`alpine/APKBUILD`](alpine/APKBUILD) | `abuild` | `.apk` |
| **Gentoo Linux** | [`gentoo/kktypr-0.2.0.ebuild`](gentoo/) | `ebuild` / `emerge` | Portage ebuild |
| **Homebrew** | [`homebrew/kktypr.rb`](homebrew/kktypr.rb) | `brew` | Formula / Bottled bottle |
| **openSUSE / OBS** | [`opensuse/kktypr.spec`](opensuse/kktypr.spec) | `osc` / `rpmbuild` | `.rpm` |
| **KISS Linux** | [`kiss/`](kiss/) | `kiss` | KISS package |
| **Windows (Scoop)** | [`scoop/kktypr.json`](scoop/kktypr.json) | `scoop` | Scoop manifest |
| **Cargo Binstall** | `Cargo.toml` metadata | `cargo binstall kktypr` | Pre-compiled binary |

---

## Distribution Recipes and Build Workflows

### 1. Arch Linux (AUR)

- **Source files**: [`packaging/arch/PKGBUILD`](arch/PKGBUILD), [`packaging/arch/.SRCINFO`](arch/.SRCINFO), and root [`PKGBUILD`](../PKGBUILD)
- **Install paths**:
  - Binary: `/usr/bin/kktypr`
  - Docs & license: `/usr/share/doc/kktypr/README.md`, `/usr/share/licenses/kktypr/LICENSE*`

#### Local Build
```bash
cd packaging/arch
makepkg -si
```

#### AUR Submission
```bash
git clone ssh://aur@aur.archlinux.org/kktypr.git
cd kktypr
cp /path/to/kktypr/packaging/arch/PKGBUILD .
updpkgsums
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.2.0"
git push origin master
```

---

### 2. Debian & Ubuntu

- **Source files**: [`packaging/debian/`](debian/) (`control`, `rules`, `changelog`, `copyright`, `source/format`)

#### Local Build with cargo-deb
```bash
cargo install cargo-deb
cargo deb
```
Output: `target/debian/kktypr_0.2.0-1_amd64.deb`

#### Local Build with Debian Tooling
```bash
dpkg-buildpackage -us -uc -b
```

---

### 3. Fedora, RHEL & Copr (RPM)

- **Source files**: [`packaging/rpm/kktypr.spec`](rpm/kktypr.spec)

#### Local Build with cargo-generate-rpm
```bash
cargo install cargo-generate-rpm
cargo build --release
cargo generate-rpm
```

#### Local Build with rpmbuild
```bash
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
cp packaging/rpm/kktypr.spec ~/rpmbuild/SPECS/
spectool -g -R ~/rpmbuild/SPECS/kktypr.spec
rpmbuild -ba ~/rpmbuild/SPECS/kktypr.spec
```

---

### 4. Homebrew

- **Source files**: [`packaging/homebrew/kktypr.rb`](homebrew/kktypr.rb)

#### Local Test
```bash
brew install --build-from-source packaging/homebrew/kktypr.rb
brew test kktypr
```

#### Tap Setup
Place in `homebrew-kktypr` tap (`github.com/kk376/homebrew-kktypr/Formula/kktypr.rb`).
Users can install directly via:
```bash
brew tap kk376/kktypr
brew install kktypr
```

---

### 5. Nix / NixOS

- **Source files**: [`flake.nix`](../flake.nix), [`packaging/nix/default.nix`](nix/default.nix), [`packaging/nix/package.nix`](nix/package.nix)

#### Run directly without installing:
```bash
nix run github:kk376/kktypr
```

#### Build with Flakes:
```bash
nix build
```

---

### 6. Alpine Linux

- **Source files**: [`packaging/alpine/APKBUILD`](alpine/APKBUILD)

```bash
cd packaging/alpine
abuild checksum
abuild -r
```

---

### 7. Void Linux

- **Source files**: [`packaging/void/template`](void/template)

```bash
cd void-packages
mkdir -p srcpkgs/kktypr
cp /path/to/kktypr/packaging/void/template srcpkgs/kktypr/
./xbps-src pkg kktypr
```

---

### 8. Windows (Scoop)

- **Source files**: [`packaging/scoop/kktypr.json`](scoop/kktypr.json)

Users install via Scoop bucket:
```powershell
scoop bucket add kk376 https://github.com/kk376/scoop-bucket
scoop install kktypr
```

---

## Universal POSIX Installer

For automated environments, CI/CD runners, and headless containers:
```bash
curl -sSL https://raw.githubusercontent.com/kk376/kktypr/main/install.sh | bash
```
Or locally from source:
```bash
./install.sh
```

---

## Maintainer Release Checklist

1. Tag the release: `git tag -a v0.2.0 -m "Release v0.2.0"` and push tags (`git push origin v0.2.0`).
2. GitHub Actions automated release pipeline builds multi-OS binaries (musl, gnu, windows, Arch `.pkg.tar.zst`) and computes SHA256 checksums.
3. Update version strings and checksums across `PKGBUILD`, `.SRCINFO`, `kktypr.spec`, `kktypr.rb`, and `APKBUILD`.
