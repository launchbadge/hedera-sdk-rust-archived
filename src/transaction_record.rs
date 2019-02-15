use crate::{id::AccountId, proto, query::ContractFunctionResult, TransactionReceipt};
use chrono::{DateTime, Utc};
use failure::{err_msg, Error};
use try_from::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub enum TransactionRecordBody {
    ContractResult(ContractFunctionResult),
    Transfer(Vec<(AccountId, i64)>),
}

#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub receipt: TransactionReceipt,
    pub transaction_hash: Vec<u8>,
    pub consensus_timestamp: Option<DateTime<Utc>>,
    pub memo: String,
    pub transaction_fee: u64,
    pub body: Option<TransactionRecordBody>,
}

impl TryFrom<proto::TransactionRecord::TransactionRecord> for TransactionRecord {
    type Err = Error;

    fn try_from(mut record: proto::TransactionRecord::TransactionRecord) -> Result<Self, Error> {
        Ok(Self {
            receipt: record.take_receipt().into(),
            transaction_hash: record.take_transactionHash(),
            consensus_timestamp: if record.has_consensusTimestamp() {
                Some(record.take_consensusTimestamp().into())
            } else {
                None
            },
            memo: record.take_memo(),
            transaction_fee: record.get_transactionFee(),
            body: {
                if record.has_contractCallResult() {
                    Some(TransactionRecordBody::ContractResult(record.take_contractCallResult().into()))
                } else if record.has_contractCreateResult() {
                    Some(TransactionRecordBody::ContractResult(record.take_contractCreateResult().into()))
                } else if record.has_transferList() {
                    Some(TransactionRecordBody::Transfer(record.take_transferList().into()))
                } else {
                    //Err(err_msg("transaction record contained no body"))?
                    None
                }
            },
        })
    }
}

impl TryFrom<proto::ContractGetRecords::ContractGetRecordsResponse> for Vec<TransactionRecord> {
    type Err = Error;

    fn try_from(
        mut response: proto::ContractGetRecords::ContractGetRecordsResponse,
    ) -> Result<Self, Error> {
        response
            .take_records()
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Self, _>>()
    }
}
