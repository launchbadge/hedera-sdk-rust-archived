use crate::{
    id::{AccountId, ContractId},
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    transaction::TransactionReceipt,
    Client, ErrorKind, PreCheckCode, TransactionId,
};
use chrono::{DateTime, Utc};
use failure::{err_msg, Error};
use std::convert::{TryFrom, TryInto};

// TODO: Move to ContractCallLocalQuery once it exists
pub struct ContractLogInfo {
    pub bloom: Vec<u8>,
    pub topic: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl From<proto::ContractCallLocal::ContractLoginfo> for ContractLogInfo {
    fn from(mut log: proto::ContractCallLocal::ContractLoginfo) -> Self {
        Self {
            bloom: log.take_bloom(),
            topic: log.take_topic().into_vec(),
            data: log.take_data(),
        }
    }
}

// TODO: Move to ContractCallLocalQuery once it exists
pub struct ContractFunctionResult {
    pub contract_id: ContractId,
    pub contract_call_result: Vec<u8>,
    pub error_message: String,
    pub bloom: Vec<u8>,
    pub gas_used: u64,
    pub log_info: Vec<ContractLogInfo>,
}

impl From<proto::ContractCallLocal::ContractFunctionResult> for ContractFunctionResult {
    fn from(mut result: proto::ContractCallLocal::ContractFunctionResult) -> Self {
        Self {
            contract_id: result.take_contractID().into(),
            contract_call_result: result.take_contractCallResult(),
            error_message: result.take_errorMessage(),
            bloom: result.take_bloom(),
            gas_used: result.get_gasUsed(),
            log_info: result.take_logInfo().into_iter().map(Into::into).collect(),
        }
    }
}

pub enum QueryTransactionGetRecordResponseBody {
    ContractCall(ContractFunctionResult),
    ContractCreate(ContractFunctionResult),
    Transfer(Vec<(AccountId, i64)>),
}

pub struct QueryTransactionGetRecordResponse {
    pub receipt: TransactionReceipt,
    pub transaction_hash: Vec<u8>,
    pub consensus_timestamp: Option<DateTime<Utc>>,
    pub memo: String,
    pub transaction_fee: u64,
    pub body: QueryTransactionGetRecordResponseBody,
}

impl TryFrom<proto::TransactionRecord::TransactionRecord> for QueryTransactionGetRecordResponse {
    type Error = Error;

    fn try_from(mut record: proto::TransactionRecord::TransactionRecord) -> Result<Self, Error> {
        Ok(Self {
            receipt: record.take_receipt().into(),
            transaction_hash: record.take_transactionHash(),
            consensus_timestamp: if record.has_consensusTimestamp() {
                Some(record.take_consensusTimestamp().try_into()?)
            } else {
                None
            },
            memo: record.take_memo(),
            transaction_fee: record.get_transactionFee(),
            body: {
                if record.has_contractCallResult() {
                    QueryTransactionGetRecordResponseBody::ContractCall(
                        record.take_contractCallResult().into(),
                    )
                } else if record.has_contractCreateResult() {
                    QueryTransactionGetRecordResponseBody::ContractCreate(
                        record.take_contractCreateResult().into(),
                    )
                } else if record.has_transferList() {
                    QueryTransactionGetRecordResponseBody::Transfer(
                        record.take_transferList().into(),
                    )
                } else {
                    Err(err_msg("transaction record contained no body"))?
                }
            },
        })
    }
}

pub struct QueryTransactionGetRecord {
    transaction: TransactionId,
}

impl QueryTransactionGetRecord {
    pub fn new(
        client: &Client,
        transaction: TransactionId,
    ) -> Query<QueryTransactionGetRecordResponse> {
        Query::new(client, Self { transaction })
    }
}

impl QueryInner for QueryTransactionGetRecord {
    type Response = QueryTransactionGetRecordResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_transactionGetRecord();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.take_transactionRecord().try_into()?),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetRecord::TransactionGetRecordQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction.to_proto()?);

        Ok(Query_oneof_query::transactionGetRecord(query))
    }
}
