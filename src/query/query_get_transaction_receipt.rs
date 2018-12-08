use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, TransactionId, TransactionReceipt,
};
use failure::Error;

pub struct QueryGetTransactionReceipt {
    transaction_id: TransactionId,
}

impl QueryGetTransactionReceipt {
    pub fn new(client: &Client, transaction_id: TransactionId) -> Query<TransactionReceipt> {
        Query::new(client, Self { transaction_id })
    }
}

impl QueryInner for QueryGetTransactionReceipt {
    type Response = TransactionReceipt;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        Ok(response.take_transactionGetReceipt().take_receipt().into())
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetReceipt::TransactionGetReceiptQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction_id.to_proto()?);

        Ok(Query_oneof_query::transactionGetReceipt(query))
    }
}
