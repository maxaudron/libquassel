use std::vec::Vec;

use crate::primitive::BufferId;
use crate::{error::ProtocolError, serialize::*};

use crate::serialize::UserType;

/// The BufferInfo struct represents a BufferInfo as received in IRC
///
/// BufferInfo is, like all other struct based types, serialized sequentially.
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct BufferInfo {
    /// a unique, sequential id for the buffer
    pub id: BufferId,
    /// NetworkId of the network the buffer belongs to
    pub network_id: i32,
    /// The Type of the Buffer
    pub buffer_type: BufferType,
    /// BufferName as displayed to the user
    pub name: String,
}

impl Serialize for BufferInfo {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: Vec<u8> = Vec::new();

        values.append(&mut Self::NAME.serialize_utf8()?);
        values.append(&mut BufferId::serialize(&self.id)?);
        values.append(&mut i32::serialize(&self.network_id)?);
        values.append(&mut i16::serialize(&(self.buffer_type as i16))?);
        values.append(&mut vec![0, 0, 0, 0]);
        values.append(&mut String::serialize_utf8(&self.name)?);

        Ok(values)
    }
}

impl Deserialize for BufferInfo {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (_, id) = BufferId::parse(&b[0..4])?;
        let (_, network_id) = i32::parse(&b[4..8])?;
        let (_, buffer_type) = i16::parse(&b[8..10])?;

        // There are 4 additional undocumented Bytes in the BufferInfo
        // so we start at byte 14
        // TODO is groupid
        let (size, name) = String::parse_utf8(&b[14..])?;

        return Ok((
            14 + size,
            Self {
                id,
                network_id,
                buffer_type: BufferType::from(buffer_type),
                name,
            },
        ));
    }
}

impl UserType for BufferInfo {
    const NAME: &str = "BufferInfo";
}

/// The Type of the Buffer
#[repr(i16)]
#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub enum BufferType {
    Status = 0x01,
    Channel = 0x02,
    Query = 0x04,
    Group = 0x08,
}

impl From<i16> for BufferType {
    fn from(value: i16) -> Self {
        match value {
            0x01 => return Self::Status,
            0x02 => return Self::Channel,
            0x04 => return Self::Query,
            0x08 => return Self::Group,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bufferinfo_serialize() {
        let buffer = BufferInfo {
            id: BufferId(1),
            network_id: 1,
            buffer_type: BufferType::Channel,
            name: "#test".to_string(),
        };

        assert_eq!(
            buffer.serialize().unwrap(),
            [
                0, 0, 0, 10, 66, 117, 102, 102, 101, 114, 73, 110, 102, 111, 0, 0, 0, 8, 66, 117, 102, 102,
                101, 114, 73, 100, 0, 0, 0, 1, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 35, 116, 101, 115,
                116
            ]
        )
    }

    #[test]
    fn bufferinfo_deserialize() {
        let buffer = BufferInfo {
            id: BufferId(1),
            network_id: 1,
            buffer_type: BufferType::Channel,
            name: "#test".to_string(),
        };
        let bytes = vec![
            0, 0, 0, 1, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 35, 116, 101, 115, 116,
        ];

        assert_eq!(BufferInfo::parse(&bytes).unwrap(), (23, buffer))
    }
}
