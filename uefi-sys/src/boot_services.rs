use crate::types::*;
use crate::define_interface;

define_interface! {
    #[repr(C)]
    pub struct EFI_BOOT_SERVICES {
        pub Hdr: EFI_TABLE_HEADER,
        pub fn RaiseTPL(EFI_TPL) -> EFI_TPL,
        pub fn RestoreTPL(EFI_TPL),
        pub fn AllocatePages(EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE, usize, *mut EFI_PHYSICAL_ADDRESS) -> EFI_STATUS,
        pub fn FreePages(EFI_PHYSICAL_ADDRESS, usize) -> EFI_STATUS,
        pub fn GetMemoryMap(*mut usize, *mut EFI_MEMORY_DESCRIPTOR, *mut usize, *mut usize, *mut u32) -> EFI_STATUS,
        pub fn AllocatePool(EFI_MEMORY_TYPE, usize, *mut *const VOID) -> EFI_STATUS,
        pub fn FreePool(*const VOID) -> EFI_STATUS,
        pub fn CreateEvent(u32, EFI_TPL, Option<EFI_EVENT_NOTIFY>, *const VOID, *mut EFI_EVENT) -> EFI_STATUS,
        pub fn SetTimer(EFI_EVENT, EFI_TIMER_DELAY, u64) -> EFI_STATUS,
        pub fn WaitForEvent(usize, *const EFI_EVENT, *mut usize) -> EFI_STATUS,
        pub fn SignalEvent(EFI_EVENT) -> EFI_STATUS,
        pub fn CloseEvent(EFI_EVENT) -> EFI_STATUS,
        pub fn CheckEvent(EFI_EVENT) -> EFI_STATUS,
        pub fn InstallProtocolInterface(*mut Option<EFI_HANDLE>, *const EFI_GUID, EFI_INTERFACE_TYPE, *const VOID) -> EFI_STATUS,
        pub fn ReinstallProtocolInterface(EFI_HANDLE, *const EFI_GUID, *const VOID, *const VOID) -> EFI_STATUS,
        pub fn UninstallProtocolInterface(EFI_HANDLE, *const EFI_GUID, *const VOID) -> EFI_STATUS,
        pub fn HandleProtocol(),
        pub Reserved: *const VOID,
        pub fn RegisterProtocolNotify(*const EFI_GUID, EFI_EVENT, *mut *const VOID) -> EFI_STATUS,
        pub fn LocateHandle(),
        pub fn LocateDevicePath(),
        pub fn InstallConfigurationTable(),
        pub fn LoadImage(),
        pub fn StartImage(),
        pub fn Exit(),
        pub fn UnloadImage(),
        pub fn ExitBootServices(),
        pub fn GetNextMonotonicCount(),
        pub fn Stall(),
        pub fn SetWatchdogTimer(),
        pub fn ConnectController(),
        pub fn DisconnectController(),
        pub fn OpenProtocol(EFI_HANDLE, *const EFI_GUID, *mut *const VOID, EFI_HANDLE, Option<EFI_HANDLE>, u32) -> EFI_STATUS,
        pub fn CloseProtocol(EFI_HANDLE, *const EFI_GUID, EFI_HANDLE, Option<EFI_HANDLE>) -> EFI_STATUS,
        pub fn OpenProtocolInformation(),
        pub fn ProtocolsPerHandle(),
        pub fn LocateHandleBuffer(),
        pub fn LocateProtocol(),
        pub fn InstallMultipleProtocolInterfaces(),
        pub fn UninstallMultipleProtocolInterfaces(),
        pub fn CalculateCrc32(),
        pub fn CopyMem(),
        pub fn SetMem(),
        pub fn CreateEventEx(u32, EFI_TPL, Option<EFI_EVENT_NOTIFY>, *const VOID, *const EFI_GUID, *mut EFI_EVENT) -> EFI_STATUS,
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EFI_ALLOCATE_TYPE {
    AllocateAnyPages = 0,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EFI_MEMORY_TYPE {
    EfiReservedMemoryType = 0,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiMaxMemoryType,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EFI_PHYSICAL_ADDRESS(pub u64);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EFI_MEMORY_DESCRIPTOR {
    pub Type: u32,
    pub PhysicalStart: EFI_PHYSICAL_ADDRESS,
    pub VirtualStart: EFI_VIRTUAL_ADDRESS,
    pub NumberOfPages: u64,
    pub Attribute: u64,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EFI_VIRTUAL_ADDRESS(pub u64);

pub type EFI_EVENT_NOTIFY = extern "win64" fn(EFI_EVENT, *const VOID);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EFI_INTERFACE_TYPE {
    EFI_NATIVE_INTERFACE = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EFI_TIMER_DELAY {
    TimerCancel = 0,
    TimerPeriodic,
    TimerRelative,
}
