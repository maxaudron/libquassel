#![cfg_attr(all(test, feature = "bench"), feature(test))]
#[cfg(all(test, feature = "bench"))]
extern crate test;

#[doc = include_str!("../README.md")]
#[cfg_attr(docsrs, feature(doc_cfg))]
extern crate self as libquassel;

#[macro_use]
mod util;

/// Quassel Structures for serialization and deserialization
pub mod message;

/// Quassels QT based primitive types that make up the more complex messages
pub mod primitive;

pub mod session;

#[allow(dead_code)]
/// Error Types
pub mod error;

#[allow(unused_variables, dead_code)]
#[cfg(feature = "framing")]
#[cfg_attr(docsrs, doc(cfg(feature = "framing")))]
/// Framing impl to be used with [`tokio_util::codec::Framed`]
pub mod frame;

#[cfg(all(feature = "client", feature = "server"))]
compile_error!("feature \"client\" and feature \"server\" cannot be enabled at the same time");

use crate::error::ProtocolError;

/// Traits for Serialization of objects
pub mod serialize {
    use crate::error::ProtocolError;

    /// Serialization of types and structs to the quassel byteprotocol
    pub trait Serialize {
        fn serialize(&self) -> Result<Vec<u8>, ProtocolError>;
    }

    /// Serialization of UTF-8 based Strings to the quassel byteprotocol
    pub trait SerializeUTF8 {
        fn serialize_utf8(&self) -> Result<Vec<u8>, ProtocolError>;
    }

    pub trait SerializeVariant {
        fn serialize_variant(&self) -> Result<Vec<u8>, ProtocolError>;
    }
}

/// Traits for parsing objects
pub mod deserialize {
    use crate::error::ProtocolError;

    /// Deserialization of types and structs to the quassel byteprotocol
    pub trait Deserialize {
        fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError>
        where
            Self: std::marker::Sized;
    }

    /// Deserialization of UTF-8 based Strings to the quassel byteprotocol
    pub trait DeserializeUTF8 {
        fn parse_utf8(b: &[u8]) -> Result<(usize, Self), ProtocolError>
        where
            Self: std::marker::Sized;
    }

    pub trait DeserializeVariant {
        fn parse_variant(b: &[u8]) -> Result<(usize, Self), ProtocolError>
        where
            Self: std::marker::Sized;
    }
}

/// HandshakeSerialize implements the serialization needed during the handhake phase.
///
/// The protocol has some minor differences during this phase compared to the regular parsing.
pub trait HandshakeSerialize {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError>;
}

/// HandshakeDeserialize implements the deserialization needed during the handhake phase.
///
/// The protocol has some minor differences during this phase compared to the regular parsing.
pub trait HandshakeDeserialize {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError>
    where
        Self: std::marker::Sized;
}
