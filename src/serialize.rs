//! Traits for Serializing and Deserializing types to their byte representations
//!
//! example for a simple type would look like this with the `TYPE` usually fetched from [libquassel::primitive#constants]
//!
//! ```rust
//! use libquassel::{ProtocolError, serialize::*};
//!
//! pub struct Example;
//!
//! impl Serialize for Example {
//!     fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
//!         // Serialize here
//!         # todo!()
//!     }
//! }
//!
//! impl Deserialize for Example {
//!     fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
//!         // Deserialize here
//!         # todo!()
//!     }
//! }
//!
//! impl VariantType for Example {
//!     // The QT Type identifier
//!     const TYPE: u32 = 0x18;
//! }
//! ```
//!
//! Some types are not serialized directly but are a [UserType]:
//! ```rust
//! use libquassel::{ProtocolError, serialize::*};
//!
//! pub struct Example;
//!
//! impl Serialize for Example {
//!     fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
//!         // Serialize here
//!         # todo!()
//!     }
//! }
//!
//! impl UserType for Example {
//!     const NAME: &str = "Example";
//! }
//! ```
//! [UserType]: libquassel::serialize::UserType

use crate::{
    error::ProtocolError,
    primitive::{self, Variant},
};

/// Sets the usertype name to be used in serialization and deserialization of the types variants.
/// Automaticly implements the [SerializeVariant] trait.
///
/// Example [Serialize] impl for a `struct NewType(T: Serialize)`:
/// ```rust
/// use libquassel::serialize::{Serialize, SerializeUTF8, UserType};
///
/// struct NewType(i64);
/// impl Serialize for NewType {
///     fn serialize(&self) -> Result<Vec<u8>, libquassel::ProtocolError> {
///         let mut res = Vec::new();
///
///         res.append(&mut Self::NAME.serialize_utf8()?);
///         res.extend(self.0.serialize()?);
///
///         Ok(res)
///     }
/// }
///
/// impl UserType for NewType {
///   const NAME: &str = "NewType";
/// }
/// ```
pub trait UserType {
    const NAME: &str;
}

impl<T: UserType> VariantType for T {
    const TYPE: u32 = primitive::USERTYPE;
}

// =============================================
// Serialization
//

/// Serialization of types and structs to the quassel byteprotocol
pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError>;
}

/// Serialization of UTF-8 based Strings to the quassel byteprotocol
pub trait SerializeUTF8 {
    fn serialize_utf8(&self) -> Result<Vec<u8>, ProtocolError>;
}

/// Provides a easy default implementation for serializing a type to it's [Variant] given it's QT type id.
///
/// If the type is a UserType implement only [UserType], SerializeVariant is implemented automaticly.
///
/// ```rust ignore
/// impl SerializeVariant for VariantList {
///     const TYPE: u32 = primitive::QVARIANTLIST;
/// }
/// ```
pub trait VariantType {
    /// QT Type as defined by the [Primitive Constants]
    ///
    /// [Primitive Constants]: primitive#constants
    const TYPE: u32;
}

/// Serialize a Type directly to it's Variant.
///
/// Has a default implementation and is automaticly implemented for all types that
/// have a [VariantType] set and [Serialize] implemented.
pub trait SerializeVariant: VariantType + Serialize {
    fn serialize_variant(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut res: Vec<u8> = Vec::new();

        res.extend(Self::TYPE.to_be_bytes().iter());
        res.extend(0x00u8.to_be_bytes().iter());
        res.extend(self.serialize()?);

        Ok(res)
    }
}

impl<T: VariantType + Serialize> SerializeVariant for T {}

// =============================================
// Deserialization
//

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

/// Deserialize a Type for use in the Variant parsing. In opposite to [SerializeVariant] this does not deal
/// with the variant type itself as we have to match onto it genericly before passing it on to per Type
/// functions.
///
/// Still using this gives us automatic implementations and more code reuse
///
/// Has a default implementation and is automaticly implemented for all types that
/// have a [VariantType] set and [Deserialize] implemented.
pub trait DeserializeVariant: VariantType + Deserialize
where
    Variant: From<Self>,
    Self: Sized,
{
    fn parse_variant(b: &[u8], len: usize) -> Result<(usize, Variant), ProtocolError> {
        let (vlen, value) = Self::parse(&b[len..])?;
        Ok((len + vlen, value.into()))
    }
}

impl<T: VariantType + Deserialize> DeserializeVariant for T where Variant: From<T> {}
