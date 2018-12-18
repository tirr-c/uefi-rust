#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi_sys::{
    Handle,
    SystemTable,
    Status,
};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}

#[no_mangle]
extern fn efi_main(_image_handle: Handle, system_table: *const SystemTable) -> Status {
    unsafe {
        let conout = (*system_table).conout;
        ((*conout).output_string)(conout, &[0x41u16, 10u16] as *const _);
    }
    Status::Success
}
