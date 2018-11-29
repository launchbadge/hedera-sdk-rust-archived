use crate::{
    duration::Duration,
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, Client, ErrorKind, PublicKey, Transaction,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

pub struct TransactionCreateAccount {
    key: Option<PublicKey>,
    initial_balance: u64,
    send_record_threshold: i64,
    receive_record_threshold: i64,
    receiver_signature_required: bool,
    proxy_account: Option<AccountId>,
    proxy_fraction: Option<i32>,
    max_receive_proxy_fraction: Option<i32>,
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
                send_record_threshold: i64::max_value(),
                receive_record_threshold: i64::max_value(),
                receiver_signature_required: false,
                proxy_account: None,
                proxy_fraction: None,
                max_receive_proxy_fraction: None,
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

    #[inline]
    pub fn proxy_account(&mut self, account: AccountId) -> &mut Self {
        self.inner().proxy_account = Some(account);
        self
    }

    #[inline]
    pub fn proxy_fraction(&mut self, fraction: i32) -> &mut Self {
        self.inner().proxy_fraction = Some(fraction);
        self
    }

    #[inline]
    pub fn max_receive_proxy_fraction(&mut self, fraction: i32) -> &mut Self {
        self.inner().max_receive_proxy_fraction = Some(fraction);
        self
    }

    /// Create an account record for any transaction withdrawing more than this many tinybars.
    #[inline]
    pub fn send_record_threshold(&mut self, threshold: i64) -> &mut Self {
        debug_assert!(threshold > 0);

        self.inner().send_record_threshold = threshold;
        self
    }

    /// Create an account record for any transaction depositing more than this many tinybars.
    #[inline]
    pub fn receive_record_threshold(&mut self, threshold: i64) -> &mut Self {
        debug_assert!(threshold > 0);

        self.inner().receive_record_threshold = threshold;
        self
    }

    /// If true, this account's key must sign any transaction depositing into this
    /// account (in addition to all withdrawals). This field is immutable; it cannot be
    /// changed by a CryptoUpdate transaction.
    #[inline]
    pub fn receiver_signature_required(&mut self, required: bool) -> &mut Self {
        self.inner().receiver_signature_required = required;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCreateAccount {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoCreate::CryptoCreateTransactionBody::new();
        data.set_initialBalance(self.initial_balance);
        data.set_sendRecordThreshold(self.send_record_threshold as u64);
        data.set_receiveRecordThreshold(self.receive_record_threshold as u64);
        data.set_receiverSigRequired(self.receiver_signature_required);

        if let Some(account) = self.proxy_account {
            data.set_proxyAccountID(account.to_proto()?);
        }

        if let Some(fraction) = self.proxy_fraction {
            data.set_proxyFraction(fraction);
        }

        if let Some(fraction) = self.max_receive_proxy_fraction {
            data.set_maxReceiveProxyFraction(fraction);
        }

        let key = match self.key.as_ref() {
            Some(key) => key,
            None => Err(ErrorKind::MissingField("public_key"))?,
        };

        data.set_key(key.to_proto()?);
        data.set_autoRenewPeriod(Duration::new(2_592_000, 0).to_proto()?);

        Ok(TransactionBody_oneof_data::cryptoCreateAccount(data))
    }
}
