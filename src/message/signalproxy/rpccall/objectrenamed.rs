use crate::primitive::Variant;

use super::{Direction, RpcCallType};

/// Called whenever an object has been renamed, and the object store should update its name. All future sync calls for this object will use the new name instead.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectRenamed {
    classname: String,
    newname: String,
    oldname: String,
}

impl RpcCallType for ObjectRenamed {
    const NAME: &str = "__objectRenamed__";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            Variant::ByteArray(self.classname.clone()),
            self.newname.clone().into(),
            self.oldname.clone().into(),
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
                classname: input.remove(0).try_into()?,
                oldname: input.remove(0).try_into()?,
                newname: input.remove(0).try_into()?,
            }
            .into(),
        ))
    }
}
