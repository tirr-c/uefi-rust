pub type Char = u16;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Bool {
    False = 0,
    True = 1,
}

#[repr(isize)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Status {
    /// The operation completed successfully.
    Success = 0,
    /// The string contained one or more characters that the device could not render and were skipped.
    WarnUnknownGlyph = 1,
    /// The handle was closed, but the file was not deleted.
    WarnDeleteFailure = 2,
    /// The handle was closed, but the data to the file was not flushed properly.
    WarnWriteFailure = 3,
    /// The resulting buffer was too small, and the data was truncated to the buffer size.
    WarnBufferTooSmall = 4,
    /// The data has not been updated within the timeframe set by local policy for this type of data.
    WarnStaleData = 5,
    /// The resulting buffer contains UEFI-compliant file system.
    WarnFileSystem = 6,
    /// The operation will be processed across a system reset.
    WarnResetRequired = 7,
    /// The image failed to load.
    LoadError = -1,
    /// A parameter was incorrect.
    InvalidParameter = -2,
    /// The operation is not supported.
    Unsupported = -3,
    /// The buffer was not the proper size for the request.
    BadBufferSize = -4,
    /// The buffer is not large enough to hold the requested data. The required buffer size is returned in the appropriate parameter when this error occurs.
    BufferTooSmall = -5,
    /// There is no data pending upon return.
    NotReady = -6,
    /// The physical device reported an error while attempting the operation.
    DeviceError = -7,
    /// The device cannot be written to.
    WriteProtected = -8,
    /// A resource has run out.
    OutOfResources = -9,
    /// An inconstancy was detected on the file system causing the operating to fail.
    VolumeCorrupted = -10,
    /// There is no more space on the file system.
    VolumeFull = -11,
    /// The device does not contain any medium to perform the operation.
    NoMedia = -12,
    /// The medium in the device has changed since the last access.
    MediaChanged = -13,
    /// The item was not found.
    NotFound = -14,
    /// Access was denied.
    AccessDenied = -15,
    /// The server was not found or did not respond to the request.
    NoResponse = -16,
    /// A mapping to a device does not exist.
    NoMapping = -17,
    /// The timeout time expired.
    Timeout = -18,
    /// The protocol has not been started.
    NotStarted = -19,
    /// The protocol has already been started.
    AlreadyStarted = -20,
    /// The operation was aborted.
    Aborted = -21,
    /// An ICMP error occurred during the network operation.
    IcmpError = -22,
    /// A TFTP error occurred during the network operation.
    TftpError = -23,
    /// A protocol error occurred during the network operation.
    ProtocolError = -24,
    /// The function encountered an internal version that was incompatible with a version requested by the caller.
    IncompatibleVersion = -25,
    /// The function was not performed due to a security violation.
    SecurityViolation = -26,
    /// A CRC error was detected.
    CrcError = -27,
    /// Beginning or end of media was reached.
    EndOfMedia = -28,
    /// The end of the file was reached.
    EndOfFile = -31,
    /// The language specified was invalid.
    InvalidLanguage = -32,
    /// The security status of the data is unknown or compromised and the data must be updated or replaced to restore a valid security status.
    CompromisedData = -33,
    /// There is an address conflict address allocation.
    IpAddressConflict = -34,
    /// A HTTP error occurred during the network operation.
    HttpError = -35,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Handle(*const core::ffi::c_void);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Event(*const core::ffi::c_void);
