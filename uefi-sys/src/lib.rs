#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod types;
mod system_table;
mod console;
mod runtime_services;
mod boot_services;
mod configuration_table;
#[macro_use] mod macros;

pub use self::types::*;
pub use self::system_table::*;
pub use self::console::*;
pub use self::runtime_services::*;
pub use self::boot_services::*;
pub use self::configuration_table::*;
