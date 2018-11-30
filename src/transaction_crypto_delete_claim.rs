use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, Client, ErrorKind, Transaction,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;
use bytes::Bytes;

pub struct TransactionCryptoDeleteClaim {
    accountIDToDeleteFrom: AccountId,
    hashToDelete: Option<Bytes>,
}

interfaces!(
    TransactionCryptoDeleteClaim: Any,
    ToProto<TransactionBody_oneof_data>
);

impl Transaction<TransactionCryptoDeleteClaim> {
    pub fn crypto_delete_claim(client: &Client, id: AccountId) -> Self {
        Self::new(
            client,
            TransactionCryptoDeleteClaim {
                accountIDToDeleteFrom: id,
                hashToDelete: None,
            }
        )
    }

    pub fn hashToDelete(&mut self, hash: Bytes) -> Self {
        self.inner().hashToDelete = Some(hash);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoDeleteClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoDeleteClaim::CryptoDeleteClaimTransactionBody::new();

        dat.set_accountIDToDeleteFrom(self.accountIDToDeleteFrom.to_proto()?);

          match self.hashToDelete {
            Some(hash) => data.set_deleteAccountID(hash.to_proto()?),
            None => Err(ErrorKind::MissingField("hashToDelete"))?,
        }
    }
}