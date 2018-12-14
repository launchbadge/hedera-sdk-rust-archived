use std::any::Any;

use failure::Error;
use query_interface::{interfaces, vtable_for};

use crate::{
    crypto::PublicKey,
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    AccountId, Client,
};
use chrono::{DateTime, Utc};
use std::time::Duration;

pub struct TransactionCryptoUpdate {
    account: AccountId,
    key: Option<PublicKey>,
    proxy_account: Option<AccountId>,
    proxy_fraction: Option<i32>,
    send_record_threshold: Option<u64>,
    receive_record_threshold: Option<u64>,
    auto_renew_period: Option<Duration>,
    expiration_time: Option<DateTime<Utc>>,
}

interfaces!(
    TransactionCryptoUpdate: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionCryptoUpdate {
    pub fn new(client: &Client, id: AccountId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                account: id,
                key: None,
                proxy_account: None,
                proxy_fraction: None,
                send_record_threshold: None,
                receive_record_threshold: None,
                auto_renew_period: None,
                expiration_time: None,
            },
        )
    }
}

impl Transaction<TransactionCryptoUpdate> {
    #[inline]
    pub fn key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().key = Some(key);
        self
    }

    #[inline]
    pub fn proxy_account(&mut self, proxy_account: AccountId) -> &mut Self {
        self.inner().proxy_account = Some(proxy_account);
        self
    }

    #[inline]
    pub fn proxy_fraction(&mut self, proxy_fraction: i32) -> &mut Self {
        self.inner().proxy_fraction = Some(proxy_fraction);
        self
    }

    #[inline]
    pub fn send_record_threshold(&mut self, send_record_threshold: u64) -> &mut Self {
        self.inner().send_record_threshold = Some(send_record_threshold);
        self
    }

    #[inline]
    pub fn receive_record_threshold(&mut self, receive_record_threshold: u64) -> &mut Self {
        self.inner().receive_record_threshold = Some(receive_record_threshold);
        self
    }

    #[inline]
    pub fn auto_renew_period(&mut self, auto_renew_period: Duration) -> &mut Self {
        self.inner().auto_renew_period = Some(auto_renew_period);
        self
    }

    #[inline]
    pub fn expires_at(&mut self, expiration: DateTime<Utc>) -> &mut Self {
        self.inner().expiration_time = Some(expiration);
        self
    }

    #[inline]
    pub fn expires_in(&mut self, duration: Duration) -> &mut Self {
        self.expires_at(Utc::now() + chrono::Duration::from_std(duration).unwrap())
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoUpdate {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoUpdate::CryptoUpdateTransactionBody::new();
        data.set_accountIDToUpdate(self.account.to_proto()?);

        if let Some(key) = self.key.as_ref() {
            data.set_key(key.to_proto()?);
        }

        if let Some(proxy_account) = self.proxy_account.as_ref() {
            data.set_proxyAccountID(proxy_account.to_proto()?);
        }

        if let Some(proxy_fraction) = self.proxy_fraction.as_ref() {
            data.set_proxyFraction(*proxy_fraction);
        }

        if let Some(send_record_threshold) = self.send_record_threshold.as_ref() {
            data.set_sendRecordThreshold(*send_record_threshold);
        }

        if let Some(receive_record_threshold) = self.receive_record_threshold.as_ref() {
            data.set_receiveRecordThreshold(*receive_record_threshold);
        }

        if let Some(auto_renew_period) = self.auto_renew_period.as_ref() {
            data.set_autoRenewPeriod(auto_renew_period.to_proto()?);
        }

        if let Some(expiration_time) = self.expiration_time.as_ref() {
            data.set_expirationTime(expiration_time.to_proto()?);
        }

        Ok(TransactionBody_oneof_data::cryptoUpdateAccount(data))
    }
}
