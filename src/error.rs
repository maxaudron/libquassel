use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("message has wrong type")]
    WrongMsgType,
    #[error("bool value is neither 0 nor 1")]
    BoolOutOfRange,
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
    #[error("errored to parse char as utf16")]
    CharError,
    #[error("failed to deal with time: {0}")]
    TimeError(#[from] time::error::ComponentRange),
}

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
