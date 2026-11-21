#!/usr/bin/env bash
# kktypr Universal POSIX Installer
set -euo pipefail

PREFIX="${PREFIX:-/usr/local}"
BIN_DIR="${PREFIX}/bin"
SHARE_DIR="${PREFIX}/share"
MANDIR="${SHARE_DIR}/man/man1"
DOCDIR="${SHARE_DIR}/doc/kktypr"
LICDIR="${SHARE_DIR}/licenses/kktypr"

echo "========================================================"
echo "               kktypr Universal Installer              "
echo "========================================================"

# Determine source binary
SOURCE_BIN=""
if [ -f "target/release/kktypr" ]; then
    SOURCE_BIN="target/release/kktypr"
elif [ -f "kktypr" ]; then
    SOURCE_BIN="kktypr"
else
    echo "Info: Pre-built kktypr binary not found in current directory."
    if command -v cargo >/dev/null 2>&1; then
        echo "Building kktypr with cargo build --release..."
        cargo build --release
        SOURCE_BIN="target/release/kktypr"
    else
        echo "Error: Neither pre-built binary nor cargo build toolchain found." >&2
        exit 1
    fi
fi

# Automatically remove conflicting/shadowed binaries in user environment
SUDO_CALLER="${SUDO_USER:-$USER}"
if [ -n "$SUDO_CALLER" ]; then
    CALLER_HOME=$(getent passwd "$SUDO_CALLER" 2>/dev/null | cut -d: -f6 || echo "${HOME:-}")
    if [ -n "$CALLER_HOME" ]; then
        for conflicting in "$CALLER_HOME/.cargo/bin/kktypr" "$CALLER_HOME/.local/bin/kktypr"; do
            if [ -f "$conflicting" ] && [ "$conflicting" != "${BIN_DIR}/kktypr" ]; then
                rm -f "$conflicting" 2>/dev/null || true
            fi
        done
    fi
fi

# Create target directories
echo "Installing to ${PREFIX}..."
mkdir -p "${BIN_DIR}" "${MANDIR}" "${DOCDIR}" "${LICDIR}"

# Install binary
install -m 0755 "${SOURCE_BIN}" "${BIN_DIR}/kktypr"
echo "  Installed ${BIN_DIR}/kktypr"

# Install man page
if [ -f "docs/kktypr.1" ]; then
    install -m 0644 "docs/kktypr.1" "${MANDIR}/kktypr.1"
    echo "  Installed man page to ${MANDIR}/kktypr.1"
fi

# Install documentation and licenses
if [ -f "README.md" ]; then
    install -m 0644 "README.md" "${DOCDIR}/README.md"
fi
if [ -f "LICENSE" ]; then
    install -m 0644 "LICENSE" "${LICDIR}/LICENSE"
fi
if [ -f "LICENSE-MIT" ]; then
    install -m 0644 "LICENSE-MIT" "${LICDIR}/LICENSE-MIT"
fi
if [ -f "LICENSE-APACHE" ]; then
    install -m 0644 "LICENSE-APACHE" "${LICDIR}/LICENSE-APACHE"
fi

echo "========================================================"
echo "kktypr successfully installed to ${BIN_DIR}/kktypr"
"${BIN_DIR}/kktypr" --version || true
echo "========================================================"
