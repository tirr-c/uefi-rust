use alloc::vec::Vec;

pub fn str_to_utf16(input: &str) -> Vec<u16> {
    input.encode_utf16().chain(core::iter::once(0)).collect()
}
