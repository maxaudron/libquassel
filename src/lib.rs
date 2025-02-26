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
mod error;

#[allow(unused_variables, dead_code)]
#[cfg(feature = "framing")]
#[cfg_attr(docsrs, doc(cfg(feature = "framing")))]
/// Framing impl to be used with [`tokio_util::codec::Framed`]
pub mod frame;

#[cfg(all(feature = "client", feature = "server"))]
compile_error!("feature \"client\" and feature \"server\" cannot be enabled at the same time");

pub use crate::error::ProtocolError;

/// Traits for Serialization of objects
pub mod serialize;

/// Traits for parsing objects
pub mod deserialize;

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
