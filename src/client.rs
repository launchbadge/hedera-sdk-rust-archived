use failure::{format_err, Error};
use itertools::Itertools;
use std::{sync::Arc, time::Duration};

pub struct Client {
    pub(crate) inner: Arc<grpc::Client>,
}

impl Client {
    pub fn new(address: impl AsRef<str>) -> Result<Self, Error> {
        let address = address.as_ref();
        let (host, port) = address.split(':').next_tuple().ok_or_else(|| {
            format_err!("failed to parse 'host:port' from address: {:?}", address)
        })?;

        let port = port.parse()?;

        let inner = Arc::new(grpc::Client::new_plain(
            &host,
            port,
            grpc::ClientConf {
                http: httpbis::ClientConf {
                    no_delay: Some(true),
                    connection_timeout: Some(Duration::from_secs(5)),
                    ..httpbis::ClientConf::default()
                },
            },
        )?);

        Ok(Self { inner })
    }
}
