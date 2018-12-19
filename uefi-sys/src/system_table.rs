use crate::types::*;
use crate::console::{EFI_SIMPLE_TEXT_INPUT_PROTOCOL, EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL};
use crate::runtime_services::EFI_RUNTIME_SERVICES;
use crate::boot_services::EFI_BOOT_SERVICES;
use crate::configuration_table::EFI_CONFIGURATION_TABLE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EFI_SYSTEM_HEADER {
    pub Signature: u64,
    pub Revision: u32,
    pub HeaderSize: u32,
    pub Crc32: u32,
    pub Reserved: u32,
}

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    pub Hdr: EFI_SYSTEM_HEADER,
    pub FirmwareVendor: *const CHAR16,
    pub FirmwareRevision: u32,
    pub ConsoleInHandle: EFI_HANDLE,
    pub ConIn: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    pub ConsoleOutHandle: EFI_HANDLE,
    pub ConOut: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub StandardErrorHandle: EFI_HANDLE,
    pub StdErr: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub RuntimeServices: *const EFI_RUNTIME_SERVICES,
    pub BootServices: *const EFI_BOOT_SERVICES,
    pub NumberOfTableEntries: usize,
    pub ConfiruationTable: *const EFI_CONFIGURATION_TABLE,
}
