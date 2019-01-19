use crate::{
    claim::Claim,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{QueryResponse, ToQueryProto, Query,
    },
    AccountId, Client,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryCryptoGetClaim {
    account: AccountId,
    hash: Vec<u8>,
}

impl QueryCryptoGetClaim {
    pub fn new(client: &Client, account: AccountId, hash: Vec<u8>) -> Query<Self> {
        Query::new(client, Self { account, hash })
    }
}

impl QueryResponse for QueryCryptoGetClaim {
    type Response = Claim;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response.take_cryptoGetClaim().take_claim().try_into()
    }
}

impl ToQueryProto for QueryCryptoGetClaim {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetClaim::CryptoGetClaimQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);
        query.set_hash(self.hash.clone());

        Ok(Query_oneof_query::cryptoGetClaim(query))
    }
}
