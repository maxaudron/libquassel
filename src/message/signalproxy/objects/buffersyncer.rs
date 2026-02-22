use std::collections::HashMap;

use crate::{
    message::{Class, Syncable},
    primitive::{BufferId, MessageType, MsgId},
    ProtocolError, Result,
};

use libquassel_derive::{sync, NetworkList, NetworkMap};

#[derive(Default, Debug, Clone, PartialEq, NetworkList, NetworkMap)]
pub struct BufferSyncer {
    #[network(rename = "Activities", network = "list", variant = "VariantList")]
    pub activities: HashMap<BufferId, MessageType>,
    #[network(rename = "HighlightCounts", network = "list", variant = "VariantList")]
    pub highlight_counts: HashMap<BufferId, i32>,
    #[network(rename = "LastSeenMsg", network = "list", variant = "VariantList")]
    pub last_seen_msg: HashMap<BufferId, MsgId>,
    #[network(rename = "MarkerLines", network = "list", variant = "VariantList")]
    pub marker_line: HashMap<BufferId, MsgId>,
}

impl BufferSyncer {
    pub fn request_mark_buffer_as_read(&mut self, id: i32) -> Result<()> {
        sync!("requestMarkBufferAsRead", [id])
    }

    pub fn request_merge_buffers_permanently(&self, src_id: i32, target_id: i32) -> Result<()> {
        sync!("requestMergeBuffersPermanently", [src_id, target_id])
    }

    pub fn request_purge_buffer_ids(&self) -> Result<()> {
        sync!("requestPurgeBufferIds", [])
    }

    pub fn request_remove_buffer(&self, id: i32) -> Result<()> {
        sync!("requestRemoveBuffer", [id])
    }

    pub fn request_rename_buffer(&self, id: i32) -> Result<()> {
        sync!("requestRenameBuffer", [id])
    }

    pub fn request_set_last_seen_msg(&self, id: i32, msgid: i32) -> Result<()> {
        sync!("requestSetLastSeenMsg", [id, msgid])
    }

    pub fn request_set_marker_line(&self, id: i32, msgid: i32) -> Result<()> {
        sync!("requestSetMarkerLine", [id, msgid])
    }

    // // S->C calls

    pub fn mark_buffer_as_read(&mut self, id: BufferId) -> Result<()> {
        self.set_buffer_activity(id, MessageType::NONE)?;
        self.set_highlight_count(id, 0)?;

        #[cfg(feature = "server")]
        return sync!("markBufferAsRead", [id]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn merge_buffers_permanently(&mut self, target: BufferId, source: BufferId) -> Result<()> {
        if let Some(activities) = self.activities.remove(&source) {
            *self.activities.entry(target).or_insert(MessageType::NONE) |= activities;
        }

        if let Some(highlight_counts) = self.highlight_counts.remove(&source) {
            *self.highlight_counts.entry(target).or_default() += highlight_counts;
        }

        if let Some(last_seen_msg) = self.last_seen_msg.remove(&source) {
            let target = self.last_seen_msg.entry(target).or_default();
            if *target < last_seen_msg {
                *target = last_seen_msg
            };
        }

        if let Some(marker_line) = self.marker_line.remove(&source) {
            let target = self.marker_line.entry(target).or_default();
            if *target < marker_line {
                *target = marker_line
            };
        }

        #[cfg(feature = "server")]
        return sync!("mergeBuffersPermanently", [source, target]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    // TODO remove buffer from bufferviews
    pub fn remove_buffer(&mut self, id: BufferId) -> Result<()> {
        self.activities.remove(&id);
        self.highlight_counts.remove(&id);
        self.last_seen_msg.remove(&id);
        self.marker_line.remove(&id);

        #[cfg(feature = "server")]
        return sync!("removeBuffer", [id]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    // TODO actually rename the buffer in whereever we should store buffers
    // and the BufferView
    #[allow(unused_variables)]
    pub fn rename_buffer(&mut self, id: i32, name: String) -> Result<()> {
        #[cfg(feature = "server")]
        return sync!("renameBuffer", [id, name]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn set_buffer_activity(&mut self, id: BufferId, activity: MessageType) -> Result<()> {
        *self.activities.entry(id).or_insert(MessageType::NONE) = activity;

        #[cfg(feature = "server")]
        return sync!("setBufferActivity", [id, activity.bits()]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn set_highlight_count(&mut self, id: BufferId, count: i32) -> Result<()> {
        *self.highlight_counts.entry(id).or_default() = count;

        #[cfg(feature = "server")]
        return sync!("setHighlightCount", [id, count]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn set_last_seen_msg(&mut self, id: BufferId, msg_id: MsgId) -> Result<()> {
        *self.last_seen_msg.entry(id).or_default() = msg_id;

        #[cfg(feature = "server")]
        return sync!("setHighlightCount", [id, msg_id]);

        #[cfg(feature = "client")]
        return Ok(());
    }

    pub fn set_marker_line(&mut self, id: BufferId, msg_id: MsgId) -> Result<()> {
        *self.marker_line.entry(id).or_default() = msg_id;

        #[cfg(feature = "server")]
        return sync!("setHighlightCount", [id, msg_id]);

        #[cfg(feature = "client")]
        return Ok(());
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for BufferSyncer {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "markBufferAsRead" => self.mark_buffer_as_read(get_param!(msg)),
            "mergeBuffersPermanently" => self.merge_buffers_permanently(get_param!(msg), get_param!(msg)),
            "removeBuffer" => self.remove_buffer(get_param!(msg)),
            "renameBuffer" => self.rename_buffer(get_param!(msg), get_param!(msg)),
            "setBufferActivity" => self.set_buffer_activity(
                get_param!(msg),
                MessageType::from_bits(get_param!(msg)).unwrap_or(MessageType::NONE),
            ),
            "setHighlightCount" => self.set_highlight_count(get_param!(msg), get_param!(msg)),
            "setLastSeenMsg" => self.set_last_seen_msg(get_param!(msg), get_param!(msg)),
            "setMarkerLine" => self.set_marker_line(get_param!(msg), get_param!(msg)),
            unknown => Err(ProtocolError::UnknownMsgSlotName(unknown.to_string())),
        }
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for BufferSyncer {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestMarkBufferAsRead" => self.mark_buffer_as_read(get_param!(msg)),
            "requestMergeBuffersPermanently" => {
                self.merge_buffers_permanently(get_param!(msg), get_param!(msg))
            }
            "requestPurgeBufferIds" => Ok(()),
            "requestRemoveBuffer" => self.remove_buffer(get_param!(msg)),
            "requestRenameBuffer" => self.rename_buffer(get_param!(msg), get_param!(msg)),
            "requestSetLastSeenMsg" => self.set_last_seen_msg(get_param!(msg), get_param!(msg)),
            "requestSetMarkerLine" => self.set_marker_line(get_param!(msg), get_param!(msg)),
            unknown => Err(ProtocolError::UnknownMsgSlotName(unknown.to_string())),
        }
    }
}

impl Syncable for BufferSyncer {
    const CLASS: Class = Class::BufferSyncer;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::NetworkList;
    use crate::primitive::{Variant, VariantList};
    use pretty_assertions::assert_eq;

    fn get_network() -> VariantList {
        vec![
            Variant::ByteArray(s!("Activities")),
            Variant::VariantList(vec![
                Variant::BufferId(BufferId(1)),
                Variant::i32(0),
                Variant::BufferId(BufferId(2)),
                Variant::i32(0),
                Variant::BufferId(BufferId(3)),
                Variant::i32(0),
                Variant::BufferId(BufferId(4)),
                Variant::i32(0),
                Variant::BufferId(BufferId(5)),
                Variant::i32(0),
            ]),
            Variant::ByteArray(s!("HighlightCounts")),
            Variant::VariantList(vec![
                Variant::BufferId(BufferId(1)),
                Variant::i32(0),
                Variant::BufferId(BufferId(2)),
                Variant::i32(0),
                Variant::BufferId(BufferId(3)),
                Variant::i32(0),
                Variant::BufferId(BufferId(4)),
                Variant::i32(0),
                Variant::BufferId(BufferId(5)),
                Variant::i32(0),
            ]),
            Variant::ByteArray(s!("LastSeenMsg")),
            Variant::VariantList(vec![
                Variant::BufferId(BufferId(1)),
                Variant::MsgId(MsgId(2185)),
                Variant::BufferId(BufferId(2)),
                Variant::MsgId(MsgId(2188)),
                Variant::BufferId(BufferId(3)),
                Variant::MsgId(MsgId(860)),
                Variant::BufferId(BufferId(4)),
                Variant::MsgId(MsgId(2183)),
                Variant::BufferId(BufferId(5)),
                Variant::MsgId(MsgId(2180)),
            ]),
            Variant::ByteArray(s!("MarkerLines")),
            Variant::VariantList(vec![
                Variant::BufferId(BufferId(1)),
                Variant::MsgId(MsgId(2185)),
                Variant::BufferId(BufferId(2)),
                Variant::MsgId(MsgId(2188)),
                Variant::BufferId(BufferId(3)),
                Variant::MsgId(MsgId(860)),
                Variant::BufferId(BufferId(4)),
                Variant::MsgId(MsgId(1527)),
                Variant::BufferId(BufferId(5)),
                Variant::MsgId(MsgId(2180)),
            ]),
        ]
    }

    fn get_runtime() -> BufferSyncer {
        BufferSyncer {
            activities: map! {
                BufferId(1) => MessageType::NONE,
                BufferId(2) => MessageType::NONE,
                BufferId(3) => MessageType::NONE,
                BufferId(4) => MessageType::NONE,
                BufferId(5) => MessageType::NONE,
            },
            highlight_counts: map! {
                BufferId(1) => 0,
                BufferId(2) => 0,
                BufferId(3) => 0,
                BufferId(4) => 0,
                BufferId(5) => 0,
            },
            last_seen_msg: map! {
                BufferId(1) => MsgId(2185),
                BufferId(2) => MsgId(2188),
                BufferId(3) => MsgId(860),
                BufferId(4) => MsgId(2183),
                BufferId(5) => MsgId(2180),
            },
            marker_line: map! {
                BufferId(1) => MsgId(2185),
                BufferId(2) => MsgId(2188),
                BufferId(3) => MsgId(860),
                BufferId(4) => MsgId(1527),
                BufferId(5) => MsgId(2180),
            },
        }
    }

    // Disabled cus not sorted
    // #[test]
    // fn buffersyncer_to_network() {
    //     assert_eq!(get_runtime().to_network(), get_network())
    // }

    #[test]
    fn buffersyncer_from_network() {
        assert_eq!(
            BufferSyncer::from_network_list(get_network()).unwrap(),
            get_runtime()
        )
    }
}
