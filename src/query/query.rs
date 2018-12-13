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
    transaction::{Transaction, TransactionCryptoTransfer},
    AccountId, Client, ErrorKind, Status, SecretKey,
};
use failure::Error;
use futures::{Future, TryFutureExt};
use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};
use tokio::await;
use tokio_async_await::compat::backward::Compat;

pub(crate) trait ToQueryProto {
    fn is_free(&self) -> bool { false }
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error>;
}

#[doc(hidden)]
pub trait QueryResponse {
    type Response: Send;

    fn get(response: proto::Response::Response) -> Result<Self::Response, Error>;
}

impl QueryResponse for () {
    type Response = ();

    fn get(_: proto::Response::Response) -> Result<(), Error> {
        Ok(())
    }
}

pub struct Query<T>
where
    T: QueryResponse + Send + Sync + 'static,
{
    crypto_service: Arc<CryptoServiceClient>,
    contract_service: Arc<SmartContractServiceClient>,
    file_service: Arc<FileServiceClient>,
    payment: Option<proto::Transaction::Transaction>,
    secret: Option<Arc<dyn Fn() -> Result<SecretKey, Error> + Send + Sync>>,
    operator: Option<AccountId>,
    node: Option<AccountId>,
    inner: Box<dyn ToQueryProto + Send + Sync>,
    phantom: PhantomData<T>,
}

impl<T> Query<T>
where
    T: QueryResponse + Send + Sync + 'static,
{
    pub(crate) fn new(client: &Client, inner: T) -> Self
    where
        T: ToQueryProto,
    {
        Self {
            payment: None,
            crypto_service: client.crypto.clone(),
            contract_service: client.contract.clone(),
            file_service: client.file.clone(),
            node: client.node,
            operator: client.operator,
            secret: client.operator_secret.clone(),
            inner: Box::new(inner),
            phantom: PhantomData,
        }
    }

    pub fn payment<S: 'static>(
        &mut self,
        transaction: &mut Transaction<TransactionCryptoTransfer, S>,
    ) -> Result<&mut Self, Error> {
        self.payment = Some(transaction.build().take_raw()?.tx);
        Ok(self)
    }

    pub fn get_async(&mut self) -> impl Future<Output = Result<T::Response, Error>> {
        self.send()
            .and_then(move |(_, response)| futures::future::ready(T::get(response)))
    }

    pub fn get(&mut self) -> Result<T::Response, Error> {
        crate::RUNTIME
            .lock()
            .block_on(Compat::new(self.get_async()))
    }

    fn send(
        &mut self,
    ) -> impl Future<
        Output = Result<
            (
                proto::ResponseHeader::ResponseHeader,
                proto::Response::Response,
            ),
            Error,
        >,
    > {
        use self::proto::Query::Query_oneof_query::*;

        if !self.inner.is_free() && self.payment.is_none() {
            // Attach a payment transaction if this is a non-free query and we
            // have payment details
            if self.operator.is_some() && self.node.is_some() && self.secret.is_some() {
                let cost = 100_000;
                self.payment = TransactionCryptoTransfer::new(&Client {
                    node: self.node.clone(),
                    operator: self.operator.clone(),
                    operator_secret: self.secret.clone(),
                    crypto: self.crypto_service.clone(),
                    file: self.file_service.clone(),
                    contract: self.contract_service.clone(),
                })
                .transfer(*self.node.as_ref().unwrap(), cost as i64)
                .transfer(*self.operator.as_ref().unwrap(), -(cost as i64))
                .build()
                .take_raw().ok().map(|tx| tx.tx);
            }
        }

        let attempt = AtomicUsize::new(0);
        let crypto = self.crypto_service.clone();
        let file = self.file_service.clone();
        let contract = self.contract_service.clone();
        let query_res: Option<Result<proto::Query::Query, _>> = Some(self.to_proto());

        async move {
            #[allow(clippy::never_loop)]
            loop {
                break if let Some(Ok(query)) = &query_res {
                    if attempt.load(Ordering::SeqCst) == 0 {
                        log::trace!("sent: {:#?}", query);
                    }

                    let query = query.clone();
                    let o = grpc::RequestOptions::default();
                    let response = match query.query {
                        Some(cryptogetAccountBalance(_)) => crypto.crypto_get_balance(o, query),
                        Some(cryptoGetInfo(_)) => crypto.get_account_info(o, query),
                        Some(fileGetInfo(_)) => file.get_file_info(o, query),
                        Some(fileGetContents(_)) => file.get_file_content(o, query),
                        Some(transactionGetRecord(_)) => crypto.get_tx_record_by_tx_id(o, query),
                        Some(cryptoGetAccountRecords(_)) => crypto.get_account_records(o, query),
                        Some(contractGetInfo(_)) => contract.get_contract_info(o, query),
                        Some(contractGetBytecode(_)) => contract.contract_get_bytecode(o, query),
                        Some(transactionGetReceipt(_)) => crypto.get_transaction_receipts(o, query),

                        _ => unreachable!(),
                    };

                    let mut response = await!(response.drop_metadata())?;
                    log::trace!("recv: {:#?}", response);

                    let header = take_header(&mut response);
                    match header.get_nodeTransactionPrecheckCode().into() {
                        Status::Busy if attempt.load(Ordering::SeqCst) < 5 => {
                            let attempt = attempt.fetch_add(1, Ordering::SeqCst) + 1;
                            sleep(Duration::from_secs((attempt * 2) as u64));
                            continue;
                        }

                        Status::Ok => Ok((header, response)),

                        pre_check_code => Err(ErrorKind::PreCheck(pre_check_code))?,
                    }
                } else if let Some(Err(error)) = query_res {
                    Err(error)
                } else {
                    unreachable!()
                };
            }
        }
    }
}

impl<T> ToProto<proto::Query::Query> for Query<T>
where
    T: QueryResponse + Send + Sync + 'static,
{
    fn to_proto(&self) -> Result<proto::Query::Query, Error> {
        let mut header = proto::QueryHeader::QueryHeader::new();
        header.set_responseType(proto::QueryHeader::ResponseType::ANSWER_ONLY);

        if let Some(payment) = &self.payment {
            header.set_payment(payment.clone());
        } else if !self.inner.is_free() {
            return Err(ErrorKind::MissingField("payment"))?;
        }

        let mut query = proto::Query::Query::new();
        query.query = Some(self.inner.to_query_proto(header)?);

        Ok(query)
    }
}

// this is needed because some times a query is responded to with the wrong
// envelope type when an error occurs; this ensures we can get the error
pub(crate) fn take_header(
    response: &mut proto::Response::Response,
) -> proto::ResponseHeader::ResponseHeader {
    use self::proto::Response::Response_oneof_response::*;

    match &mut response.response {
        Some(getByKey(ref mut res)) => res.take_header(),
        Some(getBySolidityID(ref mut res)) => res.take_header(),
        Some(contractCallLocal(ref mut res)) => res.take_header(),
        Some(contractGetBytecodeResponse(ref mut res)) => res.take_header(),
        Some(contractGetInfo(ref mut res)) => res.take_header(),
        Some(contractGetRecordsResponse(ref mut res)) => res.take_header(),
        Some(cryptogetAccountBalance(ref mut res)) => res.take_header(),
        Some(cryptoGetAccountRecords(ref mut res)) => res.take_header(),
        Some(cryptoGetInfo(ref mut res)) => res.take_header(),
        Some(cryptoGetClaim(ref mut res)) => res.take_header(),
        Some(cryptoGetProxyStakers(ref mut res)) => res.take_header(),
        Some(fileGetContents(ref mut res)) => res.take_header(),
        Some(fileGetInfo(ref mut res)) => res.take_header(),
        Some(transactionGetReceipt(ref mut res)) => res.take_header(),
        Some(transactionGetRecord(ref mut res)) => res.take_header(),
        Some(transactionGetFastRecord(ref mut res)) => res.take_header(),

        None => unreachable!(),
    }
}
