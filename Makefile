build:
	cargo build --release
	sudo cp ./target/release/pdf-rename /usr/local/bin/