use crate::{
    proto::{self, ToProto},
    Client, Duration, ErrorKind, PublicKey, Transaction,
};
use failure::Error;

pub struct TransactionCreateAccount {
    key: Option<PublicKey>,
    initial_balance: u64,
}

impl Transaction<TransactionCreateAccount> {
    pub fn create_account(client: &Client) -> Self {
        Self::new(
            client,
            TransactionCreateAccount {
                key: None,
                initial_balance: 0,
            },
        )
    }

    pub fn key(&mut self, key: PublicKey) -> &mut Self {
        self.inner.key = Some(key);
        self
    }

    pub fn initial_balance(&mut self, balance: u64) -> &mut Self {
        self.inner.initial_balance = balance;
        self
    }
}

impl ToProto<proto::Transaction::TransactionBody_oneof_data> for TransactionCreateAccount {
    fn to_proto(&self) -> Result<proto::Transaction::TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoCreate::CryptoCreateTransactionBody::new();
        data.set_initialBalance(self.initial_balance);

        let key = match self.key.as_ref() {
            Some(key) => key,
            None => Err(ErrorKind::MissingField("public_key"))?,
        };

        data.set_key(key.to_proto()?);
        data.set_autoRenewPeriod(Duration::new(2_592_000, 0).to_proto()?);

        Ok(proto::Transaction::TransactionBody_oneof_data::cryptoCreateAccount(data))
    }
}
