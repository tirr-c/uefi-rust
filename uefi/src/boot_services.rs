use core::ptr::NonNull;

use uefi_sys::{EFI_BOOT_SERVICES, EFI_MEMORY_TYPE};

use crate::types::*;
use crate::protocol::{Protocol, RawProtocol};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BootServices {
    inner: NonNull<EFI_BOOT_SERVICES>,
}

impl BootServices {
    pub unsafe fn new(boot_services: *const EFI_BOOT_SERVICES) -> Self {
        let inner = NonNull::new(boot_services as *mut _).expect("BootServices is null");
        BootServices {
            inner,
        }
    }

    pub(crate) fn allocate_pool(&self, size: usize) -> EfiResult<*mut u8> {
        let boot_ptr = self.inner.as_ptr();
        let mut ptr: *const Void = core::ptr::null_mut();
        let alloc_status = unsafe {
            ((*boot_ptr).AllocatePool)(
                EFI_MEMORY_TYPE::EfiLoaderData,
                size,
                &mut ptr,
            )
        };
        <Status as Into<EfiResult<_>>>::into(alloc_status)?;
        Ok(ptr as *const u8 as *mut _)
    }

    pub(crate) fn free_pool(&self, ptr: *mut u8) -> EfiResult<()> {
        let boot_ptr = self.inner.as_ptr();
        unsafe {
            ((*boot_ptr).FreePool)(ptr as *const _ as *const Void)
        }.into()
    }

    pub(crate) fn open_protocol<T: Protocol>(
        &self,
        handle: Handle,
        agent_handle: Handle,
        controller_handle: Option<Handle>,
        attributes: OpenProtocolAttributes,
    ) -> EfiResult<T> {
        let ptr = self.inner.as_ptr();
        let mut raw_protocol = core::ptr::null();
        let open_status: Status = unsafe {
            ((*ptr).OpenProtocol)(
                handle,
                &<T::Raw as RawProtocol>::GUID,
                &mut raw_protocol,
                agent_handle,
                controller_handle,
                attributes.bits(),
            )
        };
        <Status as Into<EfiResult<_>>>::into(open_status)?;
        let raw_protocol = NonNull::new(raw_protocol as *mut _)
            .expect("TEST_PROTOCOL is invalid for open_protocol, use test_protocol instead");
        Ok(T::from_ptr(raw_protocol))
    }

    pub fn test_protocol<T: Protocol>(
        &self,
        handle: Handle,
        agent_handle: Handle,
        controller_handle: Option<Handle>,
    ) -> EfiResult<()> {
        let ptr = self.inner.as_ptr();
        unsafe {
            ((*ptr).OpenProtocol)(
                handle,
                &<T::Raw as RawProtocol>::GUID,
                core::ptr::null_mut(),
                agent_handle,
                controller_handle,
                0x04, // TEST_PROTOCOL
            )
        }.into()
    }

    pub(crate) fn close_protocol(
        &self,
        handle: Handle,
        guid: &Guid,
        agent_handle: Handle,
        controller_handle: Option<Handle>,
    ) -> EfiResult<()> {
        let ptr = self.inner.as_ptr();
        unsafe {
            ((*ptr).CloseProtocol)(
                handle,
                guid as *const _,
                agent_handle,
                controller_handle,
            )
        }.into()
    }
}

bitflags::bitflags! {
    pub struct OpenProtocolAttributes: u32 {
        const GET_PROTOCOL = 0x02;
        const BY_CHILD_CONTROLLER = 0x08;
        const BY_DRIVER = 0x10;
        const EXCLUSIVE = 0x20;
    }
}
