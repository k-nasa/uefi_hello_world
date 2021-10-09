#![no_std]
#![no_main]

mod uefi;

#[no_mangle]
fn efi_main(_image: uefi::Handle) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
