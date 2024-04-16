mod bufferid;
mod bufferinfo;
mod datetime;
mod message;
mod msgid;
mod signedint;
mod string;
mod stringlist;
mod unsignedint;
mod variant;
mod variantlist;
mod variantmap;

pub use bufferid::*;
pub use bufferinfo::*;
pub use datetime::*;
pub use message::*;
pub use msgid::*;
#[allow(unused_imports)]
pub use signedint::*;
#[allow(unused_imports)]
pub use string::*;
pub use stringlist::*;
#[allow(unused_imports)]
pub use unsignedint::*;
pub use variant::*;
pub use variantlist::*;
pub use variantmap::*;

/// Byte Representation of the type used in Variant to identify it
pub const VOID: u32 = 0x00000000;
/// Byte Representation of the type used in Variant to identify it
pub const BOOL: u32 = 0x00000001;
/// Byte Representation of the type used in Variant to identify it
pub const QCHAR: u32 = 0x00000007;

/// Byte Representation of the type used in Variant to identify it
pub const QVARIANT: u32 = 0x00000090;
/// Byte Representation of the type used in Variant to identify it
pub const QVARIANTMAP: u32 = 0x00000008;
/// Byte Representation of the type used in Variant to identify it
pub const QVARIANTLIST: u32 = 0x00000009;

/// Byte Representation of the type used in Variant to identify it
pub const QSTRING: u32 = 0x0000000a;
/// Byte Representation of the type used in Variant to identify it
pub const QSTRINGLIST: u32 = 0x0000000b;
/// Byte Representation of the type used in Variant to identify it
pub const QBYTEARRAY: u32 = 0x0000000c;

/// Byte Representation of the type used in Variant to identify it
pub const QDATE: u32 = 0x0000000e;
/// Byte Representation of the type used in Variant to identify it
pub const QTIME: u32 = 0x0000000f;
/// Byte Representation of the type used in Variant to identify it
pub const QDATETIME: u32 = 0x00000010;
/// Byte Representation of the type used in Variant to identify it
pub const USERTYPE: u32 = 0x0000007f;

// Basic types
/// Byte Representation of the type used in Variant to identify it
pub const LONG: u32 = 0x00000081; // int64_t
/// Byte Representation of the type used in Variant to identify it
pub const INT: u32 = 0x00000002; // int32_t
/// Byte Representation of the type used in Variant to identify it
pub const SHORT: u32 = 0x00000082; // int16_t
/// Byte Representation of the type used in Variant to identify it
pub const CHAR: u32 = 0x00000083; // int8_t

/// Byte Representation of the type used in Variant to identify it
pub const ULONG: u32 = 0x00000084; // uint64_t
/// Byte Representation of the type used in Variant to identify it
pub const UINT: u32 = 0x00000003; // uint32_t
/// Byte Representation of the type used in Variant to identify it
pub const USHORT: u32 = 0x00000085; // uint16_t
/// Byte Representation of the type used in Variant to identify it
pub const UCHAR: u32 = 0x00000086; // uint8_t
