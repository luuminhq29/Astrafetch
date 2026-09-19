PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

.PHONY: build release test fmt clippy install deb clean

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

install: release
	install -Dm755 target/release/astrafetch $(DESTDIR)$(BINDIR)/astrafetch
	ln -sfn astrafetch $(DESTDIR)$(BINDIR)/asfetch

deb:
	./packaging/build-deb.sh

clean:
	cargo clean
