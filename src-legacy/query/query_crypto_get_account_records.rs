use crate::{
    id::AccountId,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryResponse, ToQueryProto},
    Client, TransactionRecord,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryCryptoGetAccountRecords {
    account: AccountId,
}

impl QueryCryptoGetAccountRecords {
    pub fn new(client: &Client, account: AccountId) -> Query<Self> {
        Query::new(client, Self { account })
    }
}

impl QueryResponse for QueryCryptoGetAccountRecords {
    type Response = Vec<TransactionRecord>;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response
            .take_cryptoGetAccountRecords()
            .take_records()
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
    }
}

impl ToQueryProto for QueryCryptoGetAccountRecords {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountRecords::CryptoGetAccountRecordsQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptoGetAccountRecords(query))
    }
}
