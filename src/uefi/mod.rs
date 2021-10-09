use core::ffi::c_void;

pub struct Handle(*mut c_void);

#[repr(C)]
struct SystemTable {}

#[repr(C)]
struct Header {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}
