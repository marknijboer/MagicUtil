# Search for the first line in Cargo.toml that contains 'version' and return its
# value, stripping the quotes.
VERSION := $(shell awk '/version/{gsub(/"/, "", $$3); print $$3; exit}' ./Cargo.toml)

native:
	cargo build --release --bins

install:
	cp target/release/magicutil /usr/local/bin

windows:
	cross build --target x86_64-pc-windows-gnu --release

release:
	@rsync target/x86_64-pc-windows-gnu/release/magicutil.exe ./dist/MagicUtil.exe
	@gh release create v$(VERSION) --title "MagicUtil $(VERSION)" ./dist/MagicUtil.exe

deploy: windows release

generate-bindings:
	cbindgen --config cbindgen.toml --output include/magicutil.h

pull-cross-containers:
	docker pull --platform=linux/amd64 ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5
	docker pull --platform=linux/amd64 ghcr.io/cross-rs/x86_64-unknown-linux-gnu:0.2.5

build-libs:
	cross build --target aarch64-unknown-linux-gnu --release --lib
	cross build --target x86_64-unknown-linux-gnu --release --lib
