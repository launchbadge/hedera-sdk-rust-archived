use crate::proto::{self, ToProto};
use failure::Error;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Duration {
    pub seconds: i64,
    pub nanos: i32,
}

impl Duration {
    pub fn new(seconds: i64, nanos: i32) -> Self {
        Duration { seconds, nanos }
    }
}

impl ToProto<proto::Duration::Duration> for Duration {
    fn to_proto(&self) -> Result<proto::Duration::Duration, Error> {
        let mut duration = proto::Duration::Duration::new();
        duration.set_seconds(self.seconds);
        duration.set_nanos(self.nanos);

        Ok(duration)
    }
}
