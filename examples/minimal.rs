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
extern fn efi_main(_image_handle: EFI_HANDLE, system_table: *const EFI_SYSTEM_TABLE) -> EFI_STATUS {
    unsafe {
        let conout = (*system_table).ConOut;
        ((*conout).OutputString)(conout, &[0x41u16, 10u16] as *const _);
    }
    EFI_STATUS::EFI_SUCCESS
}
