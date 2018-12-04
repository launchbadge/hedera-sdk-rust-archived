use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    AccountId, Client, ErrorKind, crypto::PublicKey, ContractId, PreCheckCode,
};

use failure::Error;
use std::{time::Duration};
use chrono::{DateTime, Utc};

pub struct QueryContractGetInfoResponse {
    contract_id: ContractId,
    account_id: AccountId,
    contract_account_id: String,
    admin_key: Option<PublicKey>,
    expiration_time: Option<DateTime<Utc>>,
    auto_renew_period: Option<Duration>,
    storage: u64,
}

pub struct QueryContractGetInfo {
    contract: ContractId,
}

impl QueryContractGetInfo {
    pub fn new(client: &Client, contract: ContractId) -> Query<QueryContractGetInfoResponse> {
        Query::new(client, Self { contract })
    }
}

impl QueryInner for QueryContractGetInfo {
    type Response = QueryContractGetInfoResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error>{
        let mut response = response.take_contractGetInfo();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.into()),
            code => Err(ErrorKind::PreCheck(code))?
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetInfo::ContractGetInfoQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract.to_proto()?);

        Ok(Query_oneof_query::contractGetInfo(query))

    }
}
