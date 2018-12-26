use uefi_sys::{CHAR16, EFI_GUID, EFI_HANDLE, EFI_STATUS, VOID};

pub type Void = VOID;
pub type Handle = EFI_HANDLE;
pub type Guid = EFI_GUID;
pub type EfiChar = CHAR16;
pub type Status = EFI_STATUS;

pub type EfiResult<T> = Result<T, Status>;
