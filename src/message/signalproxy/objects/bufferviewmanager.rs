use std::collections::HashMap;

use libquassel_derive::sync;

use crate::message::Class;
#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;
use crate::message::{NetworkMap, Syncable};

use crate::primitive::{Variant, VariantList, VariantMap};
use crate::Result;

use super::BufferViewConfig;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BufferViewManager {
    pub buffer_view_configs: HashMap<i32, Option<BufferViewConfig>>,
}

// TODO initialize the BufferViewConfigs from somewhere
// TODO add buffer view configs, where does the data come from?
impl BufferViewManager {
    pub fn request_create_buffer_view(&self, properties: BufferViewConfig) -> Result<()> {
        sync!("requestCreateBufferView", [properties.to_network_map()])
    }

    pub fn request_create_buffer_views(&self, properties: &[BufferViewConfig]) -> Result<()> {
        self.send_sync(
            "requestCreateBufferViews",
            properties
                .iter()
                .map(|view| view.to_network_map().into())
                .collect(),
        )
    }

    pub fn request_delete_buffer_view(&self, id: i32) -> Result<()> {
        sync!("requestDeleteBufferView", [id])
    }

    pub fn request_delete_buffer_views(&self, ids: &[i32]) -> Result<()> {
        self.send_sync(
            "requestCreateBufferViews",
            ids.iter().map(|id| (*id).into()).collect(),
        )
    }

    #[cfg(feature = "client")]
    #[allow(unused_variables)]
    pub fn add_buffer_view_config(&mut self, id: i32) -> Result<()> {
        // TODO init!("BufferViewConfig", id);
        Ok(())
    }

    #[cfg(feature = "server")]
    pub fn add_buffer_view_config(&mut self, config: BufferViewConfig) -> Result<()> {
        self.buffer_view_configs.insert(0, Some(config));

        sync!("addBufferViewConfig", [0])
    }

    pub fn delete_buffer_view_config(&mut self, id: i32) -> Result<()> {
        if self.buffer_view_configs.contains_key(&id) {
            self.buffer_view_configs.remove(&id);
        }

        #[cfg(feature = "server")]
        return sync!("deleteBufferViewConfig", [id]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn init_buffer_view_config(&mut self, config: BufferViewConfig) {
        if let Some(stored) = self.buffer_view_configs.get_mut(&config.buffer_view_id) {
            *stored = Some(config);
        } else {
            self.buffer_view_configs
                .insert(config.buffer_view_id, Some(config));
        }
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for BufferViewManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "addBufferViewConfig" | "newBufferViewConfig" => {
                self.add_buffer_view_config(msg.params.remove(0).try_into()?)
            }
            "deleteBufferViewConfig" => self.delete_buffer_view_config(msg.params.remove(0).try_into()?),
            _ => Ok(()),
        }
    }
}

#[cfg(feature = "server")]
impl StatefulSyncableServer for BufferViewManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestCreateBufferView" => self.add_buffer_view_config(BufferViewConfig::from_network_map(
                &mut msg.params.remove(0).try_into()?,
            ))?,
            "requestCreateBufferViews" => {
                let views: VariantList = msg.params.remove(0).try_into()?;
                for view in views.into_iter() {
                    self.add_buffer_view_config(BufferViewConfig::from_network_map(&mut view.try_into()?))?
                }
            }
            "requestDeleteBufferView" => self.delete_buffer_view_config(msg.params.remove(0).try_into()?)?,
            "requestDeleteBufferViews" => {
                let ids: VariantList = msg.params.remove(0).try_into()?;
                for id in ids.into_iter() {
                    self.delete_buffer_view_config(id.try_into()?)?
                }
            }
            _ => (),
        }
        Ok(())
    }
}

impl Syncable for BufferViewManager {
    const CLASS: Class = Class::BufferViewManager;
}

impl super::NetworkList for BufferViewManager {
    fn to_network_list(&self) -> Result<VariantList> {
        Ok(vec![
            Variant::ByteArray(s!("bufferViewIds")),
            Variant::VariantList(self.buffer_view_configs.keys().map(|k| i32::into(*k)).collect()),
        ])
    }

    fn from_network_list(input: &mut VariantList) -> Result<Self> {
        let mut i = input.iter();
        i.position(|x| *x == Variant::ByteArray(String::from("BufferViewIds")))
            .expect("failed to get field BufferViewIds");

        let ids = match i.next().expect("failed to get next field") {
            libquassel::primitive::Variant::VariantList(var) => var.clone(),
            _ => panic!("network::list::from: wrong variant type"),
        };

        // TODO Somehow do the initrequests for all the IDs we get here
        Ok(Self {
            buffer_view_configs: ids
                .into_iter()
                .map(|id| (i32::try_from(id).unwrap(), Option::None))
                .collect(),
        })
    }
}

impl super::NetworkMap for BufferViewManager {
    type Item = VariantMap;

    fn to_network_map(&self) -> Self::Item {
        let mut res = VariantMap::new();

        res.insert(
            s!("bufferViewIds"),
            Variant::VariantList(self.buffer_view_configs.keys().map(|k| i32::into(*k)).collect()),
        );

        res
    }

    fn from_network_map(_input: &mut Self::Item) -> Self {
        // TODO Somehow do the initrequests for all the IDs we get here
        Self {
            buffer_view_configs: HashMap::new(),
        }
    }
}
