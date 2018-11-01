use chrono::{Duration, Utc};
use crate::proto::{self, ToProto};
use failure::{err_msg, Error};
use itertools::Itertools;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

impl Timestamp {
    pub fn new() -> Self {
        // Allows the transaction to be accepted as long as the
        // server is not more than 5 seconds behind us
        let now = Utc::now() - Duration::seconds(5);

        Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}.{}", self.seconds, self.nanos)
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seconds, nanos) = s
            .split('.')
            .next_tuple()
            .ok_or_else(|| err_msg("expected string of the format: {seconds}.{nanos}"))?;

        Ok(Self {
            seconds: seconds.parse()?,
            nanos: nanos.parse()?,
        })
    }
}

impl From<crate::proto::Timestamp::Timestamp> for Timestamp {
    fn from(pb: crate::proto::Timestamp::Timestamp) -> Self {
        Self {
            seconds: pb.get_seconds(),
            nanos: pb.get_nanos(),
        }
    }
}

impl ToProto<proto::Timestamp::Timestamp> for Timestamp {
    fn to_proto(&self) -> proto::Timestamp::Timestamp {
        let mut timestamp = proto::Timestamp::Timestamp::new();
        timestamp.set_seconds(self.seconds);
        timestamp.set_nanos(self.nanos);

        timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn test_display() {
        let timestamp = Timestamp {
            seconds: 1234567,
            nanos: 10001,
        };

        assert_eq!(format!("{}", timestamp), "1234567.10001");
    }

    #[test]
    fn test_parse() {
        let timestamp = Timestamp {
            seconds: 1234567,
            nanos: 10001,
        };

        assert_eq!("1234567.10001".parse::<Timestamp>().unwrap(), timestamp);
    }
}
