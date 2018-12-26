#![no_std]

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
    let env = unsafe { ENV.as_ref() }.expect("UEFI environment is not initialized");
    env.system_table
}
