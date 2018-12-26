use core::ptr::NonNull;

use uefi_sys::EFI_SYSTEM_TABLE;

use crate::types::*;
use crate::protocol::Protocol;
use crate::console::{SimpleTextInputProtocol, SimpleTextOutputProtocol};
use crate::boot_services::BootServices;

#[derive(Copy, Clone)]
pub struct SystemTable {
    console_in_handle: Handle,
    console_out_handle: Handle,
    standard_error_handle: Handle,
    conin: SimpleTextInputProtocol,
    conout: SimpleTextOutputProtocol,
    stderr: SimpleTextOutputProtocol,
    boot_services: BootServices,
}

impl SystemTable {
    pub unsafe fn new(system_table: *const EFI_SYSTEM_TABLE) -> Self {
        let system_table = &*system_table;
        SystemTable {
            console_in_handle: system_table.ConsoleInHandle,
            console_out_handle: system_table.ConsoleOutHandle,
            standard_error_handle: system_table.StandardErrorHandle,
            conin: Protocol::from_ptr(NonNull::new_unchecked(system_table.ConIn as *mut _)),
            conout: Protocol::from_ptr(NonNull::new_unchecked(system_table.ConOut as *mut _)),
            stderr: Protocol::from_ptr(NonNull::new_unchecked(system_table.StdErr as *mut _)),
            boot_services: BootServices::new(system_table.BootServices),
        }
    }

    pub fn conin(&self) -> SimpleTextInputProtocol {
        self.conin
    }

    pub fn conout(&self) -> SimpleTextOutputProtocol {
        self.conout
    }

    pub fn stderr(&self) -> SimpleTextOutputProtocol {
        self.stderr
    }

    pub fn boot_services(&self) -> BootServices {
        self.boot_services
    }
}
