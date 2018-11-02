use crate::proto::Query::Query_oneof_query;
use crate::proto::QueryHeader::QueryHeader;
use crate::{
    proto::{self, ToProto},
    query::ToQueryProto,
    AccountId, Client, ErrorKind, PreCheckCode, Query, TransactionId,
};
use failure::Error;

pub struct QueryGetTransactionReceipt {
    transaction_id: TransactionId,
}

#[repr(C)]
pub struct QueryGetTransactionReceiptAnswer {
    status: u8,
    account_id: Option<Box<AccountId>>,
    // unsupported: contract_id: Option<Box<ContractId>>,
    // unsupported: file_id: Option<Box<FileId>>,
}

impl Query<QueryGetTransactionReceipt> {
    pub fn get_transaction_receipt(client: &Client, transaction_id: TransactionId) -> Self {
        Self::new(client, QueryGetTransactionReceipt { transaction_id })
    }

    pub fn answer(self) -> Result<QueryGetTransactionReceiptAnswer, Error> {
        let mut response = self.send()?;

        let mut response = response.take_transactionGetReceipt();
        let header = response.take_header();
        let mut receipt = response.take_receipt();

        let account_id = if receipt.has_accountID() {
            Some(Box::new(receipt.take_accountID().into()))
        } else {
            None
        };

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(QueryGetTransactionReceiptAnswer {
                status: receipt.get_status() as u8,
                account_id,
            }),

            code => Err(ErrorKind::PreCheck(code))?,
        }
    }
}

impl ToQueryProto for QueryGetTransactionReceipt {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetReceipt::TransactionGetReceiptQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction_id.to_proto()?);

        Ok(Query_oneof_query::transactionGetReceipt(query))
    }
}
