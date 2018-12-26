#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi_sys::{
    EFI_HANDLE,
    EFI_STATUS,
    EFI_SYSTEM_TABLE,
};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}

#[no_mangle]
extern fn efi_main(image_handle: EFI_HANDLE, system_table: *const EFI_SYSTEM_TABLE) -> EFI_STATUS {
    unsafe { uefi::init_env(image_handle, system_table); }
    let output_status = unsafe {
        uefi::system_table().conout().output_string_raw(&[0x41u16, 0x10, 0x0])
    };
    if let Err(status) = output_status {
        return status;
    }
    EFI_STATUS::EFI_SUCCESS
}
