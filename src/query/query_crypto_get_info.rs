use crate::{
    claim::Claim,
    crypto::PublicKey,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    AccountId, Client, ErrorKind, PreCheckCode,
};
use chrono::{DateTime, Utc};
use failure::Error;
use std::{
    convert::{TryFrom, TryInto},
    time::Duration,
};

#[derive(Debug)]
pub struct QueryCryptoGetInfoResponse {
    pub account_id: AccountId,
    pub contract_account_id: String,
    pub deleted: bool,
    pub proxy_account_id: AccountId,
    pub proxy_fraction: i32,
    pub proxy_received: i64,
    pub key: PublicKey,
    pub balance: u64,
    pub generate_send_record_threshold: u64,
    pub generate_receive_record_threshold: u64,
    pub receiver_signature_required: bool,
    pub expiration_time: DateTime<Utc>,
    pub auto_renew_period: Duration,
    pub claims: Vec<Claim>,
}

impl TryFrom<proto::CryptoGetInfo::CryptoGetInfoResponse_AccountInfo>
    for QueryCryptoGetInfoResponse
{
    type Error = Error;

    fn try_from(
        mut info: proto::CryptoGetInfo::CryptoGetInfoResponse_AccountInfo,
    ) -> Result<Self, Error> {
        Ok(Self {
            account_id: info.take_accountID().into(),
            contract_account_id: info.take_contractAccountID(),
            deleted: info.get_deleted(),
            proxy_account_id: info.take_proxyAccountID().into(),
            proxy_fraction: info.get_proxyFraction(),
            proxy_received: info.get_proxyReceived(),
            key: info.take_key().try_into()?,
            balance: info.get_balance(),
            generate_send_record_threshold: info.get_generateSendRecordThreshold(),
            generate_receive_record_threshold: info.get_generateReceiveRecordThreshold(),
            receiver_signature_required: info.get_receiverSigRequired(),
            expiration_time: info.take_expirationTime().into(),
            auto_renew_period: info.take_autoRenewPeriod().try_into()?,
            claims: info
                .take_claims()
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

pub struct QueryCryptoGetInfo {
    account: AccountId,
}

impl QueryCryptoGetInfo {
    pub fn new(client: &Client, account: AccountId) -> Query<QueryCryptoGetInfoResponse> {
        Query::new(client, Self { account })
    }
}

impl QueryInner for QueryCryptoGetInfo {
    type Response = QueryCryptoGetInfoResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_cryptoGetInfo();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.take_accountInfo().try_into()?),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetInfo::CryptoGetInfoQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptoGetInfo(query))
    }
}
