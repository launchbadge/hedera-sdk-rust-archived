use crate::{
    id::AccountId,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, TransactionRecord,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryCryptoGetAccountRecords {
    account: AccountId,
}

impl QueryCryptoGetAccountRecords {
    pub fn new(client: &Client, account: AccountId) -> Query<Vec<TransactionRecord>> {
        Query::new(client, Self { account })
    }
}

impl QueryInner for QueryCryptoGetAccountRecords {
    type Response = Vec<TransactionRecord>;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_cryptoGetAccountRecords();
        let header = response.take_header();

        try_precheck!(header).and_then(move |_| {
            response
                .take_records()
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountRecords::CryptoGetAccountRecordsQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptoGetAccountRecords(query))
    }
}
