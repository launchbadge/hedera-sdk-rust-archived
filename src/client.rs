use crate::{
    AccountId, Query, QueryGetAccountBalance, QueryGetTransactionReceipt, Transaction,
    TransactionCreateAccount, TransactionCryptoTransfer, TransactionId,
};
use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::sync::Arc;

pub struct Client {
    pub(crate) channel: Channel,
}

impl Client {
    pub fn new(address: impl AsRef<str>) -> Self {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(address.as_ref());

        Self { channel: ch }
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
