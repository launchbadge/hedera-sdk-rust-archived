use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryResponse, ToQueryProto},
    AccountId, Client, ContractId
};
use failure::Error;

pub struct QueryCryptoGetAccountBalance {
    account: Option<AccountId>,
    contract: Option<ContractId>
}

impl QueryCryptoGetAccountBalance {
    pub fn new(client: &Client, account: Option<AccountId>,
               contract: Option<ContractId>) -> Query<Self> {
        let mut valid = false;

        if let Some(acct) = account {
            valid = true;
        }

        if let Some(cont) = contract {
            if valid {
                format_err!("either a ContractID or AccountID can be passed to GetAccountBalance \
                but not both");
            }
            valid = true;
        }

        if !valid {
            format_err!("either a ContractID or AccountID must be passed to GetAccountBalance")
        }

        let q = Self {
            account: acct,
            contract: cont
        };

        Query::new(client, q)
    }
}

impl QueryResponse for QueryCryptoGetAccountBalance {
    type Response = u64;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        Ok(response.take_cryptogetAccountBalance().get_balance())
    }
}

// TODO: Add support for ContractID
impl ToQueryProto for QueryCryptoGetAccountBalance {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountBalance::CryptoGetAccountBalanceQuery::new();
        query.set_header(header);

        if let Some(account) = self.account {
            query.set_accountID(self.account.to_proto()?);
        }

        if let Some(contract) = self.contract {
            query.set_contractID(self.contract.to_proto()?);
        }

        Ok(Query_oneof_query::cryptogetAccountBalance(query))
    }
}
