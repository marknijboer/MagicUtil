windows:
	cross build --target x86_64-pc-windows-gnu --release
	mv target/x86_64-pc-windows-gnu/release/magicutil.exe target/x86_64-pc-windows-gnu/release/MagicUtil.exe

bundle: 
	python scoop/bundle.py

deploy: windows bundle