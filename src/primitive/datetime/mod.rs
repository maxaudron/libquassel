//! DateTime implementation for the Quassel protocol
//!
//! This module provides two alternative datetime implementations using either the time or chrono
//! crates.
//! Choose the implementation that best fits your project's needs.
//!
//!
//! ## Network
//!
//! The DateTime struct represents a DateTime as received in IRC
//! DateTime is, like all other struct based types, serialized sequentially.
//! ```rust
//! pub struct DateTime {
//!     /// Day in Julian calendar, unknown if signed or unsigned
//!     julian_day: i32,
//!     /// Milliseconds since start of day
//!     millis_of_day: i32,
//!     /// Timezone of DateTime, 0x00 is local, 0x01 is UTC
//!     zone: u8,
//! }
//! ```

#[cfg(all(feature = "time", feature = "chrono"))]
compile_error!("feature \"time\" and feature \"chrono\" cannot be enabled at the same time");

#[cfg(feature = "time")]
mod time;

#[cfg(feature = "time")]
pub use time::*;

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "chrono")]
pub use chrono::*;

use crate::Result;

/// Generic implementation of common use tools over both time and chrono
pub trait DateTimeTools {
    /// Get the unix epoch as DateTime
    fn epoch() -> Self;

    /// Convert a DateTime to i64 unix timestamp
    fn to_i64(&self) -> i64;

    /// Convert a DateTime to i32 unix timestamp
    fn to_i32(&self) -> Result<i32>;

    /// Convert a i64 unix timestamp to DateTime
    /// Errors if timestamp is out of range
    fn from_i64(timestamp: i64) -> Result<Self>
    where
        Self: std::marker::Sized;

    /// Convert a i32 unix timestamp to DateTime
    /// Errors if timestamp is out of range
    fn from_i32(timestamp: i32) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        Self::from_i64(timestamp.into())
    }
}

/// TimeSpec specifies whether the time is a local time, daylightsaving local time or a form of UTC Offset
#[repr(i8)]
#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub enum TimeSpec {
    LocalUnknown = -0x01,
    LocalStandard = 0x00,
    LocalDST = 0x01,
    UTC = 0x02,
    OffsetFromUTC = 0x03,
}

impl From<i8> for TimeSpec {
    fn from(val: i8) -> Self {
        match val {
            -0x01 => TimeSpec::LocalUnknown,
            0x00 => TimeSpec::LocalStandard,
            0x01 => TimeSpec::LocalDST,
            0x02 => TimeSpec::UTC,
            0x03 => TimeSpec::OffsetFromUTC,
            _ => unimplemented!(),
        }
    }
}
