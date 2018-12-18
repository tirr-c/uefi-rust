use crate::types::*;
use crate::console::{SimpleTextInputProtocol, SimpleTextOutputProtocol};
use crate::runtime_services::RuntimeServices;
use crate::boot_services::BootServices;
use crate::configuration_table::ConfigurationTable;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SystemHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    pub hdr: SystemHeader,
    pub firmware_vendor: *const Char,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub conin: *const SimpleTextInputProtocol,
    pub console_out_handle: Handle,
    pub conout: *const SimpleTextOutputProtocol,
    pub standard_error_handle: Handle,
    pub stderr: *const SimpleTextOutputProtocol,
    pub runtime_services: *const RuntimeServices,
    pub boot_services: *const BootServices,
    pub number_of_table_entries: usize,
    pub confiruation_table: *const ConfigurationTable,
}
