use core::ptr::NonNull;

use crate::types::*;
use crate::boot_services::{BootServices, OpenProtocolAttributes};

pub trait RawProtocol {
    const GUID: Guid;
}

pub trait Protocol: Sized {
    type Raw: RawProtocol;

    fn from_ptr(ptr: NonNull<Self::Raw>) -> Self;

    fn open(
        boot_services: &BootServices,
        handle: Handle,
        agent_handle: Handle,
        controller_handle: Option<Handle>,
        attributes: OpenProtocolAttributes,
    ) -> EfiResult<OpenProtocol<Self>> {
        boot_services
            .open_protocol(
                handle,
                agent_handle,
                controller_handle,
                attributes,
            )
            .map(|protocol| OpenProtocol {
                boot_services,
                handle,
                agent_handle,
                controller_handle,
                protocol,
            })
    }
}

pub struct OpenProtocol<'a, T: Protocol> {
    boot_services: &'a BootServices,
    handle: Handle,
    agent_handle: Handle,
    controller_handle: Option<Handle>,
    protocol: T,
}

impl<'a, T: Protocol> Drop for OpenProtocol<'a, T> {
    fn drop(&mut self) {
        self.boot_services.close_protocol(
            self.handle,
            &<T::Raw as RawProtocol>::GUID,
            self.agent_handle,
            self.controller_handle,
        ).ok();
    }
}

impl<'a, T: Protocol> core::ops::Deref for OpenProtocol<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.protocol
    }
}

impl<'a, T: Protocol> core::ops::DerefMut for OpenProtocol<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.protocol
    }
}

impl RawProtocol for uefi_sys::EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    const GUID: Guid = uefi_sys::EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID;
}

impl RawProtocol for uefi_sys::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    const GUID: Guid = uefi_sys::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID;
}
