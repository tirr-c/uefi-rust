use crate::types::*;
use crate::define_interface;

pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID(0x387477c1, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID(0x387477c2, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

define_interface! {
    #[repr(C)]
    pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
        pub fn Reset(*const self, BOOL) -> EFI_STATUS,
        pub fn ReadKeyStroke(*const self, *mut EFI_INPUT_KEY) -> EFI_STATUS,
        pub WaitForKey: EFI_EVENT,
    }
}

define_interface! {
    #[repr(C)]
    pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
        pub fn Reset(*const self, BOOL) -> EFI_STATUS,
        pub fn OutputString(*const self, *const CHAR16) -> EFI_STATUS,
        pub fn TestString(*const self, *const CHAR16) -> EFI_STATUS,
        pub fn QueryMode(*const self, usize, *mut usize, *mut usize) -> EFI_STATUS,
        pub fn SetMode(*const self, usize) -> EFI_STATUS,
        pub fn ClearScreen(*const self) -> EFI_STATUS,
        pub fn SetCursorPosition(*const self, usize, usize) -> EFI_STATUS,
        pub fn EnableCursor(*const self, BOOL) -> EFI_STATUS,
        pub Mode: *const SIMPLE_TEXT_OUTPUT_MODE,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
    pub MaxMode: i32,
    pub Mode: i32,
    pub Attribute: i32,
    pub CursorColumn: i32,
    pub CursorRow: i32,
    pub CursorVisible: BOOL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EFI_INPUT_KEY {
    pub ScanCode: u16,
    pub UnicodeChar: CHAR16,
}
