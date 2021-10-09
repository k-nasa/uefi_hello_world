use core::ffi::c_void;

#[derive(Debug)]
pub struct Handle(*mut c_void);

#[derive(Debug)]
pub struct UnimplementFunctionPointor(*mut c_void);

#[derive(Debug)]
#[repr(C)]
pub struct SystemTable {
    header: Header,

    fw_vendor: *const Char16,
    fw_revision: u32,

    console_in_handle: Handle,
    con_in: UnimplementFunctionPointor,

    console_out_handle: Handle,
    con_out: UnimplementFunctionPointor,

    std_error_handle: Handle,
    std_err: UnimplementFunctionPointor,

    boot_services: UnimplementFunctionPointor,
    number_of_table_entries: usize,
    configuration_table: UnimplementFunctionPointor,
}

#[derive(Debug)]
#[repr(C)]
struct Header {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

// NOTE ここを参考に定義していくと良さそう
// ref: https://github.com/rust-osdev/uefi-rs/blame/87db82058c1f7dc7fed8fc2679218779b916eda5/src/result/status.rs#L20
#[repr(usize)]
pub enum Status {
    SUCCESS = 0,
    OUT_OF_RESOURCES = ERROR_BIT | 9,
}

#[derive(Debug)]
pub struct Char16(u16);
