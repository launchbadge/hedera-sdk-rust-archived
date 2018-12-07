use crate::{
    crypto::PublicKey,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    AccountId, Client, ContractId, ErrorKind, PreCheckCode,
};
use chrono::{DateTime, Utc};
use failure::Error;
use std::{
    convert::{TryFrom, TryInto},
    time::Duration,
};

pub struct ContractInfo {
    pub contract_id: ContractId,
    pub account_id: AccountId,
    pub contract_account_id: String,
    pub admin_key: Option<PublicKey>,
    pub expiration_time: DateTime<Utc>,
    pub auto_renew_period: Duration,
    pub storage: i64,
}

impl TryFrom<proto::ContractGetInfo::ContractGetInfoResponse_ContractInfo> for ContractInfo {
    type Error = Error;

    fn try_from(
        mut info: proto::ContractGetInfo::ContractGetInfoResponse_ContractInfo,
    ) -> Result<Self, Error> {
        let admin_key = if info.has_adminKey() {
            Some(info.take_adminKey().try_into()?)
        } else {
            None
        };

        Ok(Self {
            contract_id: info.take_contractID().into(),
            account_id: info.take_accountID().into(),
            contract_account_id: info.take_contractAccountID(),
            admin_key,
            expiration_time: info.take_expirationTime().into(),
            auto_renew_period: info.take_autoRenewPeriod().try_into()?,
            storage: info.get_storage(),
        })
    }
}

pub struct QueryContractGetInfo {
    contract: ContractId,
}

impl QueryContractGetInfo {
    pub fn new(client: &Client, contract: ContractId) -> Query<ContractInfo> {
        Query::new(client, Self { contract })
    }
}

impl QueryInner for QueryContractGetInfo {
    type Response = ContractInfo;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_contractGetInfo();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => response.take_contractInfo().try_into(),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetInfo::ContractGetInfoQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract.to_proto()?);

        Ok(Query_oneof_query::contractGetInfo(query))
    }
}
