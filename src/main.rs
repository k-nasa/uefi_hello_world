#![no_std]
#![no_main]

mod uefi;

#[no_mangle]
fn efi_main(_image: uefi::Handle, _stable: &'static uefi::SystemTable) -> uefi::Status {
    return uefi::Status::SUCCESS;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
