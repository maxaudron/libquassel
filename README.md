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

## Road to 1.0

- [ ] Implementation of Syncable Objects
  - [X] AliasManager
  - [ ] BacklogManager
  - [X] BufferSyncer
  - [X] BufferViewConfig
  - [X] BufferViewManager
  - [X] CertManager
  - [X] CoreInfo
  - [X] HighlightRuleManager
  - [X] Identity
  - [X] IgnoreListManager
  - [X] IrcChannel
  - [ ] IrcListHelper
  - [X] IrcUser
  - [X] Network
  - [X] NetworkInfo
  - [X] NetworkConfig
- [X] Implement RPC Calls
  - [X] RPC Call Objects
  - [X] Serialization
- [ ] Rework Error handling to actually handle errors
- [ ] Rework Quassel feature flags to work at runtime, not compile time

## Nice to Have

- [ ] Rewrite the parsers using nom
- [ ] SessionManager
  - [ ] Automatic InitRequest handling in SessionManager
  - [ ] Add RPC Calls to SessionManager
