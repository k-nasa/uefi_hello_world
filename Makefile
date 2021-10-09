.PHONY: build
build:
	cargo build -Z build-std --target x86_64-unknown-uefi
