Name:           kktypr
Version:        0.2.0
Release:        1
Summary:        Terminal touch typing academy and developer code syntax trainer in Rust
License:        MIT OR Apache-2.0
Group:          Development/Tools/Other
URL:            https://github.com/kk376/kktypr
Source0:        https://github.com/kk376/kktypr/archive/refs/tags/v%{version}.tar.gz#/%{name}-%{version}.tar.gz
Source1:        vendor.tar.zst
ExclusiveArch:  x86_64 aarch64

%if 0%{?suse_version} < 1600
BuildRequires:  cargo >= 1.75.0
BuildRequires:  rust >= 1.75.0
%else
BuildRequires:  cargo >= 1.88.0
BuildRequires:  rust >= 1.88.0
%endif
BuildRequires:  zstd
BuildRequires:  gcc
BuildRequires:  python3-base

%description
KKTypr is a modern terminal-based touch typing tutor and developer code
syntax trainer written in pure Rust with Ratatui and Crossterm.

%prep
%autosetup -n %{name}-%{version}
tar -I zstd -xf %{SOURCE1}
%if 0%{?suse_version} < 1600
sed -i -e 's/edition = "2024"/edition = "2021"/' -e '/rust-version/d' Cargo.toml
sed -i 's/^version = 4$/version = 3/' Cargo.lock || true
find vendor -name Cargo.toml -exec sed -i -e 's/edition = "2024"/edition = "2021"/' -e '/rust-version/d' {} +
python3 -c '
import glob
for path in glob.glob("vendor/**/*.rs", recursive=True):
    with open(path, "r") as f: content = f.read()
    if "pub const fn height(&self)" in content:
        content = content.replace("pub const fn height(&self)", "pub fn height(&self)")
        with open(path, "w") as f: f.write(content)
'
%endif
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

%build
cargo build --release --offline

%check
%if 0%{?suse_version} >= 1600
cargo test --release --offline
%else
./target/release/%{name} --version
%endif

%install
# Install executable binary
install -Dpm 0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install man page
if [ -f docs/%{name}.1 ]; then
    install -Dpm 0644 docs/%{name}.1 %{buildroot}%{_mandir}/man1/%{name}.1
fi

# Install documentation
install -Dpm 0644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license LICENSE LICENSE-MIT LICENSE-APACHE
%doc README.md
%{_bindir}/%{name}
%{_mandir}/man1/%{name}.1*

%changelog
* Sat Nov 21 2026 Kushagra Kumar (kk376) <kkushagra86@gmail.com> - 0.2.0-1
- Release version 0.2.0 for openSUSE with Leap 15.6 and Tumbleweed support
