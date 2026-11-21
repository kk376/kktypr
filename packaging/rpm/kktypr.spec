%global debug_package %{nil}

Name:           kktypr
Version:        0.1.1
Release:        1%{?dist}
Summary:        Terminal touch typing academy and developer code syntax trainer in Rust

License:        MIT OR Apache-2.0
URL:            https://github.com/kk376/kktypr
Source0:        https://github.com/kk376/kktypr/archive/refs/tags/v%{version}.tar.gz#/%{name}-%{version}.tar.gz

BuildRequires:  cargo >= 1.88.0
BuildRequires:  rust >= 1.88.0
BuildRequires:  gcc

%description
kktypr is a high-performance terminal touch typing tutor and developer syntax
trainer written in pure Rust (Edition 2024) with Ratatui and Crossterm.
It features stationary typewriter cursor physics, weak-key heatmap analysis,
dynamic remedial drill synthesis, real-time typing metrics, and a 60 FPS
Word Drop arcade game.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release --locked

%check
cargo test --release --locked

%install
install -Dpm 0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
if [ -f docs/%{name}.1 ]; then
    install -Dpm 0644 docs/%{name}.1 %{buildroot}%{_mandir}/man1/%{name}.1
fi
install -Dpm 0644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license LICENSE LICENSE-MIT LICENSE-APACHE
%doc README.md
%{_bindir}/%{name}
%{_mandir}/man1/%{name}.1*

%changelog
* Sat Nov 21 2026 Kushagra Kumar (kk376) <kkushagra86@gmail.com> - 0.1.1-1
- Release version 0.1.1: Modernized toolchain, MSRV 1.88.0, and packaging matrix
