# Copyright 2026 Gentoo Authors
# Distributed under the terms of the MIT License or Apache License 2.0

EAPI=8

CRATES=""

inherit cargo

DESCRIPTION="Terminal touch typing academy and developer code syntax trainer in pure Rust"
HOMEPAGE="https://github.com/kk376/kktypr"
SRC_URI="https://github.com/kk376/${PN}/archive/refs/tags/v${PV}.tar.gz -> ${P}.tar.gz
	$(cargo_crate_uris)"

LICENSE="|| ( MIT Apache-2.0 )"
SLOT="0"
KEYWORDS="~amd64 ~arm64 ~x86"

src_install() {
	cargo_src_install

	dodoc README.md
	dodoc LICENSE-MIT
	dodoc LICENSE-APACHE
}
