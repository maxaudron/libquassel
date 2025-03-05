use libquassel_derive::sync;
use libquassel_derive::{NetworkList, NetworkMap};

#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;
use crate::message::{Class, Syncable};

use crate::primitive::{BufferId, NetworkId, VariantList};
use crate::{ProtocolError, Result, SyncProxyError};

/// Configuration for the Chat List displaying our buffers
#[derive(Debug, Default, Clone, PartialEq, NetworkList, NetworkMap)]
pub struct BufferViewConfig {
    #[network(rename = "BufferList", network = "list", variant = "VariantList")]
    pub buffers: Vec<BufferId>,
    #[network(rename = "RemovedBuffers", network = "list", variant = "VariantList")]
    pub removed_buffers: Vec<BufferId>,
    #[network(
        rename = "TemporarilyRemovedBuffers",
        network = "list",
        variant = "VariantList"
    )]
    pub temporarily_removed_buffers: Vec<BufferId>,

    #[network(rename = "bufferViewId", default, skip)]
    pub buffer_view_id: i32,
    #[network(rename = "bufferViewName")]
    pub buffer_view_name: String,
    #[network(rename = "networkId")]
    pub network_id: NetworkId,
    #[network(rename = "addNewBuffersAutomatically")]
    pub add_new_buffers_automatically: bool,
    #[network(rename = "sortAlphabetically")]
    pub sort_alphabetically: bool,
    #[network(rename = "hideInactiveBuffers")]
    pub hide_inactive_buffers: bool,
    #[network(rename = "hideInactiveNetworks")]
    pub hide_inactive_networks: bool,
    #[network(rename = "disableDecoration")]
    pub disable_decoration: bool,
    // TODO use bitflags for buffertypes
    #[network(rename = "allowedBufferTypes")]
    pub allowed_buffer_types: i32,
    #[network(rename = "minimumActivity")]
    pub minimum_activity: i32,
    #[network(rename = "showSearch")]
    pub show_search: bool,
}

#[allow(dead_code)]
impl BufferViewConfig {
    pub fn request_add_buffer(&self, id: BufferId, pos: usize) -> Result<()> {
        sync!("requestAddBuffer", [id, (pos as i32)])
    }

    pub fn add_buffer(&mut self, id: BufferId, pos: usize) -> Result<()> {
        if !self.buffers.contains(&id) {
            self.buffers.insert(pos, id)
        }

        if let Some(old_pos) = self.removed_buffers.iter().position(|&x| x == id) {
            self.removed_buffers.remove(old_pos);
        }

        if let Some(old_pos) = self.temporarily_removed_buffers.iter().position(|&x| x == id) {
            self.temporarily_removed_buffers.remove(old_pos);
        }

        #[cfg(feature = "server")]
        return sync!("addBuffer", [id, (pos as i32)]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn request_move_buffer(&self, id: BufferId, pos: usize) -> Result<()> {
        sync!("requestMoveBuffer", [id, (pos as i32)])
    }

    pub fn move_buffer(&mut self, id: BufferId, pos: usize) -> Result<()> {
        let old_pos = self.buffers.iter().position(|&x| x == id).ok_or(ProtocolError::BufferNotFound(id))?;
        self.buffers.remove(old_pos);
        self.buffers.insert(pos, id);

        #[cfg(feature = "server")]
        return sync!("moveBuffer", [id, (pos as i32)]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn request_remove_buffer(&mut self, id: BufferId) -> Result<()> {
        sync!("requestRemoveBuffer", [id])
    }

    pub fn remove_buffer(&mut self, id: BufferId) -> Result<()> {
        if let Some(old_pos) = self.buffers.iter().position(|&x| x == id) {
            self.buffers.remove(old_pos);
        }

        if let Some(old_pos) = self.removed_buffers.iter().position(|&x| x == id) {
            self.removed_buffers.remove(old_pos);
        }

        if !self.temporarily_removed_buffers.contains(&id) {
            self.temporarily_removed_buffers.push(id)
        }

        #[cfg(feature = "server")]
        return sync!("removeBuffer", [id]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn request_remove_buffer_permanently(&mut self, id: BufferId) -> Result<()> {
        sync!("requestRemoveBufferPermanently", [id])
    }

    pub fn remove_buffer_permanently(&mut self, id: BufferId) -> Result<()> {
        if let Some(old_pos) = self.buffers.iter().position(|&x| x == id) {
            self.buffers.remove(old_pos);
        }

        if let Some(old_pos) = self.temporarily_removed_buffers.iter().position(|&x| x == id) {
            self.temporarily_removed_buffers.remove(old_pos);
        }

        if !self.removed_buffers.contains(&id) {
            self.removed_buffers.push(id)
        }

        #[cfg(feature = "server")]
        return sync!("removeBufferPermanently", [id]);

        #[cfg(feature = "client")]
        return Ok(());
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for BufferViewConfig {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        log::debug!("entering bufferviewconfig sync_custom()");
        match msg.slot_name.as_str() {
            "addBuffer" => self.add_buffer(
                msg.params.remove(0).try_into()?,
                i32::try_from(msg.params.remove(0))? as usize,
            ),
            "moveBuffer" => self.move_buffer(
                msg.params.remove(0).try_into()?,
                i32::try_from(msg.params.remove(0))? as usize,
            ),
            "removeBuffer" => self.remove_buffer(msg.params.remove(0).try_into()?),
            "removeBufferPermanently" => self.remove_buffer_permanently(msg.params.remove(0).try_into()?),
            unknown => Err(ProtocolError::UnknownMsgSlotName(unknown.to_string())),
        }
    }
}

#[cfg(feature = "server")]
impl StatefulSyncableServer for BufferViewConfig {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestAddBuffer" => self.add_buffer(
                msg.params.remove(0).try_into()?,
                i32::try_from(msg.params.remove(0))? as usize,
            )?,
            "requestMoveBuffer" => self.move_buffer(
                msg.params.remove(0).try_into()?,
                i32::try_from(msg.params.remove(0))? as usize,
            )?,
            "requestRemoveBuffer" => self.remove_buffer(msg.params.remove(0).try_into()?)?,
            "requestRemoveBufferPermanently" => {
                self.remove_buffer_permanently(msg.params.remove(0).try_into()?)?
            }
            "setAddNewBuffersAutomatically" => {
                self.add_new_buffers_automatically = msg.params.remove(0).try_into()?
            }
            "setAllowedBufferTypes" => self.allowed_buffer_types = msg.params.remove(0).try_into()?,
            "setBufferViewName" => self.buffer_view_name = msg.params.remove(0).try_into()?,
            "setDisableDecoration" => self.disable_decoration = msg.params.remove(0).try_into()?,
            "setHideInactiveBuffers" => self.hide_inactive_buffers = msg.params.remove(0).try_into()?,
            "setHideInactiveNetworks" => self.hide_inactive_networks = msg.params.remove(0).try_into()?,
            "setMinimumActivity" => self.minimum_activity = msg.params.remove(0).try_into()?,
            "setNetworkId" => self.network_id = msg.params.remove(0).try_into()?,
            "setShowSearch" => self.show_search = msg.params.remove(0).try_into()?,
            "setSortAlphabetically" => self.sort_alphabetically = msg.params.remove(0).try_into()?,
            unknown => Err(ProtocolError::UnknownMsgSlotName(unknown.to_string()))?,
        }

        Ok(())
    }
}

impl Syncable for BufferViewConfig {
    const CLASS: Class = Class::BufferViewConfig;

    fn send_sync(&self, function: &str, params: VariantList) -> Result<()> {
        crate::message::signalproxy::SYNC_PROXY
            .get()
            .ok_or(SyncProxyError::NotInitialized)?
            .sync(
                Self::CLASS,
                Some(&self.buffer_view_id.to_string()),
                function,
                params,
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bufferviewconfig_sample() -> BufferViewConfig {
        BufferViewConfig {
            buffers: vec![1.into(), 2.into(), 3.into()],
            removed_buffers: vec![4.into(), 5.into()],
            temporarily_removed_buffers: vec![6.into(), 7.into()],
            ..Default::default()
        }
    }

    #[test]
    fn bufferviewconfig_add_buffer() {
        // Add existing buffer, no change
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.add_buffer(1.into(), 2).unwrap();
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Add new buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.add_buffer(10.into(), 1).unwrap();
        assert_eq!(
            BufferViewConfig {
                buffers: vec![1.into(), 10.into(), 2.into(), 3.into()],
                removed_buffers: vec![4.into(), 5.into()],
                temporarily_removed_buffers: vec![6.into(), 7.into()],
                ..Default::default()
            },
            buffer_view_config
        );

        // Add new buffer, remove from removed buffers
        let mut buffer_view_config = BufferViewConfig {
            buffers: vec![1.into(), 2.into(), 3.into()],
            removed_buffers: vec![4.into(), 5.into(), 10.into()],
            temporarily_removed_buffers: vec![6.into(), 7.into(), 10.into()],
            ..Default::default()
        };
        buffer_view_config.add_buffer(10.into(), 1).unwrap();
        assert_eq!(
            BufferViewConfig {
                buffers: vec![1.into(), 10.into(), 2.into(), 3.into()],
                removed_buffers: vec![4.into(), 5.into()],
                temporarily_removed_buffers: vec![6.into(), 7.into()],
                ..Default::default()
            },
            buffer_view_config
        );
    }

    #[test]
    fn bufferviewconfig_remove_buffer() {
        // Remove already removed buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer(6.into()).unwrap();
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Remove buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer(1.into()).unwrap();
        assert_eq!(
            BufferViewConfig {
                buffers: vec![2.into(), 3.into()],
                removed_buffers: vec![4.into(), 5.into()],
                temporarily_removed_buffers: vec![6.into(), 7.into(), 1.into()],
                ..Default::default()
            },
            buffer_view_config
        );
    }

    #[test]
    fn bufferviewconfig_remove_buffer_permanently() {
        // Remove already removed buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer_permanently(4.into()).unwrap();
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Remove buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer_permanently(1.into()).unwrap();
        assert_eq!(
            BufferViewConfig {
                buffers: vec![2.into(), 3.into()],
                removed_buffers: vec![4.into(), 5.into(), 1.into()],
                temporarily_removed_buffers: vec![6.into(), 7.into()],
                ..Default::default()
            },
            buffer_view_config
        );
    }

    #[test]
    fn bufferviewconfig_move_buffer() {
        // Do nothing
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.move_buffer(1.into(), 0).unwrap();
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Move buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.move_buffer(1.into(), 1).unwrap();
        assert_eq!(
            BufferViewConfig {
                buffers: vec![2.into(), 1.into(), 3.into()],
                removed_buffers: vec![4.into(), 5.into()],
                temporarily_removed_buffers: vec![6.into(), 7.into()],
                ..Default::default()
            },
            buffer_view_config
        );
    }
}
