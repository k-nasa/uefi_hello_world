.PHONY: build
build:
	cargo build --target x86_64-unknown-uefi

.PHONY: mnt
mnt:
	mkdir -p mnt/EFI/BOOT
	cp ./target/x86_64-unknown-uefi/debug/blue_magic.efi mnt/EFI/BOOT/BOOTx64.EFI

.PHONY: run_qemu
run_qemu: build mnt
	qemu-system-x86_64 -bios RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:mnt
