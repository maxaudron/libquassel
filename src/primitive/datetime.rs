use crate::{error::ProtocolError, primitive, serialize::*};

use time::{Duration, OffsetDateTime, PrimitiveDateTime, UtcOffset};

/// The DateTime struct represents a DateTime as received in IRC
///
/// DateTime is, like all other struct based types, serialized sequentially.
/// #[derive(Clone, Debug, std::cmp::PartialEq)]
/// pub struct DateTime {
///     /// Day in Julian calendar, unknown if signed or unsigned
///     julian_day: i32,
///     /// Milliseconds since start of day
///     millis_of_day: i32,
///     /// Timezone of DateTime, 0x00 is local, 0x01 is UTC
///     zone: u8,
/// }
pub type DateTime = OffsetDateTime;
pub use time::{Date, Time};

use crate::serialize::VariantType;

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

impl Serialize for OffsetDateTime {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: Vec<u8> = Vec::new();

        values.extend(i32::serialize(&self.date().to_julian_day())?);

        let duration = self.time() - Time::MIDNIGHT;
        let time = duration.whole_milliseconds();

        values.extend(i32::serialize(&(time as i32))?);
        values.extend(u8::serialize(&(TimeSpec::OffsetFromUTC as u8))?);
        values.extend(i32::serialize(&self.offset().whole_seconds())?);

        Ok(values)
    }
}

impl Deserialize for OffsetDateTime {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (_, julian_day) = i32::parse(&b[0..4])?;
        let (_, millis_of_day) = i32::parse(&b[4..8])?;
        let (_, zone) = u8::parse(&b[8..9])?;

        let mut pos = 9;

        let zone = TimeSpec::from(zone as i8);

        // Default to unix epoch when one of these is set to -1
        if julian_day == -1 || millis_of_day == -1 {
            return Ok((pos, OffsetDateTime::UNIX_EPOCH));
        }

        let offset = match zone {
            TimeSpec::LocalUnknown | TimeSpec::LocalStandard | TimeSpec::LocalDST => {
                UtcOffset::current_local_offset().unwrap_or_else(|_| {
                    log::warn!("could not get local offset defaulting to utc");
                    UtcOffset::UTC
                })
            }
            TimeSpec::UTC => UtcOffset::UTC,
            TimeSpec::OffsetFromUTC => {
                let (_, tmp_offset) = i32::parse(&b[9..13])?;
                pos += 4;
                UtcOffset::from_whole_seconds(tmp_offset).unwrap_or(UtcOffset::UTC)
            }
        };

        let date = Date::from_julian_day(julian_day)?;

        let duration = Duration::milliseconds(millis_of_day as i64);
        let time = Time::MIDNIGHT + duration;

        let primitivedatetime = PrimitiveDateTime::new(date, time);
        let datetime = primitivedatetime.assume_offset(offset);

        Ok((pos, datetime))
    }
}

impl VariantType for OffsetDateTime {
    const TYPE: u32 = primitive::QDATETIME;
}

impl Serialize for Date {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut values: Vec<u8> = Vec::new();

        values.extend(i32::serialize(&self.to_julian_day())?);

        Ok(values)
    }
}

impl Deserialize for Date {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (_, julian_day) = i32::parse(&b[0..4])?;
        let date = Date::from_julian_day(julian_day)?;

        Ok((4, date))
    }
}

impl VariantType for Date {
    const TYPE: u32 = primitive::QDATE;
}

impl Serialize for Time {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut values: Vec<u8> = Vec::new();

        let duration = *self - Time::MIDNIGHT;
        let time = duration.whole_milliseconds();

        values.extend(i32::serialize(&(time as i32))?);

        Ok(values)
    }
}

impl Deserialize for Time {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (_, millis_of_day) = i32::parse(&b[0..4])?;

        let duration = Duration::milliseconds(millis_of_day as i64);
        let time = Time::MIDNIGHT + duration;

        Ok((4, time))
    }
}

impl VariantType for Time {
    const TYPE: u32 = primitive::QTIME;
}

#[test]
pub fn datetime_serialize() {
    let datetime = DateTime::parse(
        "2020-02-19 13:00 +0200",
        time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute] [offset_hour sign:mandatory][offset_minute]"
        ),
    )
    .unwrap();

    let sers = datetime.serialize().unwrap();
    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];

    assert_eq!(sers, bytes)
}

#[test]
pub fn datetime_deserialize() {
    let datetime = DateTime::parse(
        "2020-02-19 13:00 +0200",
        time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute] [offset_hour sign:mandatory][offset_minute]"
        ),
    )
    .unwrap();

    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];
    let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    assert_eq!(res, datetime)
}

#[test]
pub fn datetime_deserialize_epoch() {
    let datetime = DateTime::UNIX_EPOCH;

    let bytes = vec![0, 37, 133, 19, 0xff, 0xff, 0xff, 0xff, 3, 0, 0, 28, 32];
    let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    let bytes = vec![0xff, 0xff, 0xff, 0xff, 2, 202, 28, 128, 3, 0, 0, 28, 32];
    let (_, res2): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    assert_eq!(res, datetime);
    assert_eq!(res2, datetime)
}
