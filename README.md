Native rust implementation of the Quassel protocol and library functions

# Features

| Feature              | Description                                                                                                            |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| client               | Enable client side trait implementations                                                                               |
| server               | Enable server side trait implementations                                                                               |
| framing              | Enable support for tokio\'s [codec::Framed](url:https://docs.rs/tokio-util/latest/tokio_util/codec/struct.Framed.html) |
| all-quassel-features | enable all protocol features                                                                                           |
| long-message-id      | Serialize message IDs as i64                                                                                           |
| long-time            | Serialize Message Time as i64                                                                                          |
| rich-messages        | add avatar url and real name to messages                                                                               |
| sender-prefixes      | Show prefixes for senders in backlog                                                                                   |
| authenticators       | Support for exchangeable auth backends                                                                                 |
| bench                | Enable the test crate/feature for running benchmarks                                                                   |

# TODOs

- [TODO] Implementation of Syncable Objects {#implementation-of-syncable-objects}
  - [DONE] AliasManager
  - [TODO] BacklogManager
  - [DONE] BufferSyncer
  - [DONE] BufferViewConfig
  - [DONE] BufferViewManager
  - [DONE] CertManager
  - [DONE] CoreInfo
  - [DONE] HighlightRuleManager
  - [DONE] Identity
  - [DONE] IgnoreListManager
  - [DONE] IrcChannel
  - [TODO] IrcListHelper
  - [TODO] IrcUser
  - [TODO] Network
  - [TODO] NetworkInfo
  - [TODO] NetworkConfig
