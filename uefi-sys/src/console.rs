use crate::types::*;

pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID(0x387477c1, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID(0x387477c2, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    pub Reset: unsafe extern "win64" fn(*const EFI_SIMPLE_TEXT_INPUT_PROTOCOL, BOOL) -> EFI_STATUS,
    pub ReadKeyStroke: unsafe extern "win64" fn(*const EFI_SIMPLE_TEXT_INPUT_PROTOCOL, *mut EFI_INPUT_KEY) -> EFI_STATUS,
    pub WaitForKey: EFI_EVENT,
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: unsafe extern "win64" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, BOOL) -> EFI_STATUS,
    pub OutputString: unsafe extern "win64" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const CHAR16) -> EFI_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EFI_INPUT_KEY {
    pub ScanCode: u16,
    pub UnicodeChar: CHAR16,
}
