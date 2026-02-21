use crate::primitive::{BufferInfo, Variant};

use super::{Direction, RpcCallType};

#[derive(Clone, Debug, PartialEq)]
pub struct BufferInfoUpdated {
    buffer: BufferInfo,
}

impl RpcCallType for BufferInfoUpdated {
    const NAME: &str = "2bufferInfoUpdated(BufferInfo)";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.buffer.clone().into(),
        ])
    }

    fn from_network(
        size: usize,
        input: &mut crate::primitive::VariantList,
    ) -> Result<(usize, super::RpcCall), crate::ProtocolError>
    where
        Self: Sized,
    {
        Ok((
            size,
            Self {
                buffer: input.remove(0).try_into().unwrap(),
            }
            .into(),
        ))
    }
}
