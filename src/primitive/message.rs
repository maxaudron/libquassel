use std::{collections::HashMap, vec::Vec};

use crate::error::ProtocolError;
use crate::message::Feature;
use crate::serialize::*;

use crate::primitive::{BufferInfo, DateTime, DateTimeTools, MsgId};

use super::{Variant, VariantList};
use crate::serialize::UserType;

/// The Message struct represents a Message as received in IRC
///
/// Messages are, like all other struct based types, serialized sequentially.
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct Message {
    /// The unique, sequential id for the message
    pub msg_id: MsgId,
    /// The timestamp of the message in UNIX time.
    /// If long-time is disabled this is an i32 representing the seconds since EPOCH.
    /// If long-time is enabled this is an i64 representing the miliseconds since EPOCH.
    pub timestamp: DateTime,
    /// The message type as it's own type serialized as i32
    pub msg_type: MessageType,
    /// TODO The flags
    pub flags: i8,
    /// The buffer the message belongs to, usually everything but BufferId is set to NULL
    pub buffer: BufferInfo,
    /// The sender as nick!ident@host
    pub sender: String,
    /// The prefix modes of the sender.
    /// Feature: SenderPrefixes
    pub sender_prefixes: Option<String>,
    /// The realName of the sender
    /// Feature: RichMessages
    pub real_name: Option<String>,
    /// The avatarUrl of the sender, if available
    /// Feature: RichMessages
    pub avatar_url: Option<String>,
    /// The message content, already stripped from CTCP formatting, but containing mIRC format codes
    pub content: String,
}

impl Serialize for Message {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: Vec<u8> = Vec::new();

        values.append(&mut MsgId::serialize(&self.msg_id)?);

        if Feature::LongTime.enabled()? {
            values.append(&mut self.timestamp.to_i64().serialize()?);
        } else {
            values.append(&mut self.timestamp.to_i32()?.serialize()?);
        }

        values.append(&mut i32::serialize(&(self.msg_type.bits()))?);
        values.append(&mut i8::serialize(&self.flags)?);
        values.append(&mut BufferInfo::serialize(&self.buffer)?);
        values.append(&mut String::serialize_utf8(&self.sender)?);

        if let Some(prefix) = self.sender_prefixes.as_ref()
            && Feature::SenderPrefixes.enabled()?
        {
            values.append(&mut String::serialize_utf8(prefix)?);
        }

        if Feature::RichMessages.enabled()?
            && let Some(real_name) = self.real_name.as_ref()
            && let Some(avatar_url) = self.avatar_url.as_ref()
        {
            values.append(&mut String::serialize_utf8(real_name)?);
            values.append(&mut String::serialize_utf8(avatar_url)?);
        }

        values.append(&mut String::serialize_utf8(&self.content)?);

        Ok(values)
    }
}

impl Deserialize for Message {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut pos = 0;
        let (parsed, msg_id) = MsgId::parse(&b[pos..])?;
        pos += parsed;

        let timestamp = if Feature::LongTime.enabled()? {
            let (parsed, temp_timestamp) = i64::parse(&b[pos..])?;
            pos += parsed;
            DateTime::from_i64(temp_timestamp)?
        } else {
            let (parsed, temp_timestamp) = i32::parse(&b[pos..])?;
            pos += parsed;
            DateTime::from_i32(temp_timestamp)?
        };

        let (parsed, msg_type) = i32::parse(&b[pos..])?;
        pos += parsed;
        let (parsed, flags) = i8::parse(&b[pos..])?;
        pos += parsed;
        let (parsed, buffer) = BufferInfo::parse(&b[pos..])?;
        pos += parsed;
        let (parsed, sender) = String::parse_utf8(&b[pos..])?;
        pos += parsed;

        let mut sender_prefixes = None;
        if Feature::SenderPrefixes.enabled()? {
            let (parsed, temp) = String::parse_utf8(&b[pos..])?;
            sender_prefixes = Some(temp);
            pos += parsed;
        }

        let mut real_name = None;
        let mut avatar_url = None;
        if Feature::RichMessages.enabled()? {
            let (parsed, temp) = String::parse_utf8(&b[pos..])?;
            real_name = Some(temp);
            pos += parsed;

            let (parsed, temp) = String::parse_utf8(&b[pos..])?;
            avatar_url = Some(temp);
            pos += parsed;
        }

        let (parsed, content) = String::parse_utf8(&b[pos..])?;
        pos += parsed;

        Ok((
            pos,
            Self {
                msg_id,
                timestamp,
                msg_type: MessageType::from_bits(msg_type).ok_or(ProtocolError::UnknownMsgType)?,
                flags,
                buffer,
                sender,
                sender_prefixes,
                real_name,
                avatar_url,
                content,
            },
        ))
    }
}

impl UserType for Message {
    const NAME: &str = "Message";
}

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MessageType: i32 {
        const NONE = 0x00000000;
        const PLAIN = 0x00000001;
        const NOTICE = 0x00000002;
        const ACTION = 0x00000004;
        const NICK = 0x00000008;
        const MODE = 0x00000010;
        const JOIN = 0x00000020;
        const PART = 0x00000040;
        const QUIT = 0x00000080;
        const KICK = 0x00000100;
        const KILL = 0x00000200;
        const SERVER = 0x00000400;
        const INFO = 0x00000800;
        const ERROR = 0x00001000;
        const DAY_CHANGE = 0x00002000;
        const TOPIC = 0x00004000;
        const NETSPLIT_JOIN = 0x00008000;
        const NETSPLIT_QUIT = 0x00010000;
        const INVITE = 0x00020000;
        const MARKERLINE = 0x00040000;
    }
}

impl<T> crate::message::NetworkList for HashMap<T, MessageType>
where
    T: std::convert::TryFrom<Variant> + Into<Variant> + Clone + std::hash::Hash + std::cmp::Eq,
{
    fn to_network_list(&self) -> Result<VariantList, ProtocolError> {
        let mut res = Vec::with_capacity(self.len() * 2);

        self.iter().for_each(|(k, v)| {
            res.push((*k).clone().into());
            res.push((*v).clone().bits().into());
        });

        Ok(res)
    }

    fn from_network_list(input: VariantList) -> Result<Self, ProtocolError> {
        use itertools::Itertools;

        let mut res = HashMap::with_capacity(input.len() / 2);

        for (k, v) in input.into_iter().tuples() {
            res.insert(
                match T::try_from(k.clone()) {
                    Ok(it) => it,
                    _ => unreachable!(),
                },
                {
                    let typ = v.try_into()?;
                    MessageType::from_bits(typ).ok_or(ProtocolError::UnknownMsgType)?
                },
            );
        }

        Ok(res)
    }
}

#[cfg(test)]
#[cfg(feature = "all-quassel-features")]
mod tests {
    use super::*;
    use crate::primitive::{BufferId, BufferInfo, BufferType};

    #[test]
    fn message_serialize() {
        let _ = Feature::enable_all();
        let message = Message {
            msg_id: MsgId(1),
            timestamp: DateTime::from_i64(1609846597).unwrap(),
            msg_type: MessageType::PLAIN,
            flags: 0,
            buffer: BufferInfo {
                id: BufferId(1),
                network_id: 1,
                buffer_type: BufferType::Channel,
                name: "#test".to_string(),
            },
            sender: "test".to_string(),
            content: "this is a test message".to_string(),
            sender_prefixes: Some("blabla".to_string()),
            real_name: Some("test user".to_string()),
            avatar_url: Some("https://jfkalsdkjfj.com/kjkj".to_string()),
        };

        assert_eq!(
            message.serialize().unwrap(),
            [
                0, 0, 0, 5, 77, 115, 103, 73, 100, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 95, 244, 79, 69, 0, 0,
                0, 1, 0, 0, 0, 0, 10, 66, 117, 102, 102, 101, 114, 73, 110, 102, 111, 0, 0, 0, 8, 66, 117,
                102, 102, 101, 114, 73, 100, 0, 0, 0, 1, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 35, 116,
                101, 115, 116, 0, 0, 0, 4, 116, 101, 115, 116, 0, 0, 0, 6, 98, 108, 97, 98, 108, 97, 0, 0, 0,
                9, 116, 101, 115, 116, 32, 117, 115, 101, 114, 0, 0, 0, 28, 104, 116, 116, 112, 115, 58, 47,
                47, 106, 102, 107, 97, 108, 115, 100, 107, 106, 102, 106, 46, 99, 111, 109, 47, 107, 106,
                107, 106, 0, 0, 0, 22, 116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32,
                109, 101, 115, 115, 97, 103, 101
            ]
        )
    }

    #[test]
    fn message_deserialize() {
        let _ = Feature::enable_all();
        let message = Message {
            msg_id: MsgId(1),
            timestamp: DateTime::from_i64(1609846597).unwrap(),
            msg_type: MessageType::PLAIN,
            flags: 0,
            buffer: BufferInfo {
                id: BufferId(1),
                network_id: 1,
                buffer_type: BufferType::Channel,
                name: "#test".to_string(),
            },
            sender: "test".to_string(),
            content: "this is a test message".to_string(),
            sender_prefixes: Some("blabla".to_string()),
            real_name: Some("test user".to_string()),
            avatar_url: Some("https://jfkalsdkjfj.com/kjkj".to_string()),
        };

        let bytes = vec![
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 95, 244, 79, 69, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 2,
            0, 0, 0, 0, 0, 0, 0, 5, 35, 116, 101, 115, 116, 0, 0, 0, 4, 116, 101, 115, 116, 0, 0, 0, 6, 98,
            108, 97, 98, 108, 97, 0, 0, 0, 9, 116, 101, 115, 116, 32, 117, 115, 101, 114, 0, 0, 0, 28, 104,
            116, 116, 112, 115, 58, 47, 47, 106, 102, 107, 97, 108, 115, 100, 107, 106, 102, 106, 46, 99,
            111, 109, 47, 107, 106, 107, 106, 0, 0, 0, 22, 116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116,
            101, 115, 116, 32, 109, 101, 115, 115, 97, 103, 101,
        ];

        assert_eq!(Message::parse(&bytes).unwrap(), (133, message))
    }
}
