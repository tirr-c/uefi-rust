#![no_std]
#![cfg_attr(feature = "alloc", feature(alloc, alloc_error_handler, core_intrinsics))]

pub mod types;
mod system_table;
pub mod protocol;
pub mod console;
mod boot_services;

use uefi_sys::{EFI_HANDLE, EFI_SYSTEM_TABLE};

use self::types::*;
pub use self::system_table::SystemTable;
pub use self::boot_services::BootServices;

struct Env {
    image_handle: Handle,
    system_table: SystemTable,
}

static mut ENV: Option<Env> = None;

pub unsafe fn init_env(image_handle: EFI_HANDLE, system_table: *const EFI_SYSTEM_TABLE) {
    if ENV.is_some() {
        panic!("UEFI environment is already initialized");
    }
    ENV = Some(Env {
        image_handle,
        system_table: SystemTable::new(system_table),
    });
}

pub fn image_handle() -> Handle {
    let env = unsafe { ENV.as_ref() }.expect("UEFI environment is not initialized");
    env.image_handle
}

pub fn system_table() -> SystemTable {
    system_table_opt().expect("UEFI environment is not initialized")
}

pub(crate) fn system_table_opt() -> Option<SystemTable> {
    unsafe { ENV.as_ref() }.map(|env| {
        env.system_table
    })
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod allocator;
#[cfg(feature = "alloc")]
mod utf16;

#[cfg(feature = "alloc")]
use self::allocator::UefiPoolAllocator;

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOCATOR: UefiPoolAllocator = UefiPoolAllocator;

#[cfg(feature = "alloc")]
#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    unsafe { core::intrinsics::abort(); }
}
