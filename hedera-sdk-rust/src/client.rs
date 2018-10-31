use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::sync::Arc;

pub struct Client {
    pub(crate) channel: Channel,
}

impl Client {
    pub fn new(address: &str) -> Self {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(address);

        Self { channel: ch }
    }
}
