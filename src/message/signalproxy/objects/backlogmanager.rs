// interface BacklogManager {
//
//
//     // C->S calls
//
//     /**
//      * Loads backlog for `bufferId`, starting at message `first`, up to `last`
//      * (plus `additional` more messages after `last`) but at most `limit`
//      * messages total.
//      */
//     requestBacklog(bufferId: BufferId, first: MsgId, last: MsgId, limit: Int, additional: Int)
//     /**
//      * Same as `requestBacklog`, but only messages of a certain message `type`
//      * with certain `flags` set.
//      */
//     requestBacklogFiltered(bufferId: BufferId, first: MsgId, last: MsgId, limit: Int, additional: Int, type: Int, flags: Int)
//     /**
//      * Same as `requestBacklog`, but applied to all buffers.
//      */
//     requestBacklogAll(first: MsgId, last: MsgId, limit: Int, additional: Int)
//     /**
//      * Same as `requestBacklogFiltered`, but applied to all buffers.
//      */
//     requestBacklogAllFiltered(first: MsgId, last: MsgId, limit: Int, additional: Int, type: Int, flags: Int)
//
//
//     // S->C calls
//
//     /**
//      * The response to `requestBacklog`, with the messages encoded as QVariants
//      * in the `messages` parameter.
//      */
//     receiveBacklog(bufferId: BufferId, first: MsgId, last: MsgId, limit: Int, additional: Int, messages: QVariantList)
//     /**
//      * The response to `requestBacklogFiltered`, with the messages encoded as
//      * QVariants in the `messages` parameter.
//      */
//     receiveBacklogFiltered(bufferId: BufferId, first: MsgId, last: MsgId, limit: Int, additional: Int, type: Int, flags: Int, messages: QVariantList)
//     /**
//      * The response to `requestBacklogAll`, with the messages encoded as
//      * QVariants in the `messages` parameter.
//      */
//     receiveBacklogAll(first: MsgId, last: MsgId, limit: Int, additional: Int, messages: QVariantList)
//     /**
//      * The response to `requestBacklogAllFiltered`, with the messages encoded as
//      * QVariants in the `messages` parameter.
//      */
//     receiveBacklogAllFiltered(first: MsgId, last: MsgId, limit: Int, additional: Int, type: Int, flags: Int, messages: QVariantList)
// }

#![allow(unused_variables, unused_imports)]
#![allow(non_snake_case, dead_code)]

use libquassel_derive::{sync, NetworkMap};

use crate::message::{Class, StatefulSyncableClient, Syncable};
use crate::primitive::{BufferId, MessageType, MsgId, VariantList};

/// Receive and Request Backlog
/// All "request" functions are Client to Server and all "receive" functions are Server to Client
#[derive(Clone, Debug, std::cmp::PartialEq, Default, NetworkMap)]
pub struct BacklogManager {}

#[allow(non_snake_case)]
impl BacklogManager {
    /// Loads backlog for `bufferId`, starting at message `first`, up to `last`
    /// (plus `additional` more messages after `last`) but at most `limit`
    /// messages total.
    pub fn requestBacklog(
        &self,
        buffer_id: BufferId,
        first: MsgId,
        last: MsgId,
        limit: i32,
        additional: i32,
    ) {
        sync!(
            "requestBacklog",
            [buffer_id, first, last, limit, additional]
        );
    }

    /// Same as `requestBacklog`, but only messages of a certain message `type`
    /// with certain `flags` set.
    pub fn requestBacklogFiltered(
        &self,
        buffer_id: BufferId,
        first: MsgId,
        last: MsgId,
        limit: i32,
        additional: i32,
        msgtype: MessageType,
        flags: i32,
    ) {
        sync!(
            "requestBacklogFiltered",
            [
                buffer_id,
                first,
                last,
                limit,
                additional,
                msgtype.bits(),
                flags
            ]
        );
    }

    /// Same as `requestBacklog`, but applied to all buffers.
    pub fn requestBacklogAll(&self, first: MsgId, last: MsgId, limit: i32, additional: i32) {
        sync!("requestBacklogAll", [first, last, limit, additional]);
    }

    /// Same as `requestBacklogFiltered`, but applied to all buffers.
    pub fn requestBacklogAllFiltered(
        &self,
        first: MsgId,
        last: MsgId,
        limit: i32,
        additional: i32,
        msgtype: MessageType,
        flags: i32,
    ) {
        sync!(
            "requestBacklogAllFiltered",
            [first, last, limit, additional, msgtype.bits(), flags]
        );
    }

    /// The response to `requestBacklog`, with the messages encoded as Variants
    /// in the `messages` parameter.
    pub fn receiveBacklog(
        &self,
        buffer_id: BufferId,
        first: MsgId,
        last: MsgId,
        limit: i32,
        additional: i32,
        messages: VariantList,
    ) {
        sync!(
            "receiveBacklog",
            [buffer_id, first, last, limit, additional, messages]
        );
    }

    /// The response to `requestBacklogFiltered`, with the messages encoded as
    /// Variants in the `messages` parameter.
    pub fn receiveBacklogFiltered(
        &self,
        buffer_id: BufferId,
        first: MsgId,
        last: MsgId,
        limit: i32,
        additional: i32,
        msgtype: MessageType,
        flags: i32,
        messages: VariantList,
    ) {
        sync!(
            "receiveBacklogFiltered",
            [buffer_id, first, last, limit, additional, msgtype.bits(), flags, messages]
        );
    }

    /// Same as `receiveBacklog`, but applied to all buffers.
    pub fn receiveBacklogAll(
        &self,
        first: MsgId,
        last: MsgId,
        limit: i32,
        additional: i32,
        messages: VariantList,
    ) {
        sync!(
            "receiveBacklogAll",
            [first, last, limit, additional, messages]
        );
    }

    /// Same as `receiveBacklogFiltered`, but applied to all buffers.
    pub fn receiveBacklogAllFiltered(
        &self,
        first: MsgId,
        last: MsgId,
        limit: i32,
        additional: i32,
        msgtype: MessageType,
        flags: i32,
        messages: VariantList,
    ) {
        sync!(
            "receiveBacklogAllFiltered",
            [first, last, limit, additional, msgtype.bits(), flags, messages]
        );
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for BacklogManager {}

#[cfg(feature = "server")]
impl StatefulSyncableServer for BacklogManager {}

impl Syncable for BacklogManager {
    const CLASS: Class = Class::BacklogManager;
}
