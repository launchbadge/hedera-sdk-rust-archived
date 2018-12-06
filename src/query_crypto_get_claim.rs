use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    claim::Claim,
    AccountId, Client, ErrorKind, PreCheckCode,
};
use failure::Error;
use std::convert::TryInto;

pub struct QueryCryptoGetClaimResponse {
    claim: Claim
}

pub struct QueryCryptoGetClaim{
    account: AccountId,
    hash: Vec<u8>,
}

impl QueryCryptoGetClaim {
    pub fn new(client: &Client, account: AccountId, hash: Vec<u8>) -> Query<QueryCryptoGetClaimResponse> {
        Query::new(client, Self { account, hash })
    }
}

impl QueryInner for QueryCryptoGetClaim {
    type Response = QueryCryptoGetClaimResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error>{
        let mut response = response.take_cryptoGetClaim();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(QueryCryptoGetClaimResponse{claim: response.take_claim().try_into()?}),
            code => Err(ErrorKind::PreCheck(code))?
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetClaim::CryptoGetClaimQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);
        query.set_hash(self.hash.clone());

        Ok(Query_oneof_query::cryptoGetClaim(query))

    }
}
