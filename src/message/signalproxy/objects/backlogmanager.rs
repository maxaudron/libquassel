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

use crate::primitive::VariantList;

/// Receive and Request Backlog
/// All "request" functions are Client to Server and all "receive" functions are Server to Client
#[derive(Clone, Debug, std::cmp::PartialEq, Default)]
pub struct BacklogManager {}

impl BacklogManager {
    /// Loads backlog for `bufferId`, starting at message `first`, up to `last`
    /// (plus `additional` more messages after `last`) but at most `limit`
    /// messages total.
    fn requestBacklog(self: Self, buffer_id: u32, first: u32, last: u32, limit: u32, additional: u32) {
        unimplemented!()
    }

    /// Same as `requestBacklog`, but only messages of a certain message `type`
    /// with certain `flags` set.
    fn requestBacklogFiltered(
        self: Self,
        buffer_id: u32,
        first: u32,
        last: u32,
        limit: u32,
        additional: u32,
        msgtype: u32,
        flags: u32,
    ) {
        unimplemented!()
    }

    /// Same as `requestBacklog`, but applied to all buffers.
    fn requestBacklogAll(self: Self, first: u32, last: u32, limit: u32, additional: u32) {
        unimplemented!()
    }

    /// Same as `requestBacklogFiltered`, but applied to all buffers.
    fn requestBacklogAllFiltered(
        self: Self,
        first: u32,
        last: u32,
        limit: u32,
        additional: u32,
        msgtype: u32,
        flags: u32,
    ) {
        unimplemented!()
    }

    /// The response to `requestBacklog`, with the messages encoded as Variants
    /// in the `messages` parameter.
    fn receiveBacklog(
        self: Self,
        buffer_id: u32,
        first: u32,
        last: u32,
        limit: u32,
        additional: u32,
        messages: VariantList,
    ) {
        unimplemented!()
    }

    /// The response to `requestBacklogFiltered`, with the messages encoded as
    /// Variants in the `messages` parameter.
    fn receiveBacklogFiltered(
        self: Self,
        buffer_id: u32,
        first: u32,
        last: u32,
        limit: u32,
        additional: u32,
        msgtype: u32,
        flags: u32,
        messages: VariantList,
    ) {
        unimplemented!()
    }

    /// Same as `receiveBacklog`, but applied to all buffers.
    fn receiveBacklogAll(
        self: Self,
        first: u32,
        last: u32,
        limit: u32,
        additional: u32,
        messages: VariantList,
    ) {
        unimplemented!()
    }

    /// Same as `receiveBacklogFiltered`, but applied to all buffers.
    fn receiveBacklogAllFiltered(
        self: Self,
        first: u32,
        last: u32,
        limit: u32,
        additional: u32,
        msgtype: u32,
        flags: u32,
        messages: VariantList,
    ) {
        unimplemented!()
    }
}
