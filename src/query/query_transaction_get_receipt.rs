use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{
        query::{QueryResponse, ToQueryProto},
        Query,
    },
    Client, TransactionId, TransactionReceipt,
};
use failure::Error;

pub struct QueryTransactionGetReceipt {
    transaction_id: TransactionId,
}

impl QueryTransactionGetReceipt {
    pub fn new(client: &Client, transaction_id: TransactionId) -> Query<Self> {
        Query::new(client, Self { transaction_id })
    }
}

impl QueryResponse for QueryTransactionGetReceipt {
    type Response = TransactionReceipt;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        Ok(response.take_transactionGetReceipt().take_receipt().into())
    }
}

impl ToQueryProto for QueryTransactionGetReceipt {
    fn is_free(&self) -> bool {
        true
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetReceipt::TransactionGetReceiptQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction_id.to_proto()?);

        Ok(Query_oneof_query::transactionGetReceipt(query))
    }
}
