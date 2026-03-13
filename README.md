[![git mirror](https://img.shields.io/badge/git-repo-cba6f7?logo=git)](https://git.vapor.systems/libquassel.git)
[![github mirror](https://img.shields.io/badge/github-repo-blue?logo=github)](https://github.com/maxaudron/libquassel)
[![gitlab mirror](https://img.shields.io/badge/gitlab-repo-orange?logo=github)](https://gitlab.com/cocainefarm/libquassel)

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
- [X] Rework Error handling to actually handle errors
- [X] Rework Quassel feature flags to work at runtime, not compile time
- [X] Rewrite NetworkList impl to convert to NetworkMap first

## Nice to Have

- [ ] Rewrite the parsers using nom
- [ ] SessionManager
  - [ ] Automatic InitRequest handling in SessionManager
  - [ ] Add RPC Calls to SessionManager
