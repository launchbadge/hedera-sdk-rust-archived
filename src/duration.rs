use crate::proto::{self, ToProto};
use failure::Error;
use std::convert::TryInto;
use std::convert::TryFrom;

#[repr(C)]
#[derive(Debug)]
pub struct Duration(pub(crate) u64, pub(crate) u32);

impl From<std::time::Duration> for Duration {
    fn from(duration: std::time::Duration) -> Self {
        Duration(duration.as_secs(), duration.subsec_nanos())
    }
}

impl From<Duration> for std::time::Duration {
    fn from(Duration(secs, nanos): Duration) -> Self {
        Self::new(secs, nanos)
    }
}

impl ToProto<proto::Duration::Duration> for std::time::Duration {
    fn to_proto(&self) -> Result<proto::Duration::Duration, Error> {
        let mut duration = proto::Duration::Duration::new();
        duration.set_seconds(self.as_secs().try_into()?);
        duration.set_nanos(self.subsec_nanos().try_into()?);

        Ok(duration)
    }
}

impl TryFrom<proto::Duration::Duration> for std::time::Duration {
    type Error = Error;
    fn try_from(duration: proto::Duration::Duration) -> Result<Self, Error> {
        Ok(Self::new(
            duration.get_seconds().try_into()?,
            duration.get_nanos().try_into()?,
        ))
    }
}
