use crate::{
    crypto::PublicKey,
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, FileId,
};

use crate::{transaction::Transaction, Client};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, time::Duration};

pub struct TransactionContractCreate {
    file_id: Option<FileId>,
    admin_key: Option<PublicKey>,
    gas: i64,
    initial_balance: i64,
    proxy_account: Option<AccountId>,
    proxy_fraction: Option<i32>,
    auto_renew_period: Duration,
    constructor_parameters: Option<Vec<u8>>,
}

interfaces!(
    TransactionContractCreate: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionContractCreate {
    pub fn new(client: &Client) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                file_id: None,
                admin_key: None,
                gas: 0,
                initial_balance: 0,
                proxy_account: None,
                proxy_fraction: None,
                auto_renew_period: Duration::from_secs(2_592_000),
                constructor_parameters: None,
            },
        )
    }
}

impl Transaction<TransactionContractCreate> {
    #[inline]
    pub fn file(&mut self, id: FileId) -> &mut Self {
        self.inner().file_id = Some(id);
        self
    }

    #[inline]
    pub fn gas(&mut self, gas: i64) -> &mut Self {
        self.inner().gas = gas;
        self
    }

    #[inline]
    pub fn admin_key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().admin_key = Some(key);
        self
    }

    #[inline]
    pub fn initial_balance(&mut self, balance: i64) -> &mut Self {
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
    pub fn auto_renew_period(&mut self, period: Duration) -> &mut Self {
        self.inner().auto_renew_period = period;
        self
    }

    #[inline]
    pub fn constructor_parameters(&mut self, params: Vec<u8>) -> &mut Self {
        self.inner().constructor_parameters = Some(params);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionContractCreate {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::ContractCreate::ContractCreateTransactionBody::new();

        let mut shard = proto::BasicTypes::ShardID::new();
        shard.set_shardNum(0);
        data.set_shardID(shard);

        let mut realm = proto::BasicTypes::RealmID::new();
        realm.set_realmNum(0);
        data.set_realmID(realm);

        data.set_initialBalance(self.initial_balance);

        if let Some(account) = self.proxy_account {
            data.set_proxyAccountID(account.to_proto()?);
        }

        if let Some(fraction) = self.proxy_fraction {
            data.set_proxyFraction(fraction);
        }

        if let Some(id) = self.file_id {
            data.set_fileID(id.to_proto()?);
        }

        if let Some(key) = &self.admin_key {
            data.set_adminKey(key.to_proto()?);
        }

        data.set_autoRenewPeriod(self.auto_renew_period.to_proto()?);

        data.set_gas(self.gas);

        if let Some(params) = &self.constructor_parameters {
            data.set_constructorParameters(params.clone());
        }

        Ok(TransactionBody_oneof_data::contractCreateInstance(data))
    }
}
