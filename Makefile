VERSION := $(shell grep version ./Cargo.toml | awk '{print substr($$3, 2, length($$3)-2); exit}')

windows:
	cross build --target x86_64-pc-windows-gnu --release
	@rsync target/x86_64-pc-windows-gnu/release/magicutil.exe target/x86_64-pc-windows-gnu/release/MagicUtil.exe

release: 
	@rsync target/x86_64-pc-windows-gnu/release/MagicUtil.exe ./dist/MagicUtil.exe 
	@gh release create v$(VERSION) --title "MagicUtil $(VERSION)" ./dist/MagicUtil.exe

deploy: windows release