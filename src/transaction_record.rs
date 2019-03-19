use crate::{id::AccountId, id:: ContractId, proto, query::ContractFunctionResult, TransactionReceipt, proto::CryptoTransfer::TransferList};
use chrono::{DateTime, Utc};
use failure::{err_msg, Error};
use try_from::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub receipt: TransactionReceipt,
    pub transaction_hash: Vec<u8>,
    pub consensus_timestamp: Option<DateTime<Utc>>,
    pub memo: String,
    pub transaction_fee: u64,
    pub contract_function_result: Option<ContractFunctionResult>,
    pub transfers: Option<TransferList>,
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
            transfers: if record.has_transferList() {
                Some(record.take_transferList())
            } else {
                None
            },
            contract_function_result: if record.has_contractCallResult() {
                Some(record.take_contractCallResult().into())
            } else if record.has_contractCreateResult() {
                Some(record.take_contractCreateResult().into())
            } else {
                None
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
