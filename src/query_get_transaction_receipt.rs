use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::QueryInner,
    AccountId, Client, ErrorKind, PreCheckCode, Query, TransactionId,
};
use failure::Error;

pub struct QueryGetTransactionReceipt {
    transaction_id: TransactionId,
}

#[repr(C)]
pub struct QueryGetTransactionReceiptAnswer {
    pub status: u8,
    pub account_id: Option<Box<AccountId>>,
    // unsupported: contract_id: Option<Box<ContractId>>,
    // unsupported: file_id: Option<Box<FileId>>,
}

impl QueryGetTransactionReceipt {
    pub fn new(
        client: &Client,
        transaction_id: TransactionId,
    ) -> Query<QueryGetTransactionReceiptAnswer> {
        Query::new(client, Self { transaction_id })
    }
}

impl QueryInner for QueryGetTransactionReceipt {
    type Answer = QueryGetTransactionReceiptAnswer;

    fn answer(&self, mut response: proto::Response::Response) -> Result<Self::Answer, Error> {
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

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetReceipt::TransactionGetReceiptQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction_id.to_proto()?);

        Ok(Query_oneof_query::transactionGetReceipt(query))
    }
}
