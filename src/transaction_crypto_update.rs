use std::any::Any;

use failure::Error;
use query_interface::{interfaces, vtable_for};

use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, Client, Duration, ErrorKind, PublicKey, Timestamp, Transaction,
};

pub struct TransactionCryptoUpdate {
    account_id_to_update: Option<AccountId>,
    key: Option<PublicKey>,
    proxy_account_id: Option<AccountId>,
    proxy_fraction: Option<i32>,
    send_record_threshold: Option<u64>,
    receive_record_threshold: Option<u64>,
    auto_renew_period: Option<Duration>,
    expiration_time: Option<Timestamp>,
}

interfaces!(
    TransactionCryptoUpdate: Any,
    ToProto<TransactionBody_oneof_data>
);

impl Transaction<TransactionCryptoUpdate> {
    pub fn crypto_update(client: &Client) -> Self {
        Self::new(
            client,
            TransactionCryptoUpdate {
                account_id_to_update: None,
                key: None,
                proxy_account_id: None,
                proxy_fraction: None,
                send_record_threshold: None,
                receive_record_threshold: None,
                auto_renew_period: None,
                expiration_time: None,
            },
        )
    }

    #[inline]
    pub fn account_id_to_update(&mut self, account_id: AccountId) -> &mut Self {
        self.inner().account_id_to_update = Some(account_id);
        self
    }

    #[inline]
    pub fn key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().key = Some(key);
        self
    }

    #[inline]
    pub fn proxy_account_id(&mut self, proxy_account_id: AccountId) -> &mut Self {
        self.inner().proxy_account_id = Some(proxy_account_id);
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
    pub fn expiration_time(&mut self, expiration_time: Timestamp) -> &mut Self {
        self.inner().expiration_time = Some(expiration_time);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoUpdate {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoUpdate::CryptoUpdateTransactionBody::new();

        let account_id = match self.account_id_to_update.as_ref() {
            Some(account_id) => account_id,
            None => Err(ErrorKind::MissingField("account_id_to_update"))?,
        };

        data.set_accountIDToUpdate(account_id.to_proto()?);

        if let Some(key) = self.key.as_ref() {
            data.set_key(key.to_proto()?);
        }

        if let Some(proxy_account_id) = self.proxy_account_id.as_ref() {
            data.set_proxyAccountID(proxy_account_id.to_proto()?);
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
