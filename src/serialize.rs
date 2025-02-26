use crate::{error::ProtocolError, primitive};

/// Serialization of types and structs to the quassel byteprotocol
pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError>;
}

/// Serialization of UTF-8 based Strings to the quassel byteprotocol
pub trait SerializeUTF8 {
    fn serialize_utf8(&self) -> Result<Vec<u8>, ProtocolError>;
}

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

/// Provides a easy default implementation for serializing a type to it's [Variant] given it's QT type id.
///
/// If the type is a UserType implement only [UserType], SerializeVariant is implemented automaticly.
///
/// ```rust ignore
/// impl SerializeVariant for VariantList {
///     const TYPE: u32 = primitive::QVARIANTLIST;
/// }
/// ```
pub trait SerializeVariant {
    /// QT Type as defined by the [Primitive Constants]
    ///
    /// [Primitive Constants]: primitive#constants
    const TYPE: u32;

    /// Default implementation that passes the serialization through to [SerializeVariantInner].
    /// [SerializeVariantInner] is automaticly implemented for all types that implement [Serialize]
    ///
    /// ```rust ignore
    /// self.serialize_variant_inner(Self::TYPE)
    /// ```
    fn serialize_variant(&self) -> Result<Vec<u8>, ProtocolError>
    where
        Self: SerializeVariantInner,
    {
        self.serialize_variant_inner(Self::TYPE)
    }
}

/// Implemented automaticly for all types that implement [Serialize] refer to [SerializeVariant]
pub trait SerializeVariantInner {
    fn serialize_variant_inner(&self, t: u32) -> Result<Vec<u8>, ProtocolError>;
}

impl<T: Serialize> SerializeVariantInner for T {
    fn serialize_variant_inner(&self, t: u32) -> Result<Vec<u8>, ProtocolError> {
        let mut res: Vec<u8> = Vec::new();

        res.extend(t.to_be_bytes().iter());
        res.extend(0x00u8.to_be_bytes().iter());
        res.extend(self.serialize()?);

        Ok(res)
    }
}

impl<T: UserType> SerializeVariant for T {
    const TYPE: u32 = primitive::USERTYPE;
}
