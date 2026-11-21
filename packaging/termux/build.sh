TERMUX_PKG_NAME="kktypr"
TERMUX_PKG_HOMEPAGE="https://github.com/kk376/kktypr"
TERMUX_PKG_DESCRIPTION="Terminal touch typing academy and developer code syntax trainer in pure Rust"
TERMUX_PKG_LICENSE="MIT, Apache-2.0"
TERMUX_PKG_MAINTAINER="Kushagra Kumar (kk376) <kkushagra86@gmail.com>"
TERMUX_PKG_VERSION="0.1.1"
TERMUX_PKG_SRCURL="https://github.com/kk376/kktypr/archive/refs/tags/v${TERMUX_PKG_VERSION}.tar.gz"
TERMUX_PKG_SHA256=SKIP
TERMUX_PKG_AUTO_UPDATE=true
TERMUX_PKG_BUILD_IN_SRC=true

termux_step_make() {
	termux_setup_rust
	cargo build --jobs $TERMUX_PKG_MAKE_PROCESSES --target $CARGO_TARGET_NAME --release
}

termux_step_make_install() {
	install -Dm755 target/${CARGO_TARGET_NAME}/release/kktypr $TERMUX_PREFIX/bin/kktypr
	install -Dm644 README.md $TERMUX_PREFIX/share/doc/kktypr/README.md
}
