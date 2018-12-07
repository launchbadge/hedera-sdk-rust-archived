use crate::{
    crypto::PublicKey,
    id::{AccountId, ContractId, FileId},
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Client,
};
use chrono::{DateTime, Utc};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, time::Duration};

pub struct TransactionContractUpdate {
    contract: ContractId,
    expiration_time: Option<DateTime<Utc>>,
    admin_key: Option<PublicKey>,
    proxy_account: Option<AccountId>,
    auto_renew_period: Option<Duration>,
    file: Option<FileId>,
}

interfaces!(
    TransactionContractUpdate: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionContractUpdate {
    pub fn new(client: &Client, contract: ContractId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                contract,
                expiration_time: None,
                admin_key: None,
                proxy_account: None,
                auto_renew_period: None,
                file: None,
            },
        )
    }
}

impl Transaction<TransactionContractUpdate> {
    #[inline]
    pub fn expires_at(&mut self, expiration: DateTime<Utc>) -> &mut Self {
        self.inner().expiration_time = Some(expiration);
        self
    }

    #[inline]
    pub fn admin_key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().admin_key = Some(key);
        self
    }

    #[inline]
    pub fn proxy_account(&mut self, account: AccountId) -> &mut Self {
        self.inner().proxy_account = Some(account);
        self
    }

    #[inline]
    pub fn auto_renew_period(&mut self, duration: Duration) -> &mut Self {
        self.inner().auto_renew_period = Some(duration);
        self
    }

    #[inline]
    pub fn file(&mut self, file: FileId) -> &mut Self {
        self.inner().file = Some(file);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionContractUpdate {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::ContractUpdate::ContractUpdateTransactionBody::new();
        data.set_contractID(self.contract.to_proto()?);

        if let Some(time) = self.expiration_time.as_ref() {
            data.set_expirationTime(time.to_proto()?);
        }

        if let Some(key) = self.admin_key.as_ref() {
            data.set_adminKey(key.to_proto()?);
        }

        if let Some(account) = self.proxy_account.as_ref() {
            data.set_proxyAccountID(account.to_proto()?);
        }

        if let Some(duration) = self.auto_renew_period.as_ref() {
            data.set_autoRenewPeriod(duration.to_proto()?);
        }

        if let Some(file) = self.file.as_ref() {
            data.set_fileID(file.to_proto()?);
        }

        Ok(TransactionBody_oneof_data::contractUpdateInstance(data))
    }
}
