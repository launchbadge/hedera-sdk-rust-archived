use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{QueryResponse, ToQueryProto, Query,
    },
    AccountId, Client,
};
use failure::Error;

pub struct QueryCryptoGetAccountBalance {
    account: AccountId,
}

impl QueryCryptoGetAccountBalance {
    pub fn new(client: &Client, account: AccountId) -> Query<Self> {
        Query::new(client, Self { account })
    }
}

impl QueryResponse for QueryCryptoGetAccountBalance {
    type Response = u64;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        Ok(response.take_cryptogetAccountBalance().get_balance())
    }
}

impl ToQueryProto for QueryCryptoGetAccountBalance {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountBalance::CryptoGetAccountBalanceQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptogetAccountBalance(query))
    }
}
