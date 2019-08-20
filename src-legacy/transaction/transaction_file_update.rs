use crate::{
    crypto::PublicKey,
    proto::{self, ToProto, Transaction::Transaction_oneof_bodyData},
    transaction::Transaction,
    Client, FileId,
};
use chrono::{DateTime, Utc};
use failure::Error;
use protobuf::RepeatedField;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, time::Duration};

pub struct TransactionFileUpdate {
    id: FileId,
    expiration_time: Option<DateTime<Utc>>,
    keys: Vec<PublicKey>,
    bytes: Vec<u8>,
}

interfaces!(
    TransactionFileUpdate: dyn Any,
    dyn ToProto<Transaction_oneof_bodyData>
);

impl TransactionFileUpdate {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                id,
                expiration_time: None,
                keys: Vec::new(),
                bytes: Vec::new(),
            },
        )
    }
}

impl Transaction<TransactionFileUpdate> {
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
        self.inner().keys.push(key);
        self
    }

    #[inline]
    pub fn contents(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.inner().bytes = bytes;
        self
    }
}

impl ToProto<Transaction_oneof_bodyData> for TransactionFileUpdate {
    fn to_proto(&self) -> Result<Transaction_oneof_bodyData, Error> {
        let mut data = proto::FileUpdate::FileUpdateTransactionBody::new();

        data.set_fileID(self.id.to_proto()?);

        if let Some(expiration_time) = &self.expiration_time.as_ref() {
            data.set_expirationTime(expiration_time.to_proto()?);
        }

        let mut key_list = proto::BasicTypes::KeyList::new();
        key_list.set_keys(RepeatedField::from_vec(
            self.keys
                .iter()
                .map(ToProto::to_proto)
                .collect::<Result<Vec<_>, _>>()?,
        ));

        data.set_keys(key_list);

        data.set_contents(self.bytes.clone());

        Ok(Transaction_oneof_bodyData::fileUpdate(data))
    }
}
