use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    Client, ErrorKind, PreCheckCode,
};
use failure::Error;
use grpcio::Channel;

#[doc(hidden)]
pub trait ToQueryProto {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error>;
}

pub struct Query<T> {
    pub(crate) channel: Channel,
    kind: proto::QueryHeader::ResponseType,
    // TODO: payment: Transaction,
    inner: T,
}

impl<T: ToQueryProto> Query<T> {
    pub(crate) fn new(client: &Client, inner: T) -> Self {
        Self {
            kind: proto::QueryHeader::ResponseType::ANSWER_ONLY,
            channel: client.channel.clone(),
            inner,
        }
    }

    pub(crate) fn send(self) -> Result<proto::Response::Response, Error> {
        use self::proto::Query::Query_oneof_query::*;

        let query = self.to_proto()?;
        let client = proto::CryptoService_grpc::CryptoServiceClient::new(self.channel);

        match query.query {
            Some(cryptogetAccountBalance(_)) => Ok(client.crypto_get_balance(&query)?),
            Some(transactionGetReceipt(_)) => Ok(client.get_transaction_receipts(&query)?),

            _ => unimplemented!(),
        }
    }

    pub fn cost(mut self) -> Result<u64, Error> {
        use self::proto::Response::Response_oneof_response::*;

        // NOTE: This isn't the most ideal way to switch response types..
        self.kind = proto::QueryHeader::ResponseType::COST_ANSWER;
        let response = self.send()?;

        // Why is the cost field inside the specific answer type field in the proto ?
        // Maybe send up a question later.

        let header = match response.response {
            Some(cryptogetAccountBalance(mut res)) => res.take_header(),
            Some(transactionGetReceipt(mut res)) => res.take_header(),

            _ => unreachable!(),
        };

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(header.get_cost()),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }
}

impl<T: ToQueryProto> ToProto<proto::Query::Query> for Query<T> {
    fn to_proto(&self) -> Result<proto::Query::Query, Error> {
        let mut header = proto::QueryHeader::QueryHeader::new();
        header.set_responseType(self.kind);

        let mut query = proto::Query::Query::new();
        query.query = Some(self.inner.to_query_proto(header)?);

        Ok(query)
    }
}
