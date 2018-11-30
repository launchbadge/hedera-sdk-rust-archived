use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, Client, ErrorKind, Transaction,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;
use std::vec::Vec;

pub struct TransactionCryptoDeleteClaim {
    accountIDToDeleteFrom: AccountId,
    hashToDelete: Option<Vec<u8>>,
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

    pub fn hashToDelete(&mut self, hash: Vec<u8>) -> &mut Self {
        self.inner().hashToDelete = Some(hash);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoDeleteClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoDeleteClaim::CryptoDeleteClaimTransactionBody::new();

        data.set_accountIDToDeleteFrom(self.accountIDToDeleteFrom.to_proto()?);

          match self.hashToDelete {
            Some(hash) => data.set_hashToDelete(hash),
            None => Err(ErrorKind::MissingField("hashToDelete"))?,
        };

        Ok(TransactionBody_oneof_data::cryptoDeleteClaim(data))
    }
}