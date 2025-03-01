use std::{collections::HashMap, vec::Vec};

use itertools::Itertools;
use log::{error, trace};

use crate::error::ProtocolError;
use crate::message::objects::{Identity, IrcChannel, IrcUser, NetworkInfo, NetworkServer};
use crate::primitive::{self, NetworkId, PeerPtr};
use crate::primitive::{IdentityId, StringList};
use crate::serialize::*;

use crate::primitive::{BufferId, BufferInfo, Date, DateTime, Message, MsgId, Time, VariantList, VariantMap};

use libquassel_derive::From;

/// Variant represents the possible types we can receive
///
/// Variant's are serizalized as the Type as a i32 and then the Type in it's own format
///
/// BufferInfo and Message are UserTypes
/// but we represent them as a native Type here.
///
/// ByteArray is de-/serialized as a C ByteArray.
#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Debug, PartialEq, From)]
pub enum Variant {
    Unknown,
    #[from(ignore)]
    UserType(String, Vec<u8>),
    BufferId(BufferId),
    BufferInfo(BufferInfo),
    IdentityId(IdentityId),
    IrcUser(IrcUser),
    IrcChannel(IrcChannel),
    Identity(Identity),
    Message(Message),
    MsgId(MsgId),
    NetworkId(NetworkId),
    NetworkInfo(NetworkInfo),
    NetworkServer(NetworkServer),
    Time(Time),
    Date(Date),
    DateTime(DateTime),
    VariantMap(VariantMap),
    VariantList(VariantList),
    #[from(ignore)]
    String(String),
    #[from(ignore)]
    ByteArray(String),
    StringList(StringList),
    PeerPtr(PeerPtr),
    char(char),
    bool(bool),
    u64(u64),
    u32(u32),
    u16(u16),
    u8(u8),
    i64(i64),
    i32(i32),
    i16(i16),
    i8(i8),
}

impl From<Variant> for String {
    fn from(input: Variant) -> Self {
        match input {
            Variant::String(value) => value,
            Variant::ByteArray(value) => value,
            _ => panic!("unknown variant expected string or bytearray {:?}", input),
        }
    }
}

impl From<&Variant> for String {
    fn from(input: &Variant) -> Self {
        match input {
            Variant::String(value) => value.clone(),
            Variant::ByteArray(value) => value.clone(),
            _ => panic!("unknown variant expected string or bytearray {:?}", input),
        }
    }
}

impl From<String> for Variant {
    fn from(input: String) -> Self {
        Self::String(input)
    }
}

impl From<&str> for Variant {
    fn from(input: &str) -> Self {
        Self::String(input.to_owned())
    }
}

/// Implements the Network trait genericly for everything that
/// can be a [VariantList]
impl<T, S> crate::message::NetworkList for HashMap<T, S>
where
    T: std::convert::TryFrom<Variant> + Into<Variant> + Clone + std::hash::Hash + std::cmp::Eq,
    S: std::convert::TryFrom<Variant> + Into<Variant> + Clone + std::hash::Hash + std::cmp::Eq,
{
    fn to_network_list(&self) -> VariantList {
        let mut res = Vec::with_capacity(self.len() * 2);

        self.iter().for_each(|(k, v)| {
            res.push((*k).clone().into());
            res.push((*v).clone().into());
        });

        return res;
    }

    fn from_network_list(input: &mut VariantList) -> Self {
        let mut res = HashMap::with_capacity(input.len() / 2);

        input.iter().tuples().for_each(|(k, v)| {
            res.insert(
                match T::try_from(k.clone()) {
                    Ok(it) => it,
                    _ => unreachable!(),
                },
                match S::try_from(v.clone()) {
                    Ok(it) => it,
                    _ => unreachable!(),
                },
            );
        });

        return res;
    }
}

impl<S> crate::message::NetworkMap for HashMap<String, S>
where
    S: std::convert::TryFrom<Variant> + Into<Variant> + Clone + std::hash::Hash + std::cmp::Eq,
{
    type Item = super::VariantMap;

    fn to_network_map(&self) -> Self::Item {
        let mut res = HashMap::with_capacity(self.len());

        self.iter().for_each(|(k, v)| {
            res.insert(k.clone(), (*v).clone().into());
        });

        return res;
    }

    fn from_network_map(input: &mut Self::Item) -> Self {
        input
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    match S::try_from(v.clone()) {
                        Ok(it) => it,
                        _ => unreachable!(),
                    },
                )
            })
            .collect()
    }
}

impl Serialize for Variant {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        match self {
            Variant::Unknown => Err(ProtocolError::UnknownVariant),
            Variant::VariantMap(v) => v.serialize_variant(),
            Variant::VariantList(v) => v.serialize_variant(),
            Variant::char(v) => v.serialize_variant(),
            Variant::String(v) => v.serialize_variant(),
            Variant::ByteArray(v) => {
                let mut res: Vec<u8> = Vec::new();
                res.extend(primitive::QBYTEARRAY.to_be_bytes().iter());
                res.extend(0x00u8.to_be_bytes().iter());
                res.extend(v.serialize_utf8()?.iter());
                Ok(res)
            }
            Variant::StringList(v) => v.serialize_variant(),
            Variant::bool(v) => v.serialize_variant(),
            Variant::u64(v) => v.serialize_variant(),
            Variant::u32(v) => v.serialize_variant(),
            Variant::u16(v) => v.serialize_variant(),
            Variant::u8(v) => v.serialize_variant(),
            Variant::i64(v) => v.serialize_variant(),
            Variant::i32(v) => v.serialize_variant(),
            Variant::i16(v) => v.serialize_variant(),
            Variant::i8(v) => v.serialize_variant(),
            Variant::UserType(name, bytes) => {
                let mut res: Vec<u8> = Vec::new();
                res.extend(primitive::USERTYPE.to_be_bytes().iter());
                res.extend(0x00u8.to_be_bytes().iter());
                res.append(&mut name.serialize_utf8()?);
                res.extend(bytes);
                Ok(res)
            }
            Variant::BufferId(v) => v.serialize_variant(),
            Variant::BufferInfo(v) => v.serialize_variant(),
            Variant::IdentityId(v) => v.serialize_variant(),
            Variant::Message(v) => v.serialize_variant(),
            Variant::MsgId(v) => v.serialize_variant(),
            Variant::NetworkId(v) => v.serialize_variant(),
            Variant::PeerPtr(v) => v.serialize_variant(),
            Variant::DateTime(v) => v.serialize_variant(),
            Variant::Time(v) => v.serialize_variant(),
            Variant::Date(v) => v.serialize_variant(),
            Variant::IrcUser(v) => v.serialize_variant(),
            Variant::IrcChannel(v) => v.serialize_variant(),
            Variant::Identity(v) => v.serialize_variant(),
            Variant::NetworkInfo(v) => v.serialize_variant(),
            Variant::NetworkServer(v) => v.serialize_variant(),
        }
    }
}

impl Deserialize for Variant {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        trace!("trying to parse variant with bytes: {:x?}", b);
        let (_, qtype) = i32::parse(&b[0..4])?;
        let qtype = qtype as u32;

        let _unknown: u8 = b[4];

        let len = 5;
        match qtype {
            VariantMap::TYPE => return VariantMap::parse_variant(b, len),
            VariantList::TYPE => return VariantList::parse_variant(b, len),
            char::TYPE => return char::parse_variant(b, len),
            String::TYPE => String::parse_variant(b, len),
            primitive::QBYTEARRAY => {
                trace!(target: "primitive::Variant", "Parsing Variant: ByteArray");
                let (vlen, value) = String::parse_utf8(&b[len..])?;
                return Ok((len + vlen, Variant::ByteArray(value.clone())));
            }
            StringList::TYPE => StringList::parse_variant(b, len),
            DateTime::TYPE => DateTime::parse_variant(b, len),
            Date::TYPE => Date::parse_variant(b, len),
            Time::TYPE => Time::parse_variant(b, len),
            bool::TYPE => bool::parse_variant(b, len),
            u64::TYPE => u64::parse_variant(b, len),
            u32::TYPE => u32::parse_variant(b, len),
            u16::TYPE => u16::parse_variant(b, len),
            u8::TYPE => u8::parse_variant(b, len),
            i64::TYPE => i64::parse_variant(b, len),
            i32::TYPE => i32::parse_variant(b, len),
            i16::TYPE => i16::parse_variant(b, len),
            i8::TYPE => i8::parse_variant(b, len),
            primitive::USERTYPE => {
                trace!(target: "primitive::Variant", "Parsing UserType");
                // Parse UserType name
                let (user_type_len, user_type) = String::parse_utf8(&b[len..])?;

                trace!(target: "primitive::Variant", "Parsing UserType: {:?}", user_type);

                // Match Possible User Types to basic structures
                match user_type.as_str() {
                    BufferId::NAME => BufferId::parse_variant(b, len + user_type_len),
                    IrcUser::NAME => IrcUser::parse_variant(b, len + user_type_len),
                    IrcChannel::NAME => IrcChannel::parse_variant(b, len + user_type_len),
                    Identity::NAME => Identity::parse_variant(b, len + user_type_len),
                    NetworkInfo::NAME => NetworkInfo::parse_variant(b, len + user_type_len),
                    NetworkServer::NAME => NetworkServer::parse_variant(b, len + user_type_len),
                    NetworkId::NAME => NetworkId::parse_variant(b, len + user_type_len),
                    IdentityId::NAME => IdentityId::parse_variant(b, len + user_type_len),
                    PeerPtr::NAME => PeerPtr::parse_variant(b, len + user_type_len),
                    BufferInfo::NAME => BufferInfo::parse_variant(b, len + user_type_len),
                    Message::NAME => Message::parse_variant(b, len + user_type_len),
                    MsgId::NAME => MsgId::parse_variant(b, len + user_type_len),
                    _ => Err(ProtocolError::UnknownUserType(user_type)),
                }
            }
            err => {
                error!(target: "parser", "UnknownVariant: {:x?}", err);
                return Err(ProtocolError::UnknownVariant);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use time::macros::format_description;

    use super::*;

    #[test]
    fn signed_serialize() {
        let i_64 = Variant::i64(847291274197592);
        let i_32 = Variant::i32(897911521);
        let i_16 = Variant::i16(8179);
        let i_8 = Variant::i8(78);

        let i_n_64 = Variant::i64(-847291274197592);
        let i_n_32 = Variant::i32(-897911521);
        let i_n_16 = Variant::i16(-8179);
        let i_n_8 = Variant::i8(-78);

        assert_eq!(
            i_64.serialize().unwrap(),
            [0, 0, 0, 129, 0, 0, 3, 2, 155, 95, 107, 122, 88]
        );
        assert_eq!(i_32.serialize().unwrap(), [0, 0, 0, 2, 0, 53, 133, 10, 225]);
        assert_eq!(i_16.serialize().unwrap(), [0, 0, 0, 130, 0, 31, 243]);
        assert_eq!(i_8.serialize().unwrap(), [0, 0, 0, 131, 0, 78]);

        assert_eq!(
            i_n_64.serialize().unwrap(),
            [0, 0, 0, 129, 0, 255, 252, 253, 100, 160, 148, 133, 168]
        );
        assert_eq!(i_n_32.serialize().unwrap(), [0, 0, 0, 2, 0, 202, 122, 245, 31]);
        assert_eq!(i_n_16.serialize().unwrap(), [0, 0, 0, 130, 0, 224, 13]);
        assert_eq!(i_n_8.serialize().unwrap(), [0, 0, 0, 131, 0, 178]);
    }

    #[test]
    fn unsigned_serialize() {
        let u_64 = Variant::u64(847291274197592);
        let u_32 = Variant::u32(897911521);
        let u_16 = Variant::u16(8179);
        let u_8 = Variant::u8(78);

        assert_eq!(
            u_64.serialize().unwrap(),
            [0, 0, 0, 132, 0, 0, 3, 2, 155, 95, 107, 122, 88]
        );
        assert_eq!(u_32.serialize().unwrap(), [0, 0, 0, 3, 0, 53, 133, 10, 225]);
        assert_eq!(u_16.serialize().unwrap(), [0, 0, 0, 133, 0, 31, 243]);
        assert_eq!(u_8.serialize().unwrap(), [0, 0, 0, 134, 0, 78]);
    }

    #[test]
    fn variant_signed_deserialize() {
        let i_64 = Variant::i64(847291274197592);
        let i_32 = Variant::i32(897911521);
        let i_16 = Variant::i16(8179);
        let i_8 = Variant::i8(78);

        let i_n_64 = Variant::i64(-847291274197592);
        let i_n_32 = Variant::i32(-897911521);
        let i_n_16 = Variant::i16(-8179);
        let i_n_8 = Variant::i8(-78);

        let (_, v_i_64) = Variant::parse(&[0, 0, 0, 129, 0, 0, 3, 2, 155, 95, 107, 122, 88]).unwrap();
        let (_, v_i_32) = Variant::parse(&[0, 0, 0, 2, 0, 53, 133, 10, 225]).unwrap();
        let (_, v_i_16) = Variant::parse(&[0, 0, 0, 130, 0, 31, 243]).unwrap();
        let (_, v_i_8) = Variant::parse(&[0, 0, 0, 131, 0, 78]).unwrap();

        let (_, v_i_n_64) =
            Variant::parse(&[0, 0, 0, 129, 0, 255, 252, 253, 100, 160, 148, 133, 168]).unwrap();
        let (_, v_i_n_32) = Variant::parse(&[0, 0, 0, 2, 0, 202, 122, 245, 31]).unwrap();
        let (_, v_i_n_16) = Variant::parse(&[0, 0, 0, 130, 0, 224, 13]).unwrap();
        let (_, v_i_n_8) = Variant::parse(&[0, 0, 0, 131, 0, 178]).unwrap();

        assert_eq!(i_64, v_i_64);
        assert_eq!(i_32, v_i_32);
        assert_eq!(i_16, v_i_16);
        assert_eq!(i_8, v_i_8);

        assert_eq!(i_n_64, v_i_n_64);
        assert_eq!(i_n_32, v_i_n_32);
        assert_eq!(i_n_16, v_i_n_16);
        assert_eq!(i_n_8, v_i_n_8);
    }

    #[test]
    fn unsigned_deserialize() {
        let u_64 = Variant::u64(847291274197592);
        let u_32 = Variant::u32(897911521);
        let u_16 = Variant::u16(8179);
        let u_8 = Variant::u8(78);

        let (_, v_u_64) = Variant::parse(&[0, 0, 0, 132, 0, 0, 3, 2, 155, 95, 107, 122, 88]).unwrap();
        let (_, v_u_32) = Variant::parse(&[0, 0, 0, 3, 0, 53, 133, 10, 225]).unwrap();
        let (_, v_u_16) = Variant::parse(&[0, 0, 0, 133, 0, 31, 243]).unwrap();
        let (_, v_u_8) = Variant::parse(&[0, 0, 0, 134, 0, 78]).unwrap();

        assert_eq!(u_64, v_u_64);
        assert_eq!(u_32, v_u_32);
        assert_eq!(u_16, v_u_16);
        assert_eq!(u_8, v_u_8);
    }

    #[test]
    pub fn bool_serialize() {
        let test_variant_true = Variant::bool(true);
        let test_variant_false = Variant::bool(false);
        assert_eq!(test_variant_true.serialize().unwrap(), [0, 0, 0, 1, 0, 1]);
        assert_eq!(test_variant_false.serialize().unwrap(), [0, 0, 0, 1, 0, 0]);
    }

    #[test]
    pub fn bool_deserialize() {
        let test_bytes: &[u8] = &[0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
        let (len, res) = Variant::parse(test_bytes).unwrap();
        assert_eq!(len, 6);
        assert_eq!(res, Variant::bool(true));
    }

    #[test]
    pub fn variantlist_serialize() {
        let mut test_variantlist = VariantList::new();
        test_variantlist.push(Variant::bool(true));
        assert_eq!(
            test_variantlist.serialize().unwrap(),
            [0, 0, 0, 1, 0, 0, 0, 1, 0, 1]
        );
    }

    #[test]
    pub fn variantlist_deserialize() {
        let test_bytes: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
        let (len, res) = VariantList::parse(test_bytes).unwrap();
        let mut test_variantlist = VariantList::new();
        test_variantlist.push(Variant::bool(true));
        assert_eq!(len, 10);
        assert_eq!(res, test_variantlist);
    }

    #[test]
    pub fn variantmap_serialize() {
        let mut test_variantmap = VariantMap::new();
        test_variantmap.insert("Configured".to_string(), Variant::bool(true));
        let bytes = [
            0, 0, 0, 1, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101,
            0, 100, 0, 0, 0, 1, 0, 1,
        ]
        .to_vec();
        assert_eq!(test_variantmap.serialize().unwrap(), bytes);
    }

    #[test]
    pub fn variantmap_deserialize() {
        let test_bytes: &[u8] = &[
            0, 0, 0, 1, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101,
            0, 100, 0, 0, 0, 1, 0, 1,
        ];
        let (len, res) = VariantMap::parse(test_bytes).unwrap();
        let mut test_variantmap = VariantMap::new();
        test_variantmap.insert("Configured".to_string(), Variant::bool(true));
        assert_eq!(len, 34);
        assert_eq!(res, test_variantmap);
    }

    #[test]
    pub fn buffer_info_serialize() {
        let test_buffer_info = Variant::BufferInfo(BufferInfo {
            id: BufferId(1),
            network_id: 1,
            buffer_type: primitive::BufferType::Channel,
            name: "#test".to_string(),
        });

        let bytes = vec![
            0, 0, 0, 127, 0, 0, 0, 0, 10, 66, 117, 102, 102, 101, 114, 73, 110, 102, 111, 0, 0, 0, 8, 66,
            117, 102, 102, 101, 114, 73, 100, 0, 0, 0, 1, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 35, 116,
            101, 115, 116,
        ];
        assert_eq!(test_buffer_info.serialize().unwrap(), bytes);
    }

    #[test]
    pub fn buffer_info_deserialize() {
        let test_buffer_info = BufferInfo {
            id: BufferId(0),
            network_id: 0,
            buffer_type: primitive::BufferType::Status,
            name: "test".to_string(),
        };

        let bytes = vec![
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5, 0x74,
            0x65, 0x73, 0x74, 0x0,
        ];
        let (len, res) = BufferInfo::parse(&bytes).unwrap();

        assert_eq!(len, 23);
        assert_eq!(res, test_buffer_info);
    }

    #[test]
    fn char_serialize() {
        assert_eq!(Variant::char('z').serialize().unwrap(), [0, 0, 0, 7, 0, 0, 122]);
    }

    #[test]
    fn char_deserialize() {
        assert_eq!(
            (7, Variant::char('z')),
            Variant::parse(&[0, 0, 0, 7, 0, 0, 122]).unwrap()
        );
    }

    #[test]
    fn strings_serialize() {
        let test_string = "This is a Test!1!!".to_string();
        let test_string_list = vec!["test1".to_string(), "test 2".to_string()];

        assert_eq!(
            Variant::String(test_string.clone()).serialize().unwrap(),
            [
                0, 0, 0, 10, 0, 0, 0, 0, 36, 0, 0x54, 0, 0x68, 0, 0x69, 0, 0x73, 0, 0x20, 0, 0x69, 0, 0x73,
                0, 0x20, 0, 0x61, 0, 0x20, 0, 0x54, 0, 0x65, 0, 0x73, 0, 0x74, 0, 0x21, 0, 0x31, 0, 0x21, 0,
                0x21
            ]
        );
        assert_eq!(
            Variant::ByteArray(test_string.clone()).serialize().unwrap(),
            [
                0, 0, 0, 12, 0, 0, 0, 0, 18, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20,
                0x54, 0x65, 0x73, 0x74, 0x21, 0x31, 0x21, 0x21
            ]
        );
        assert_eq!(
            Variant::StringList(test_string_list).serialize().unwrap(),
            [
                0, 0, 0, 11, 0, 0, 0, 0, 2, 0, 0, 0, 10, 0, 0x74, 0, 0x65, 0, 0x73, 0, 0x74, 0, 0x31, 0, 0,
                0, 12, 0, 0x74, 0, 0x65, 0, 0x73, 0, 0x74, 0, 0x20, 0, 0x32
            ]
        );
    }

    #[test]
    fn strings_deserialize() {
        let test_string = "This is a Test!1!!".to_string();
        let test_string_list = vec!["test1".to_string(), "test 2".to_string()];

        let test_string_src = vec![
            0, 0, 0, 10, 0, 0, 0, 0, 36, 0, 0x54, 0, 0x68, 0, 0x69, 0, 0x73, 0, 0x20, 0, 0x69, 0, 0x73, 0,
            0x20, 0, 0x61, 0, 0x20, 0, 0x54, 0, 0x65, 0, 0x73, 0, 0x74, 0, 0x21, 0, 0x31, 0, 0x21, 0, 0x21,
        ];

        let test_string_src_utf8 = vec![
            0, 0, 0, 12, 0, 0, 0, 0, 18, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x54,
            0x65, 0x73, 0x74, 0x21, 0x31, 0x21, 0x21, 0,
        ];

        let test_string_list_src = vec![
            0, 0, 0, 11, 0, 0, 0, 0, 2, 0, 0, 0, 10, 0, 0x74, 0, 0x65, 0, 0x73, 0, 0x74, 0, 0x31, 0, 0, 0,
            12, 0, 0x74, 0, 0x65, 0, 0x73, 0, 0x74, 0, 0x20, 0, 0x32,
        ];

        assert_eq!(
            (45, Variant::String(test_string.clone())),
            Variant::parse(&test_string_src).unwrap()
        );
        assert_eq!(
            (27, Variant::ByteArray(test_string.clone())),
            Variant::parse(&test_string_src_utf8).unwrap()
        );
        assert_eq!(
            (39, Variant::StringList(test_string_list)),
            Variant::parse(&test_string_list_src).unwrap()
        );
    }

    #[test]
    fn datetime_serialize() {
        let datetime = Variant::DateTime(
            DateTime::parse(
                "2020-02-19 13:00 +0200",
                format_description!(
                    "[year]-[month]-[day] [hour]:[minute] [offset_hour sign:mandatory][offset_minute]"
                ),
            )
            .unwrap(),
        );

        let date =
            Variant::Date(Date::parse("2020-02-19", format_description!("[year]-[month]-[day]")).unwrap());
        let time = Variant::Time(Time::parse("13:00", format_description!("[hour]:[minute]")).unwrap());

        assert_eq!(
            datetime.serialize().unwrap(),
            [0, 0, 0, 0x10, 0, 0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32]
        );

        assert_eq!(date.serialize().unwrap(), [0, 0, 0, 0x0e, 0, 0, 37, 133, 19]);

        assert_eq!(time.serialize().unwrap(), [0, 0, 0, 0x0f, 0, 2, 202, 28, 128]);
    }

    #[test]
    fn datetime_deserialize() {
        let datetime = Variant::DateTime(
            DateTime::parse(
                "2020-02-19 13:00 +0200",
                format_description!(
                    "[year]-[month]-[day] [hour]:[minute] [offset_hour sign:mandatory][offset_minute]"
                ),
            )
            .unwrap(),
        );

        let date =
            Variant::Date(Date::parse("2020-02-19", format_description!("[year]-[month]-[day]")).unwrap());
        let time = Variant::Time(Time::parse("13:00", format_description!("[hour]:[minute]")).unwrap());

        assert_eq!(
            (18, datetime),
            Variant::parse(&[0, 0, 0, 0x10, 0, 0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32]).unwrap()
        );

        assert_eq!(
            (9, date),
            Variant::parse(&[0, 0, 0, 0x0e, 0, 0, 37, 133, 19]).unwrap()
        );

        assert_eq!(
            (9, time),
            Variant::parse(&[0, 0, 0, 0x0f, 0, 2, 202, 28, 128]).unwrap()
        );
    }

    #[test]
    fn msgid_serialize() {
        let test_msg_id = Variant::MsgId(MsgId(1));

        assert_eq!(
            test_msg_id.serialize().unwrap(),
            [0, 0, 0, 127, 0, 0, 0, 0, 5, 77, 115, 103, 73, 100, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn msgid_deserialize() {
        let test_bytes = vec![
            0, 0, 0, 127, 0, 0, 0, 0, 5, 77, 115, 103, 73, 100, 0, 0, 0, 0, 0, 0, 0, 1,
        ];

        assert_eq!(
            (test_bytes.len(), Variant::MsgId(MsgId(1))),
            Variant::parse(&test_bytes).unwrap()
        );
    }

    #[test]
    fn bufferid_serialize() {
        let test_buffer_id = Variant::BufferId(BufferId(1));
        assert_eq!(
            test_buffer_id.serialize().unwrap(),
            [0, 0, 0, 127, 0, 0, 0, 0, 8, 66, 117, 102, 102, 101, 114, 73, 100, 0, 0, 0, 1]
        );
    }

    #[test]
    fn bufferid_deserialize() {
        let test_bytes = vec![
            0, 0, 0, 127, 0, 0, 0, 0, 8, 66, 117, 102, 102, 101, 114, 73, 100, 0, 0, 0, 1,
        ];
        assert_eq!(
            (test_bytes.len(), Variant::BufferId(BufferId(1))),
            Variant::parse(&test_bytes).unwrap()
        );
    }
}
