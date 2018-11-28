use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    Client, Duration, ErrorKind, PublicKey, Transaction,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

pub struct TransactionCreateAccount {
    key: Option<PublicKey>,
    initial_balance: u64,
    send_record_threshold: u64,
    receive_record_threshold: u64,
}

interfaces!(
    TransactionCreateAccount: Any,
    ToProto<TransactionBody_oneof_data>
);

impl Transaction<TransactionCreateAccount> {
    pub fn create_account(client: &Client) -> Self {
        Self::new(
            client,
            TransactionCreateAccount {
                key: None,
                initial_balance: 0,
                send_record_threshold: u64::max_value(),
                receive_record_threshold: u64::max_value(),
            },
        )
    }

    #[inline]
    pub fn key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().key = Some(key);
        self
    }

    #[inline]
    pub fn initial_balance(&mut self, balance: u64) -> &mut Self {
        self.inner().initial_balance = balance;
        self
    }

    /// Create an account record for any transaction withdrawing more than this many tinybars.
    #[inline]
    pub fn send_record_threshold(&mut self, threshold: u64) -> &mut Self {
        self.inner().send_record_threshold = threshold;
        self
    }

    /// Create an account record for any transaction depositing more than this many tinybars.
    #[inline]
    pub fn receive_record_threshold(&mut self, threshold: u64) -> &mut Self {
        self.inner().receive_record_threshold = threshold;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCreateAccount {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoCreate::CryptoCreateTransactionBody::new();
        data.set_initialBalance(self.initial_balance);
        data.set_sendRecordThreshold(self.send_record_threshold);
        data.set_receiveRecordThreshold(self.receive_record_threshold);

        let key = match self.key.as_ref() {
            Some(key) => key,
            None => Err(ErrorKind::MissingField("public_key"))?,
        };

        data.set_key(key.to_proto()?);
        data.set_autoRenewPeriod(Duration::new(2_592_000, 0).to_proto()?);

        Ok(TransactionBody_oneof_data::cryptoCreateAccount(data))
    }
}
