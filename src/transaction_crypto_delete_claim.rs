use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, Client, ErrorKind, Transaction,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, vec::Vec};

pub struct TransactionCryptoDeleteClaim {
    account: AccountId,
    hash_to_delete: Option<Vec<u8>>,
}

interfaces!(
    TransactionCryptoDeleteClaim: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionCryptoDeleteClaim {
    pub fn new(client: &Client, account: AccountId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                account,
                hash_to_delete: None,
            },
        )
    }
}

impl Transaction<TransactionCryptoDeleteClaim> {
    pub fn hash_to_delete(&mut self, hash: Vec<u8>) -> &mut Self {
        self.inner().hash_to_delete = Some(hash);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoDeleteClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoDeleteClaim::CryptoDeleteClaimTransactionBody::default();
        data.set_accountIDToDeleteFrom(self.account.to_proto()?);

        match &self.hash_to_delete {
            Some(hash) => data.set_hashToDelete(hash.to_vec()),
            None => Err(ErrorKind::MissingField("hash_to_delete"))?,
        };

        Ok(TransactionBody_oneof_data::cryptoDeleteClaim(data))
    }
}
