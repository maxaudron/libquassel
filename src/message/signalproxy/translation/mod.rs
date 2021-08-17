/**
Quassel has 3 main ways to represent an object over the Network:

# VariantList
The struct is serialized to a Vector of Variants. This is mostly used in the InitData messages.
First the field name as a ByteArray (UTF-8 String), followed by the field value in it's own Type's variant.
The order in which the fields are transmitted cannot be assumed.

Example:
```ignore
NetworkConfig {
    ping_timeout_enabled: true,
    ping_interval: 0,
}
```
to
```ignore
VariantList([
    ByteArray(
        "pingTimeoutEnabled",
    ),
    bool(
        true,
    ),
    ByteArray(
        "pingInterval",
    ),
    i32(
        30,
    ),
])
```


# VariantMap
The struct is represented as a `VariantMap`. The keys and values of
the struct are serialized to a corresponding `HashMap<String, Variant>`.

Example:
```ignore
NetworkConfig {
    ping_timeout_enabled: false,
    ping_interval: 0,
}
```
to
```ignore
VariantMap({
  "pingTimeoutEnabled": bool(false)
  "pingInterval": i32(0)
})
```

## Structure of Arrays

For Objects that are transmitted as multiple at once the VariantMap
representation is augemented and instead of transmitting multiple `VariantMaps`,
each field is a `VariantList` of Items.

Example:
```ignore
vec![
    NetworkConfig {
        ping_timeout_enabled: false,
        ping_interval: 0,
    },
    NetworkConfig {
        ping_timeout_enabled: true,
        ping_interval: 1,
    },
]
```
to
```ignore
VariantMap({
  "pingTimeoutEnabled": VariantList([
    bool(false),
    bool(true),
  ]),
  "pingInterval": VariantList([
    i32(0),
    i32(1),
  ])
})
```
**/
use crate::primitive::{VariantList, VariantMap};

pub trait Network {
    type Item;

    fn to_network(&self) -> Self::Item;
    fn from_network(input: &mut Self::Item) -> Self;
}

pub trait NetworkMap {
    fn to_network_map(&self) -> VariantMap;
    fn from_network_map(input: &mut VariantMap) -> Self;
}

pub trait NetworkList {
    fn to_network_list(&self) -> VariantList;
    fn from_network_list(input: &mut VariantList) -> Self;
}
