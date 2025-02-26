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
