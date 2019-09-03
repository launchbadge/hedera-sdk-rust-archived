use crate::proto::{self, ToProto};
use failure::Error;
use try_from::{TryFrom, TryInto};

impl ToProto<proto::Duration::Duration> for std::time::Duration {
    fn to_proto(&self) -> Result<proto::Duration::Duration, Error> {
        let mut duration = proto::Duration::Duration::new();
        duration.set_seconds(self.as_secs().try_into()?);
//        duration.set_nanos(self.subsec_nanos().try_into()?);

        Ok(duration)
    }
}

impl TryFrom<proto::Duration::Duration> for std::time::Duration {
    type Err = Error;

    fn try_from(duration: proto::Duration::Duration) -> Result<Self, Error> {
        Ok(Self::new(
            duration.get_seconds().try_into()?,
            0,
//            duration.get_nanos().try_into()?,
        ))
    }
}
