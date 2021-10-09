#![no_std]
#![no_main]

mod uefi;

#[no_mangle]
fn efi_main(_image: uefi::Handle, stable: &'static uefi::SystemTable) -> uefi::Status {
    let stdout = stable.stdout();
    stdout.reset(true);

    stdout.output_string("Hey!!");

    loop {}
    // return Status::SUCCESS;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
