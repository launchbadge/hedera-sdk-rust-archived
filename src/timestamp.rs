use crate::{
    error::ErrorKind,
    proto::{self, ToProto},
};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use failure::Error;
use itertools::Itertools;
use std::str::FromStr;
use try_from::TryInto;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct Timestamp(pub(crate) i64, pub(crate) i32);

impl From<Timestamp> for DateTime<Utc> {
    fn from(Timestamp(seconds, nanos): Timestamp) -> Self {
        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(
            seconds,
            nanos.try_into().unwrap(),
        ))
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Timestamp(
            dt.timestamp(),
            dt.timestamp_subsec_nanos().try_into().unwrap(),
        )
    }
}

impl From<proto::Timestamp::Timestamp> for DateTime<Utc> {
    fn from(dt: proto::Timestamp::Timestamp) -> Self {
        Timestamp(dt.get_seconds(), dt.get_nanos()).into()
    }
}

impl ToProto<proto::Timestamp::Timestamp> for DateTime<Utc> {
    fn to_proto(&self) -> Result<proto::Timestamp::Timestamp, Error> {
        let mut timestamp = proto::Timestamp::Timestamp::new();
        timestamp.set_seconds(self.timestamp());
        timestamp.set_nanos(self.timestamp_subsec_nanos().try_into()?);

        Ok(timestamp)
    }
}

impl ToProto<proto::Timestamp::TimestampSeconds> for DateTime<Utc> {
    fn to_proto(&self) -> Result<proto::Timestamp::TimestampSeconds, Error> {
        let mut timestamp = proto::Timestamp::TimestampSeconds::new();
        timestamp.set_seconds(self.timestamp());
        Ok(timestamp)
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seconds, nanos) = s
            .split('.')
            .next_tuple()
            .ok_or_else(|| ErrorKind::Parse("{seconds}.{nanos}"))?;

        Ok(Timestamp(seconds.parse()?, nanos.parse()?))
    }
}
