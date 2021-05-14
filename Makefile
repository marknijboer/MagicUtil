windows:
	cross build --target x86_64-pc-windows-gnu --release
	@rsync target/x86_64-pc-windows-gnu/release/magicutil.exe target/x86_64-pc-windows-gnu/release/MagicUtil.exe

bundle: 
	@python scoop/bundle.py

deploy: windows bundle

# -----------------------------------------------------------------------------

linux:
	cross build --target x86_64-unknown-linux-gnu --release

linux-alpine:
	cross build --target x86_64-unknown-linux-musl --release
