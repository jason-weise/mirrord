use std::{
    io,
    net::{AddrParseError, SocketAddr},
};

use bincode::{Decode, Encode};
use thiserror::Error;

#[derive(Encode, Decode, Debug, PartialEq, Clone, Eq, Error)]
pub enum ResponseError {
    #[error("Index allocator is full, operation `{0}` failed!")]
    AllocationFailure(String),

    #[error("Failed to find resource `{0}`!")]
    NotFound(usize),

    #[error("Remote operation expected fd `{0}` to be a directory, but it's a file!")]
    NotDirectory(usize),

    #[error("Remote operation expected fd `{0}` to be a file, but it's a directory!")]
    NotFile(usize),

    #[error("IO failed for remote operation with `{0}!")]
    RemoteIO(RemoteIOError),

    #[error("DNS resolve failed with return code`{0}`")]
    DnsFailure(i32),

    #[error("Remote operation failed with `{0}`")]
    Remote(#[from] RemoteError),
}

#[derive(Encode, Decode, Debug, PartialEq, Clone, Eq, Error)]
pub enum RemoteError {
    #[error("Failed to find a nameserver when resolving DNS!")]
    NameserverNotFound,

    #[error("Failed parsing address into a `SocketAddr` with `{0}`!")]
    AddressParsing(String),

    #[error("Failed operation to `SocketAddr` with `{0}`!")]
    InvalidAddress(SocketAddr),

    /// Especially relevant for the outgoing traffic feature, when `golang` attempts to connect
    /// on both IPv6 and IPv4.
    #[error("Connect call to `SocketAddr` `{0}` timed out!")]
    ConnectTimedOut(SocketAddr),
}

impl From<AddrParseError> for RemoteError {
    fn from(fail: AddrParseError) -> Self {
        Self::AddressParsing(fail.to_string())
    }
}

/// Our internal version of Rust's `std::io::Error` that can be passed between mirrord-layer and
/// mirrord-agent.
#[derive(Encode, Decode, Debug, PartialEq, Clone, Eq, Error)]
#[error("Failed performing `getaddrinfo` with {raw_os_error:?} and kind {kind:?}!")]
pub struct RemoteIOError {
    pub raw_os_error: Option<i32>,
    pub kind: ErrorKindInternal,
}

impl From<io::Error> for ResponseError {
    fn from(io_error: io::Error) -> Self {
        Self::RemoteIO(RemoteIOError {
            raw_os_error: io_error.raw_os_error(),
            kind: From::from(io_error.kind()),
        })
    }
}

impl From<dns_lookup::LookupError> for ResponseError {
    fn from(error: dns_lookup::LookupError) -> Self {
        Self::DnsFailure(error.error_num())
    }
}

/// Alternative to `std::io::ErrorKind`, used to implement `bincode::Encode` and `bincode::Decode`.
#[derive(Encode, Decode, Debug, PartialEq, Clone, Copy, Eq)]
pub enum ErrorKindInternal {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    HostUnreachable,
    NetworkUnreachable,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    NetworkDown,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    NotADirectory,
    IsADirectory,
    DirectoryNotEmpty,
    ReadOnlyFilesystem,
    FilesystemLoop,
    StaleNetworkFileHandle,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    StorageFull,
    NotSeekable,
    FilesystemQuotaExceeded,
    FileTooLarge,
    ResourceBusy,
    ExecutableFileBusy,
    Deadlock,
    CrossesDevices,
    TooManyLinks,
    InvalidFilename,
    ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
}

impl const From<io::ErrorKind> for ErrorKindInternal {
    fn from(error_kind: io::ErrorKind) -> Self {
        match error_kind {
            io::ErrorKind::NotFound => ErrorKindInternal::NotFound,
            io::ErrorKind::PermissionDenied => ErrorKindInternal::PermissionDenied,
            io::ErrorKind::ConnectionRefused => ErrorKindInternal::ConnectionRefused,
            io::ErrorKind::ConnectionReset => ErrorKindInternal::ConnectionReset,
            io::ErrorKind::HostUnreachable => ErrorKindInternal::HostUnreachable,
            io::ErrorKind::NetworkUnreachable => ErrorKindInternal::NetworkUnreachable,
            io::ErrorKind::ConnectionAborted => ErrorKindInternal::ConnectionAborted,
            io::ErrorKind::NotConnected => ErrorKindInternal::NotConnected,
            io::ErrorKind::AddrInUse => ErrorKindInternal::AddrInUse,
            io::ErrorKind::AddrNotAvailable => ErrorKindInternal::AddrNotAvailable,
            io::ErrorKind::NetworkDown => ErrorKindInternal::NetworkDown,
            io::ErrorKind::BrokenPipe => ErrorKindInternal::BrokenPipe,
            io::ErrorKind::AlreadyExists => ErrorKindInternal::AlreadyExists,
            io::ErrorKind::WouldBlock => ErrorKindInternal::WouldBlock,
            io::ErrorKind::NotADirectory => ErrorKindInternal::NotADirectory,
            io::ErrorKind::IsADirectory => ErrorKindInternal::IsADirectory,
            io::ErrorKind::DirectoryNotEmpty => ErrorKindInternal::DirectoryNotEmpty,
            io::ErrorKind::ReadOnlyFilesystem => ErrorKindInternal::ReadOnlyFilesystem,
            io::ErrorKind::FilesystemLoop => ErrorKindInternal::FilesystemLoop,
            io::ErrorKind::StaleNetworkFileHandle => ErrorKindInternal::StaleNetworkFileHandle,
            io::ErrorKind::InvalidInput => ErrorKindInternal::InvalidInput,
            io::ErrorKind::InvalidData => ErrorKindInternal::InvalidData,
            io::ErrorKind::TimedOut => ErrorKindInternal::TimedOut,
            io::ErrorKind::WriteZero => ErrorKindInternal::WriteZero,
            io::ErrorKind::StorageFull => ErrorKindInternal::StorageFull,
            io::ErrorKind::NotSeekable => ErrorKindInternal::NotSeekable,
            io::ErrorKind::FilesystemQuotaExceeded => ErrorKindInternal::FilesystemQuotaExceeded,
            io::ErrorKind::FileTooLarge => ErrorKindInternal::FileTooLarge,
            io::ErrorKind::ResourceBusy => ErrorKindInternal::ResourceBusy,
            io::ErrorKind::ExecutableFileBusy => ErrorKindInternal::ExecutableFileBusy,
            io::ErrorKind::Deadlock => ErrorKindInternal::Deadlock,
            io::ErrorKind::CrossesDevices => ErrorKindInternal::CrossesDevices,
            io::ErrorKind::TooManyLinks => ErrorKindInternal::TooManyLinks,
            io::ErrorKind::InvalidFilename => ErrorKindInternal::InvalidFilename,
            io::ErrorKind::ArgumentListTooLong => ErrorKindInternal::ArgumentListTooLong,
            io::ErrorKind::Interrupted => ErrorKindInternal::Interrupted,
            io::ErrorKind::Unsupported => ErrorKindInternal::Unsupported,
            io::ErrorKind::UnexpectedEof => ErrorKindInternal::UnexpectedEof,
            io::ErrorKind::OutOfMemory => ErrorKindInternal::OutOfMemory,
            io::ErrorKind::Other => ErrorKindInternal::Other,
            _ => unimplemented!(),
        }
    }
}
