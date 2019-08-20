use crate::{
    crypto::PublicKey,
    proto::{self, ToProto, Transaction::Transaction_oneof_bodyData},
    transaction::Transaction,
    Client, ErrorKind,
};
use chrono::{DateTime, Utc};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, time::Duration};

pub struct TransactionFileCreate {
    expiration_time: Option<DateTime<Utc>>,
    key: Option<PublicKey>,
    bytes: Vec<u8>,
}

interfaces!(
    TransactionFileCreate: dyn Any,
    dyn ToProto<Transaction_oneof_bodyData>
);

impl TransactionFileCreate {
    pub fn new(client: &Client) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                expiration_time: None,
                key: None,
                bytes: Vec::new(),
            },
        )
    }
}

impl Transaction<TransactionFileCreate> {
    #[inline]
    pub fn expires_at(&mut self, expiration: DateTime<Utc>) -> &mut Self {
        self.inner().expiration_time = Some(expiration);
        self
    }

    #[inline]
    pub fn expires_in(&mut self, duration: Duration) -> &mut Self {
        self.expires_at(Utc::now() + chrono::Duration::from_std(duration).unwrap())
    }

    #[inline]
    pub fn key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().key = Some(key);
        self
    }

    #[inline]
    pub fn contents(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.inner().bytes = bytes;
        self
    }
}

impl ToProto<Transaction_oneof_bodyData> for TransactionFileCreate {
    fn to_proto(&self) -> Result<Transaction_oneof_bodyData, Error> {
        let mut data = proto::FileCreate::FileCreateTransactionBody::new();

        let mut shard = proto::BasicTypes::ShardID::new();
        shard.set_shardNum(0);
        data.set_shardID(shard);

        let mut realm = proto::BasicTypes::RealmID::new();
        realm.set_realmNum(0);
        data.set_realmID(realm);

        if let Some(expiration_time) = &self.expiration_time.as_ref() {
            data.set_expirationTime(expiration_time.to_proto()?);
        }

        let key = match self.key.as_ref() {
            Some(key) => key,
            None => Err(ErrorKind::MissingField("key"))?,
        };

        let mut key_list = proto::BasicTypes::KeyList::new();
        key_list.keys.push(key.to_proto()?);

        data.set_keys(key_list);
        data.set_contents(self.bytes.clone());

        Ok(Transaction_oneof_bodyData::fileCreate(data))
    }
}
