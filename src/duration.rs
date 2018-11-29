use crate::proto::{self, ToProto};
use failure::Error;
use std::convert::TryInto;

impl ToProto<proto::Duration::Duration> for std::time::Duration {
    fn to_proto(&self) -> Result<proto::Duration::Duration, Error> {
        let mut duration = proto::Duration::Duration::new();
        duration.set_seconds(self.as_secs().try_into()?);
        duration.set_nanos(self.subsec_nanos().try_into()?);

        Ok(duration)
    }
}
