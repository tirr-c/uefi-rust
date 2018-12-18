use crate::types::*;

pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID: Guid = Guid(0x387477c1, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: Guid = Guid(0x387477c2, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub reset: unsafe extern "win64" fn(*const SimpleTextInputProtocol, Bool) -> Status,
    pub read_key_stroke: unsafe extern "win64" fn(*const SimpleTextInputProtocol, *mut InputKey) -> Status,
    pub wait_for_key: Event,
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "win64" fn(*const SimpleTextOutputProtocol, Bool) -> Status,
    pub output_string: unsafe extern "win64" fn(*const SimpleTextOutputProtocol, *const Char) -> Status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: Char,
}
