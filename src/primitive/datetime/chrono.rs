use chrono::{
    DateTime as ChronoDateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Offset,
    TimeZone, Utc,
};
use julianday::JulianDay;

use super::TimeSpec;
use crate::{
    ProtocolError, Result, primitive,
    serialize::{VariantType, *},
};

pub type DateTime = ChronoDateTime<FixedOffset>;
pub type Date = NaiveDate;
pub type Time = NaiveTime;

static LOCAL_OFFSET: std::sync::LazyLock<FixedOffset> =
    std::sync::LazyLock::new(|| Local::now().fixed_offset().timezone());
static UTC_OFFSET: std::sync::LazyLock<FixedOffset> =
    std::sync::LazyLock::new(|| *Utc::now().fixed_offset().offset());

impl super::DateTimeTools for DateTime {
    fn epoch() -> Self {
        ChronoDateTime::<Utc>::UNIX_EPOCH.fixed_offset()
    }

    fn to_i64(&self) -> i64 {
        self.timestamp_millis()
    }

    fn to_i32(&self) -> Result<i32> {
        self.timestamp_millis()
            .try_into()
            .map_err(|_| ProtocolError::TimeStampOverflow)
    }

    fn from_i64(timestamp: i64) -> Result<Self> {
        Ok(ChronoDateTime::<Utc>::from_timestamp_millis(timestamp)
            .ok_or(ProtocolError::TimestampOutOfRange)?
            .fixed_offset())
    }
}

impl<Tz> Serialize for ChronoDateTime<Tz>
where
    Tz: TimeZone,
{
    /// Always serialize to UTC as that is our internal representation
    /// If UTC we do not need to send the fourth field for the timezone offset
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut values: Vec<u8> = Vec::new();

        values.extend(self.date_naive().serialize()?);
        values.extend(self.time().serialize()?);

        let offset = self.offset().fix();
        if offset == *LOCAL_OFFSET {
            values.extend(u8::serialize(&(TimeSpec::LocalUnknown as u8))?);
        } else if offset == *UTC_OFFSET {
            values.extend(u8::serialize(&(TimeSpec::UTC as u8))?);
        } else {
            values.extend(u8::serialize(&(TimeSpec::OffsetFromUTC as u8))?);
            values.extend(i32::serialize(&self.offset().fix().local_minus_utc())?);
        }

        Ok(values)
    }
}

impl Deserialize for DateTime {
    fn parse(b: &[u8]) -> Result<(usize, Self)> {
        let (_, date) = Date::parse(&b[0..4])?;
        let (_, time) = Time::parse(&b[4..8])?;
        let (_, zone) = u8::parse(&b[8..9])?;

        let mut pos = 9;

        let zone = TimeSpec::from(zone as i8);

        let offset: FixedOffset = match zone {
            TimeSpec::LocalUnknown | TimeSpec::LocalStandard | TimeSpec::LocalDST => *LOCAL_OFFSET,
            TimeSpec::UTC => *UTC_OFFSET,
            TimeSpec::OffsetFromUTC => {
                let (_, tmp_offset) = i32::parse(&b[9..13])?;
                pos += 4;
                FixedOffset::east_opt(tmp_offset).unwrap_or(*UTC_OFFSET)
            }
        };

        let naivedatetime = NaiveDateTime::new(date, time);
        let datetime = naivedatetime
            .and_local_timezone(offset)
            .earliest()
            .ok_or(ProtocolError::UndefinedLocalTime)?;

        Ok((pos, datetime))
    }
}

impl VariantType for DateTime {
    const TYPE: u32 = primitive::QDATETIME;
}

impl Serialize for Date {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>> {
        let mut values: Vec<u8> = Vec::new();

        values.extend(i32::serialize(&JulianDay::from(*self).into())?);

        Ok(values)
    }
}

impl Deserialize for Date {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self)> {
        let (_, julian_day) = i32::parse(&b[0..4])?;
        let date = JulianDay::new(julian_day).to_date();

        Ok((4, date))
    }
}

impl VariantType for Date {
    const TYPE: u32 = primitive::QDATE;
}

impl Serialize for Time {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>> {
        let mut values: Vec<u8> = Vec::new();

        let duration = *self - Time::MIN;
        let time = duration.num_milliseconds();

        values.extend(i32::serialize(&(time as i32))?);

        Ok(values)
    }
}

impl Deserialize for Time {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self)> {
        let (_, millis_of_day) = i32::parse(&b[0..4])?;

        let duration = Duration::milliseconds(millis_of_day as i64);
        let time = Time::MIN + duration;

        Ok((4, time))
    }
}

impl VariantType for Time {
    const TYPE: u32 = primitive::QTIME;
}

#[test]
pub fn datetime_serialize_utc_offset() {
    let datetime = DateTime::parse_from_rfc3339("2020-02-19T13:00:00+02:00").unwrap();
    let sers = datetime.serialize().unwrap();
    //               | date       |  | time        |  s  | offset    |
    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];

    assert_eq!(sers, bytes)
}

#[test]
pub fn datetime_serialize_utc() {
    let datetime = DateTime::parse_from_rfc3339("2020-02-19T13:00:00+00:00").unwrap();
    let sers = datetime.serialize().unwrap();
    //               | date       |  | time        |  s  | offset    |
    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 2];

    assert_eq!(sers, bytes)
}

// Test depends on local timezone so only use for debugging
// #[test]
// pub fn datetime_serialize_local() {
//     let datetime = DateTime::parse_from_rfc3339("2020-02-19T13:00:00+01:00").unwrap();
//     let sers = datetime.serialize().unwrap();
//     //               | date       |  | time        |  s  | offset    |
//     let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 255];
//
//     assert_eq!(sers, bytes)
// }

#[test]
pub fn datetime_deserialize_utc_offset() {
    let datetime = DateTime::parse_from_rfc3339("2020-02-19T13:00:00+02:00").unwrap();

    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];
    let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    assert_eq!(res, datetime)
}

#[test]
pub fn datetime_deserialize_utc() {
    let datetime = DateTime::parse_from_rfc3339("2020-02-19T13:00:00+00:00").unwrap();

    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 2];
    let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    assert_eq!(res, datetime)
}

// #[test]
// pub fn datetime_deserialize_epoch() {
//     let datetime = DateTime::epoch();
//
//     let bytes = vec![0, 37, 133, 19, 0xff, 0xff, 0xff, 0xff, 3, 0, 0, 28, 32];
//     let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();
//
//     let bytes = vec![0xff, 0xff, 0xff, 0xff, 2, 202, 28, 128, 3, 0, 0, 28, 32];
//     let (_, res2): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();
//
//     assert_eq!(res, datetime);
//     assert_eq!(res2, datetime)
// }
