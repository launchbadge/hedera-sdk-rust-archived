use crate::{
    AccountId, Query, QueryGetAccountBalance, QueryGetTransactionReceipt, Transaction,
    TransactionCreateAccount, TransactionCryptoTransfer, TransactionId,
};
use std::sync::Arc;
use itertools::Itertools;

pub struct Client {
    pub(crate) inner: Arc<grpc::Client>,
}

impl Client {
    pub fn new(address: impl AsRef<str>) -> Self {
        // FIXME: Handle errors
        let (host, port) = address.as_ref().split(':').next_tuple().unwrap();
        let port = port.parse().unwrap();
        let inner =
            Arc::new(grpc::Client::new_plain(&host, port, Default::default()).unwrap());

//        let env = Arc::new(EnvBuilder::new().build());
//        let ch = ChannelBuilder::new(env).connect(address.as_ref());

        Self { inner }
    }

    pub fn create_account(&self) -> Transaction<TransactionCreateAccount> {
        Transaction::create_account(self)
    }

    pub fn crypto_transfer(&self) -> Transaction<TransactionCryptoTransfer> {
        Transaction::crypto_transfer(self)
    }

    pub fn get_account_balance(&self, account: AccountId) -> Query<QueryGetAccountBalance> {
        Query::get_account_balance(self, account)
    }

    pub fn get_transaction_receipt(
        &self,
        transaction: TransactionId,
    ) -> Query<QueryGetTransactionReceipt> {
        Query::get_transaction_receipt(self, transaction)
    }
}
