use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    AccountId, AccountInfo, Client,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryCryptoGetInfo {
    account: AccountId,
}

impl QueryCryptoGetInfo {
    pub fn new(client: &Client, account: AccountId) -> Query<AccountInfo> {
        Query::new(client, Self { account })
    }
}

impl QueryInner for QueryCryptoGetInfo {
    type Response = AccountInfo;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response.take_cryptoGetInfo().take_accountInfo().try_into()
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetInfo::CryptoGetInfoQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptoGetInfo(query))
    }
}
