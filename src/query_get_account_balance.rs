use crate::proto::Query::Query_oneof_query;
use crate::proto::QueryHeader::QueryHeader;
use crate::{
    proto::{self, ToProto},
    query::ToQueryProto,
    AccountId, Client, ErrorKind, PreCheckCode, Query,
};
use failure::Error;

pub struct QueryGetAccountBalance {
    account: AccountId,
}

#[repr(C)]
pub struct QueryGetAccountBalanceAnswer {
    balance: u64,
}

impl Query<QueryGetAccountBalance> {
    pub fn get_account_balance(client: &Client, account: AccountId) -> Self {
        Self::new(client, QueryGetAccountBalance { account })
    }

    pub fn answer(self) -> Result<QueryGetAccountBalanceAnswer, Error> {
        let mut response = self.send()?;

        let mut response = response.take_cryptogetAccountBalance();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(QueryGetAccountBalanceAnswer {
                balance: response.get_balance(),
            }),

            code => Err(ErrorKind::PreCheck(code))?,
        }
    }
}

impl ToQueryProto for QueryGetAccountBalance {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountBalance::CryptoGetAccountBalanceQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptogetAccountBalance(query))
    }
}
