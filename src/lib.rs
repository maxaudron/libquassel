#![doc = include_str!("../README.md")]
#![cfg_attr(all(test, feature = "bench"), feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

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

/// Traits for Serialization & Deserialization of objects
pub mod serialize;

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

/// Derive the [message::NetworkList] and [message::NetworkMap] traits.
///
/// This provides easy and boilerplate free implementations for the network translations from and to the raw
/// Variants. For details on the different kinds of mappings see [message::translation]
///
/// ```rust ignore
/// use libquassel::{NetworkList, NetworkMap};
///
/// #[derive(NetworkMap, NetworkList)]
/// #[network(repr = "map")]
/// pub struct Example {
///     #[network(rename = "Name", default)]
///     pub name: String,
///     #[network(rename = "Nested", network = "map", variant = "VariantMap")]
///     pub nested: Nested,
/// }
///
/// type Value = i32;
///
/// #[derive(NetworkMap, NetworkList)]
/// #[network(repr = "maplist")]
/// pub struct Nested {
///     #[network(rename = "Name", type = "i32")]
///     pub field: Value
/// }
/// ```
///
/// ## Attributes
/// ### Object Attributes
/// - `#[network(repr = "...")]`
///
///   Sets the network representation used for this object. Only applies to [NetworkMap]
///
///   Either [`maplist`] or [`map`]
///
/// ### Field Attributes
/// - `#[network(rename = "name")]`
///
///   Renames the field to change case or spelling etc.
///
/// - `#[network(skip)]`
///
///   Ignore this field completly in both to and from conversions.
/// - `#[network(default)]`
///
///   If the field is not set in the network representation, fall back on [`Default::default()`].
///
///   May be used if the network representation does not always contain a field.
///
/// - `#[network(network = "...")]`
///
///   Use either NetworkMap or NetworkList translation trait to convert this field depending
///   on if `map` or `list` is set respectivly.
///
/// - `#[network(variant = "...")]`
///
///   Override the variant used to convert the field, instead of the fields type itself.
///
///   E.g if the field is of type `String` but the network representation wants `ByteArray` then
///   set this here.
///
/// - `#[network(type = "...")]`
///
///   Override the type of the field that is used e.g. when you are using a type alias.
///
///   This differs from setting the `variant` as it will also influence the inner type instead
///   of just the wrapping Variant
///
/// - `#[network(stringlist)]`
///
///   Special case handling for if the field type is `Vec<String>`.
///
///   This would by default be converted to a `Variant::VariantList(Vec<Variant::String>)` but for
///   strings the [StringList] representation is used: `Variant::StringList(Vec<String>)`
///
/// [`maplist`]: libquassel::message::translation#structure-of-arrays
/// [`map`]: libquassel::message::translation#variantmap
/// [StringList]: libquassel::primitive::StringList
pub use libquassel_derive::{NetworkList, NetworkMap};

/// send a syncmessage using [`Syncable::send_sync`] and casts it's arguments to the correct Variants
///
/// Example:
/// ```rust ignore
/// sync!("requestCreateBufferView", [properties.to_network_map()])
/// ```
///
/// [`Syncable::send_sync`]: libquassel::message::Syncable::send_sync
pub use libquassel_derive::sync;
