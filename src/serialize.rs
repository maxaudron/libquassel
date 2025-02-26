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

pub trait SerializeVariant: VariantType + Serialize {
    /// Default implementation that passes the serialization through to [SerializeVariantInner].
    /// [SerializeVariantInner] is automaticly implemented for all types that implement [Serialize]
    ///
    /// ```rust ignore
    /// self.serialize_variant_inner(Self::TYPE)
    /// ```
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

pub trait DeserializeVariant: VariantType + Deserialize
where
    Variant: From<Self>,
    Self: Sized,
{
    fn parse_variant(b: &[u8], len: usize) -> Result<(usize, Variant), ProtocolError> {
        let (vlen, value) = Self::parse(&b[len..])?;
        return Ok((len + vlen, value.into()));
    }
}

impl<T: VariantType + Deserialize> DeserializeVariant for T where Variant: From<T> {}
