use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    transaction_receipt::TransactionReceipt,
    Client, ErrorKind, PreCheckCode, TransactionId,
};
use failure::Error;

pub struct QueryGetTransactionReceipt {
    transaction_id: TransactionId,
}

pub type QueryGetTransactionReceiptResponse = TransactionReceipt;

impl QueryGetTransactionReceipt {
    pub fn new(
        client: &Client,
        transaction_id: TransactionId,
    ) -> Query<QueryGetTransactionReceiptResponse> {
        Query::new(client, Self { transaction_id })
    }
}

impl QueryInner for QueryGetTransactionReceipt {
    type Response = QueryGetTransactionReceiptResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_transactionGetReceipt();
        let header = response.take_header();
        let mut receipt = response.take_receipt();

        let account_id = if receipt.has_accountID() {
            Some(Box::new(receipt.take_accountID().into()))
        } else {
            None
        };

        let file_id = if receipt.has_fileID() {
            Some(Box::new(receipt.take_fileID().into()))
        } else {
            None
        };

        let contract_id = if receipt.has_contractID() {
            Some(Box::new(receipt.take_contractID().into()))
        } else {
            None
        };

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(QueryGetTransactionReceiptResponse {
                status: receipt.get_status().into(),
                account_id,
                contract_id,
                file_id,
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
