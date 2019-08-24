use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryResponse, ToQueryProto},
    Client, TransactionId, TransactionRecord
};
use failure::Error;
use try_from::TryInto;

pub struct QueryTransactionGetRecord {
    transaction: TransactionId,
}

impl QueryTransactionGetRecord {
    pub fn new(client: &Client, transaction: TransactionId) -> Query<Self> {
        Query::new(client, Self { transaction })
    }
}

impl QueryResponse for QueryTransactionGetRecord {
    type Response = TransactionRecord;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response
            .take_transactionGetRecord()
            .take_transactionRecord()
            .try_into()
    }
}

impl ToQueryProto for QueryTransactionGetRecord {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetRecord::TransactionGetRecordQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction.to_proto()?);

        Ok(Query_oneof_query::transactionGetRecord(query))
    }
}
