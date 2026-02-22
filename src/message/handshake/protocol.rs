use crate::{
    serialize::{Deserialize, Serialize},
    ProtocolError,
};

#[derive(Debug, Default)]
pub enum Protocol {
    Legacy = 0x00000001,
    #[default]
    Datastream = 0x00000002,
}

impl Protocol {
    pub fn new() -> Self {
        Protocol::default()
    }

    pub fn serialize(self) -> Result<Vec<u8>, ProtocolError> {
        let proto: u32 = 0x80000002;

        proto.serialize()
    }

    pub fn parse(buf: &[u8]) -> Result<Self, ProtocolError> {
        let mut protolist: Vec<u32> = Vec::new();
        let mut pos = 0;
        loop {
            let (_, proto) = u32::parse(&buf[pos..(pos + 4)])?;
            if (proto & 0x80000000) >= 1 {
                protolist.push(proto - 0x80000000);
                break;
            } else {
                protolist.push(proto);
                pos += 4;
            }
        }

        Ok(Protocol::Datastream)
    }
}
