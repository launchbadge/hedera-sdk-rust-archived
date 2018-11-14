use crate::{
    AccountId, Query, QueryGetAccountBalanceAnswer, QueryGetTransactionReceiptAnswer, Transaction,
    TransactionCreateAccount, TransactionCryptoTransfer, TransactionId,
};
use itertools::Itertools;
use std::sync::Arc;

pub struct Client {
    pub(crate) inner: Arc<grpc::Client>,
}

impl Client {
    pub fn new(address: impl AsRef<str>) -> Self {
        // FIXME: Handle errors
        let (host, port) = address.as_ref().split(':').next_tuple().unwrap();
        let port = port.parse().unwrap();
        let inner = Arc::new(grpc::Client::new_plain(&host, port, Default::default()).unwrap());

        Self { inner }
    }

    pub fn create_account(&self) -> Transaction<TransactionCreateAccount> {
        Transaction::create_account(self)
    }

    pub fn crypto_transfer(&self) -> Transaction<TransactionCryptoTransfer> {
        Transaction::crypto_transfer(self)
    }

    pub fn get_account_balance(&self, account: AccountId) -> Query<QueryGetAccountBalanceAnswer> {
        Query::get_account_balance(self, account)
    }

    pub fn get_transaction_receipt(
        &self,
        transaction: TransactionId,
    ) -> Query<QueryGetTransactionReceiptAnswer> {
        Query::get_transaction_receipt(self, transaction)
    }
}
