use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("message has wrong type")]
    WrongMsgType,
    #[error("message has unkown type")]
    UnknownMsgType,
    #[error("bool value is neither 0 nor 1")]
    BoolOutOfRange,
    #[error("Sync Message does not contain any more parameters")]
    MissingSyncMessageParams,
    #[error("QVariant is not known")]
    UnknownVariant,
    #[error("UserType is not known: {0}")]
    UnknownUserType(String),
    #[error("wrong variant has been given")]
    WrongVariant,
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("could not convert from int: {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("utf16 error: {0}")]
    Utf16Error(#[from] std::string::FromUtf16Error),
    #[error("errored to parse char as utf16")]
    CharError,
    #[error("failed to deal with time: {0}")]
    TimeError(#[from] time::error::ComponentRange),
    #[error("failed to parse int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("error in sync proxy: {0}")]
    SyncProxyError(#[from] SyncProxyError),
    #[error("error in features: {0}")]
    FeatureError(#[from] FeatureError),
    #[error("got unkown HighlightNickType: {0}")]
    UnknownHighlightNickType(i32),
    #[error("got unkown IgnoreType: {0}")]
    UnknownIgnoreType(i32),
    #[error("got unkown StrictnessType: {0}")]
    UnknownStrictnessType(i32),
    #[error("got unkown ScopeType: {0}")]
    UnknownScopeType(i32),
    #[error("got unkown message slot_name: {0}")]
    UnknownMsgSlotName(String),
    #[error("got unknown connection state")]
    UnknownConnectionState,
    #[error("failed parsing object name: {0}")]
    BrokenObjectName(String),

    // TODO potentially move this to a higher error type for the object implementations
    // to sepperate it from parser errors
    #[error("buffer was not found: {0:?}")]
    BufferNotFound(crate::primitive::BufferId)
}

#[derive(Debug, Error)]
pub enum SyncProxyError {
    #[error("SYNC_PROXY was already initialized")]
    AlreadyInitialized,
    #[error("SYNC_PROXY was not yet initialized")]
    NotInitialized,
}

#[derive(Debug, Error)]
pub enum FeatureError {
    #[error("FEATURES was already initialized")]
    AlreadyInitialized,
    #[error("FEAETURES was not yet initialized")]
    NotInitialized,
}

pub type Result<T> = std::result::Result<T, ProtocolError>;

// impl std::error::Error for ErrorKind {}
//
// impl std::convert::From<std::io::Error> for ErrorKind {
//     fn from(error: std::io::Error) -> Self {
//         ErrorKind::IOError(error)
//     }
// }
//
// impl std::convert::From<std::num::TryFromIntError> for ErrorKind {
//     fn from(error: std::num::TryFromIntError) -> Self {
//         ErrorKind::TryFromIntError(error)
//     }
// }
//
// impl std::convert::From<std::string::FromUtf8Error> for ErrorKind {
//     fn from(error: std::string::FromUtf8Error) -> Self {
//         ErrorKind::Utf8Error(error)
//     }
// }
