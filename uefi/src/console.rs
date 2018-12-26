use core::ptr::NonNull;

use uefi_sys::{EFI_SIMPLE_TEXT_INPUT_PROTOCOL, EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL};

use crate::types::*;
use crate::protocol::Protocol;

#[derive(Copy, Clone)]
pub struct SimpleTextInputProtocol {
    inner: NonNull<EFI_SIMPLE_TEXT_INPUT_PROTOCOL>,
}

impl Protocol for SimpleTextInputProtocol {
    type Raw = EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

    fn from_ptr(ptr: NonNull<Self::Raw>) -> Self {
        SimpleTextInputProtocol {
            inner: ptr,
        }
    }
}

impl SimpleTextInputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiResult<()> {
        let ptr = self.inner.as_ptr();
        unsafe { ((*ptr).Reset)(ptr as *const _, extended_verification.into()) }.into()
    }
}

#[derive(Copy, Clone)]
pub struct SimpleTextOutputProtocol {
    inner: NonNull<EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL>,
}

impl Protocol for SimpleTextOutputProtocol {
    type Raw = EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

    fn from_ptr(ptr: NonNull<Self::Raw>) -> Self {
        SimpleTextOutputProtocol {
            inner: ptr,
        }
    }
}

impl SimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiResult<()> {
        let ptr = self.inner.as_ptr();
        unsafe { ((*ptr).Reset)(ptr as *const _, extended_verification.into()) }.into()
    }

    pub unsafe fn output_string_raw(&self, string: &[u16]) -> EfiResult<()> {
        let ptr = self.inner.as_ptr();
        ((*ptr).OutputString)(ptr as *const _, string as *const _ as *const u16).into()
    }
}
