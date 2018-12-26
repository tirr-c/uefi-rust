pub type VOID = core::ffi::c_void;
pub type CHAR16 = u16;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum BOOL {
    FALSE = 0,
    TRUE = 1,
}

impl From<bool> for BOOL {
    fn from(val: bool) -> BOOL {
        if val {
            BOOL::TRUE
        } else {
            BOOL::FALSE
        }
    }
}

impl Into<bool> for BOOL {
    fn into(self) -> bool {
        match self {
            BOOL::TRUE => true,
            BOOL::FALSE => false,
        }
    }
}

#[repr(isize)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EFI_STATUS {
    /// The operation completed successfully.
    EFI_SUCCESS = 0,
    /// The string contained one or more characters that the device could not render and were skipped.
    EFI_WARN_UNKNOWN_GLYPH = 1,
    /// The handle was closed, but the file was not deleted.
    EFI_WARN_DELETE_FAILURE = 2,
    /// The handle was closed, but the data to the file was not flushed properly.
    EFI_WARN_WRITE_FAILURE = 3,
    /// The resulting buffer was too small, and the data was truncated to the buffer size.
    EFI_WARN_BUFFER_TOO_SMALL = 4,
    /// The data has not been updated within the timeframe set by local policy for this type of data.
    EFI_WARN_STALE_DATA = 5,
    /// The resulting buffer contains Uefi-compliant file system.
    EFI_WARN_FILE_SYSTEM = 6,
    /// The operation will be processed across a system reset.
    EFI_WARN_RESET_REQUIRED = 7,
    /// The image failed to load.
    EFI_LOAD_ERROR = -1,
    /// A parameter was incorrect.
    EFI_INVALID_PARAMETER = -2,
    /// The operation is not supported.
    EFI_UNSUPPORTED = -3,
    /// The buffer was not the proper size for the request.
    EFI_BAD_BUFFER_SIZE = -4,
    /// The buffer is not large enough to hold the requested data. The required buffer size is returned in the appropriate parameter when this error occurs.
    EFI_BUFFER_TOO_SMALL = -5,
    /// There is no data pending upon return.
    EFI_NOT_READY = -6,
    /// The physical device reported an error while attempting the operation.
    EFI_DEVICE_ERROR = -7,
    /// The device cannot be written to.
    EFI_WRITE_PROTECTED = -8,
    /// A resource has run out.
    EFI_OUT_OF_RESOURCES = -9,
    /// An inconstancy was detected on the file system causing the operating to fail.
    EFI_VOLUME_CORRUPTED = -10,
    /// There is no more space on the file system.
    EFI_VOLUME_FULL = -11,
    /// The device does not contain any medium to perform the operation.
    EFI_NO_MEDIA = -12,
    /// The medium in the device has changed since the last access.
    EFI_MEDIA_CHANGED = -13,
    /// The item was not found.
    EFI_NOT_FOUND = -14,
    /// Access was denied.
    EFI_ACCESS_DENIED = -15,
    /// The server was not found or did not respond to the request.
    EFI_NO_RESPONSE = -16,
    /// A mapping to a device does not exist.
    EFI_NO_MAPPING = -17,
    /// The timeout time expired.
    EFI_TIMEOUT = -18,
    /// The protocol has not been started.
    EFI_NOT_STARTED = -19,
    /// The protocol has already been started.
    EFI_ALREADY_STARTED = -20,
    /// The operation was aborted.
    EFI_ABORTED = -21,
    /// An ICMP error occurred during the network operation.
    EFI_ICMP_ERROR = -22,
    /// A TFTP error occurred during the network operation.
    EFI_TFTP_ERROR = -23,
    /// A protocol error occurred during the network operation.
    EFI_PROTOCOL_ERROR = -24,
    /// The function encountered an internal version that was incompatible with a version requested by the caller.
    EFI_INCOMPATIBLE_VERSION = -25,
    /// The function was not performed due to a security violation.
    EFI_SECURITY_VIOLATION = -26,
    /// A CRC error was detected.
    EFI_CRC_ERROR = -27,
    /// Beginning or end of media was reached.
    EFI_END_OF_MEDIA = -28,
    /// The end of the file was reached.
    EFI_END_OF_FILE = -31,
    /// The language specified was invalid.
    EFI_INVALID_LANGUAGE = -32,
    /// The security status of the data is unknown or compromised and the data must be updated or replaced to restore a valid security status.
    EFI_COMPROMISED_DATA = -33,
    /// There is an address conflict address allocation.
    EFI_IP_ADDRESS_CONFLICT = -34,
    /// A HTTP error occurred during the network operation.
    EFI_HTTP_ERROR = -35,
}

impl EFI_STATUS {
    pub fn is_ok(self) -> bool {
        self == EFI_STATUS::EFI_SUCCESS
    }

    pub fn is_warning(self) -> bool {
        (self as isize) > 0
    }

    pub fn is_error(self) -> bool {
        (self as isize) < 0
    }
}

impl Into<Result<(), EFI_STATUS>> for EFI_STATUS {
    fn into(self) -> Result<(), EFI_STATUS> {
        if self.is_error() {
            Err(self)
        } else {
            Ok(())
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EFI_GUID(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EFI_HANDLE(core::ptr::NonNull<VOID>);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EFI_EVENT(core::ptr::NonNull<VOID>);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EFI_TPL(pub usize);

impl EFI_TPL {
    pub const TPL_APPLICATION: EFI_TPL = EFI_TPL(4);
    pub const TPL_CALLBACK: EFI_TPL = EFI_TPL(8);
    pub const TPL_NOTIFY: EFI_TPL = EFI_TPL(16);
    pub const TPL_HIGH_LEVEL: EFI_TPL = EFI_TPL(31);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EFI_TABLE_HEADER {
    pub Signature: u64,
    pub Revision: u32,
    pub HeaderSize: u32,
    pub Crc32: u32,
    pub Reserved: u32,
}
