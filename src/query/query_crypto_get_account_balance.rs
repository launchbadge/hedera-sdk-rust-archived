use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    AccountId, Client,
};
use failure::Error;

pub struct QueryCryptoGetAccountBalance {
    account: AccountId,
}

impl QueryCryptoGetAccountBalance {
    pub fn new(client: &Client, account: AccountId) -> Query<u64> {
        Query::new(client, Self { account })
    }
}

impl QueryInner for QueryCryptoGetAccountBalance {
    type Response = u64;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_cryptogetAccountBalance();
        let header = response.take_header();

        try_precheck!(header).map(move |_| response.get_balance())
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountBalance::CryptoGetAccountBalanceQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptogetAccountBalance(query))
    }
}
