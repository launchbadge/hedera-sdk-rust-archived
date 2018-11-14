use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::QueryInner,
    AccountId, Client, ErrorKind, PreCheckCode, Query,
};
use failure::Error;

pub type QueryGetAccountBalanceAnswer = u64;

pub struct QueryGetAccountBalance {
    account: AccountId,
}

impl Query<u64> {
    pub fn get_account_balance(client: &Client, account: AccountId) -> Self {
        Self::new(client, QueryGetAccountBalance { account })
    }
}

impl QueryInner for QueryGetAccountBalance {
    type Answer = QueryGetAccountBalanceAnswer;

    fn answer(&self, mut response: proto::Response::Response) -> Result<Self::Answer, Error> {
        let mut response = response.take_cryptogetAccountBalance();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.get_balance()),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountBalance::CryptoGetAccountBalanceQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptogetAccountBalance(query))
    }
}
