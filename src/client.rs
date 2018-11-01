use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::sync::Arc;

pub struct Client {
    pub(crate) channel: Channel,
}

impl Client {
    pub fn new(address: impl AsRef<str>) -> Self {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(address.as_ref());

        Self { channel: ch }
    }
}
