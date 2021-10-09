#![no_std]
#![no_main]

#[no_mangle]
fn efi_main() -> usize {
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
