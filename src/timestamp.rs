use chrono::Utc;
use crate::error::ErrorKind;
use crate::proto::{self, ToProto};
use failure::Error;
use itertools::Itertools;
use std::ops::Sub;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

impl Timestamp {
    pub fn now() -> Self {
        let now = Utc::now();

        Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        }
    }
}

// Subtract seconds from this timestamp
impl Sub<i64> for Timestamp {
    type Output = Timestamp;

    fn sub(self, rhs: i64) -> Self::Output {
        Timestamp {
            seconds: self.seconds - rhs,
            nanos: self.nanos,
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
            .ok_or_else(|| ErrorKind::Parse("{seconds}.{nanos}"))?;

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
    fn to_proto(&self) -> Result<proto::Timestamp::Timestamp, Error> {
        let mut timestamp = proto::Timestamp::Timestamp::new();
        timestamp.set_seconds(self.seconds);
        timestamp.set_nanos(self.nanos);

        Ok(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::Timestamp;
    use failure::Error;

    #[test]
    fn test_display() {
        let timestamp = Timestamp {
            seconds: 1234567,
            nanos: 10001,
        };

        assert_eq!(format!("{}", timestamp), "1234567.10001");
    }

    #[test]
    fn test_parse() -> Result<(), Error> {
        let timestamp = Timestamp {
            seconds: 1234567,
            nanos: 10001,
        };

        assert_eq!("1234567.10001".parse::<Timestamp>()?, timestamp);

        Ok(())
    }
}
