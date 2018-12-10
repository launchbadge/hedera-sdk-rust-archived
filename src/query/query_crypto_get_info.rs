use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{
        query::{QueryResponse, ToQueryProto},
        Query,
    },
    AccountId, AccountInfo, Client,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryCryptoGetInfo {
    account: AccountId,
}

impl QueryCryptoGetInfo {
    pub fn new(client: &Client, account: AccountId) -> Query<Self> {
        Query::new(client, Self { account })
    }
}

impl QueryResponse for QueryCryptoGetInfo {
    type Response = AccountInfo;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response.take_cryptoGetInfo().take_accountInfo().try_into()
    }
}

impl ToQueryProto for QueryCryptoGetInfo {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetInfo::CryptoGetInfoQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptoGetInfo(query))
    }
}
