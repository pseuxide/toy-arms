use thiserror::Error;

#[derive(Debug, Error)]
pub enum TAExternalError {
    #[error("Snapshot was failed for some reason. To see the detail, destructure the inner error enum.")]
    SnapshotFailed(SnapshotFailedDetail),
    #[error("Specified process was not found. Make sure the process is running and the name is correct.")]
    ProcessNotFound,
    #[error("Specified module was not found in the process. Make sure the name is correct.")]
    ModuleNotFound,
    #[error("read failed for some reason. To see the detail, destructure the inner error enum.")]
    ReadMemoryFailed(ReadWriteMemoryFailedDetail),
    #[error("write failed for some reason. To see the detail, destructure the inner error enum.")]
    WriteMemoryFailed(ReadWriteMemoryFailedDetail),
}

#[derive(Debug, Error)]
pub enum ReadWriteMemoryFailedDetail {
    #[error("Attempt to access invalid address")]
    ErrorInvalidAddress,
    #[error("Only part of a ReadProcessMemory or WriteProcessMemory request was completed. The page might be protected.")]
    ErrorPartialCopy,
    #[error("The given handle is invalid.")]
    ErrorInvalidHandle,
    #[error("Error code {} is not supported by this library", error_code)]
    UnknownError{ error_code: u32 },
}

#[derive(Debug, Error)]
pub enum SnapshotFailedDetail {
    #[error("Invalid handle has been retrieved from CreateToolhelp32Snapshot")]
    InvalidHandle,
    #[error("No more files exist in the given snap by CreateToolhelp32Snapshot")]
    NoMoreFiles,
}
