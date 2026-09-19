#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
cargo build --release
rm -rf "$ROOT/.debroot"
mkdir -p "$ROOT/.debroot/DEBIAN" "$ROOT/.debroot/usr/bin" "$ROOT/.debroot/usr/share/doc/astrafetch"
install -m755 target/release/astrafetch "$ROOT/.debroot/usr/bin/astrafetch"
ln -s astrafetch "$ROOT/.debroot/usr/bin/asfetch"
install -m644 README.md "$ROOT/.debroot/usr/share/doc/astrafetch/README.md"
install -m644 LICENSE "$ROOT/.debroot/usr/share/doc/astrafetch/LICENSE"
install -m644 packaging/debian/control "$ROOT/.debroot/DEBIAN/control"
install -m644 packaging/debian/copyright "$ROOT/.debroot/DEBIAN/copyright"
dpkg-deb --build --root-owner-group "$ROOT/.debroot" "$ROOT/astrafetch_1.0.0_$(dpkg --print-architecture).deb"
rm -rf "$ROOT/.debroot"
