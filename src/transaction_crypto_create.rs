use crate::{
    crypto::PublicKey,
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    AccountId, Client, ErrorKind,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, convert::TryInto, time::Duration};

pub struct TransactionCryptoCreate {
    key: Option<PublicKey>,
    initial_balance: u64,
    send_record_threshold: i64,
    receive_record_threshold: i64,
    receiver_signature_required: bool,
    proxy_account: Option<AccountId>,
    proxy_fraction: Option<i32>,
    max_receive_proxy_fraction: Option<i32>,
    auto_renew_period: Duration,
}

interfaces!(
    TransactionCryptoCreate: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionCryptoCreate {
    pub fn new(client: &Client) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                key: None,
                initial_balance: 0,
                send_record_threshold: i64::max_value(),
                receive_record_threshold: i64::max_value(),
                receiver_signature_required: false,
                proxy_account: None,
                proxy_fraction: None,
                max_receive_proxy_fraction: None,
                auto_renew_period: Duration::from_secs(2_592_000),
            },
        )
    }
}

impl Transaction<TransactionCryptoCreate> {
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

    /// The account is charged to extend its expiration date every this many seconds.
    /// If it doesn't have enough, it extends as long as possible.
    /// If it is empty when it expires, then it is deleted.
    ///
    /// Defaults to `2_592_000` seconds.
    #[inline]
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.inner().auto_renew_period = period;
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

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoCreate {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoCreate::CryptoCreateTransactionBody::new();

        let mut shard = proto::BasicTypes::ShardID::new();
        shard.set_shardNum(0);
        data.set_shardID(shard);

        let mut realm = proto::BasicTypes::RealmID::new();
        realm.set_realmNum(0);
        data.set_realmID(realm);

        data.set_initialBalance(self.initial_balance);
        data.set_sendRecordThreshold(self.send_record_threshold.try_into()?);
        data.set_receiveRecordThreshold(self.receive_record_threshold.try_into()?);
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
        data.set_autoRenewPeriod(self.auto_renew_period.to_proto()?);

        Ok(TransactionBody_oneof_data::cryptoCreateAccount(data))
    }
}
