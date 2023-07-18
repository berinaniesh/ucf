compile:
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-unknown-linux-musl
install:
	cargo install --path .
format:
	# Temporary solution (until ucf supports directories)
	find . -type f -name \*.rs -exec ucf "{}" \;
release:
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-unknown-linux-musl
