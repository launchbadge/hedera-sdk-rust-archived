use crate::{
    proto::{
        self,
        CryptoService_grpc::{CryptoService, CryptoServiceClient},
        FileService_grpc::{FileService, FileServiceClient},
        Query::Query_oneof_query,
        QueryHeader::QueryHeader,
        SmartContractService_grpc::{SmartContractService, SmartContractServiceClient},
        ToProto,
    },
    Client,
};
use failure::Error;
use std::sync::Arc;

#[doc(hidden)]
pub trait QueryInner {
    type Response;
    fn get(&self, response: proto::Response::Response) -> Result<Self::Response, Error>;
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error>;
}

pub struct Query<T> {
    crypto_service: Arc<CryptoServiceClient>,
    contract_service: Arc<SmartContractServiceClient>,
    file_service: Arc<FileServiceClient>,
    kind: proto::QueryHeader::ResponseType,
    payment: Option<proto::Transaction::Transaction>,
    inner: Box<dyn QueryInner<Response = T>>,
}

impl<T> Query<T> {
    pub(crate) fn new<U: QueryInner<Response = T> + 'static>(client: &Client, inner: U) -> Self {
        Self {
            kind: proto::QueryHeader::ResponseType::ANSWER_ONLY,
            payment: None,
            crypto_service: client.crypto.clone(),
            contract_service: client.contract.clone(),
            file_service: client.file.clone(),
            inner: Box::new(inner),
        }
    }

    pub(crate) fn send(&self) -> Result<proto::Response::Response, Error> {
        use self::proto::Query::Query_oneof_query::*;

        let query: proto::Query::Query = self.to_proto()?;

        log::trace!("sent: {:#?}", query);

        let o = grpc::RequestOptions::default();

        let response = match query.query {
            Some(cryptogetAccountBalance(_)) => self.crypto_service.crypto_get_balance(o, query),

            Some(transactionGetReceipt(_)) => {
                self.crypto_service.get_transaction_receipts(o, query)
            }

            Some(cryptoGetInfo(_)) => self.crypto_service.get_account_info(o, query),

            Some(fileGetInfo(_)) => self.file_service.get_file_info(o, query),

            Some(fileGetContents(_)) => self.file_service.get_file_content(o, query),

            Some(transactionGetRecord(_)) => self.crypto_service.get_tx_record_by_tx_id(o, query),

            Some(cryptoGetAccountRecords(_)) => self.crypto_service.get_account_records(o, query),

            Some(contractGetInfo(_)) => self.contract_service.get_contract_info(o, query),

            Some(contractGetBytecode(_)) => self.contract_service.contract_get_bytecode(o, query),

            _ => unreachable!(),
        };

        // TODO: Implement async
        let response = response.wait_drop_metadata()?;

        log::trace!("recv: {:#?}", response);

        Ok(response)
    }

    pub fn get(&mut self) -> Result<T, Error> {
        self.inner.get(self.send()?)
    }

    pub fn payment(mut self, mut transaction: Transaction<TransactionCryptoTransfer>) -> Self {
        if let Some(tx) = transaction.build().as_raw() {
            self.payment = Some(tx.tx.clone())
        }

        self
    }

    pub fn cost(&mut self) -> Result<u64, Error> {
        use self::proto::Response::Response_oneof_response::*;

        // NOTE: This isn't the most ideal way to switch response types..
        self.kind = proto::QueryHeader::ResponseType::COST_ANSWER;
        let response = self.send()?;

        // Why is the cost field inside the specific answer type field in the proto ?
        // Maybe send up a question later.

        let header = match response.response {
            Some(cryptogetAccountBalance(mut res)) => res.take_header(),
            Some(transactionGetReceipt(mut res)) => res.take_header(),
            Some(cryptoGetInfo(mut res)) => res.take_header(),
            Some(fileGetInfo(mut res)) => res.take_header(),
            Some(fileGetContents(mut res)) => res.take_header(),
            Some(transactionGetRecord(mut res)) => res.take_header(),
            Some(cryptoGetAccountRecords(mut res)) => res.take_header(),

            _ => unreachable!(),
        };

        try_precheck!(header).map(|h| h.get_cost())
    }
}

impl<T> ToProto<proto::Query::Query> for Query<T> {
    fn to_proto(&self) -> Result<proto::Query::Query, Error> {
        let mut header = proto::QueryHeader::QueryHeader::new();
        header.set_responseType(self.kind);

        if let Some(payment) = &self.payment {
            header.set_payment(payment.clone());
        }

        let mut query = proto::Query::Query::new();
        query.query = Some(self.inner.to_query_proto(header)?);

        Ok(query)
    }
}
