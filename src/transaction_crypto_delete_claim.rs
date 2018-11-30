use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, Client, ErrorKind, Transaction,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, vec::Vec};

pub struct TransactionCryptoDeleteClaim {
    account_id_to_delete_from: AccountId,
    hash_to_delete: Option<Vec<u8>>,
}

interfaces!(
    TransactionCryptoDeleteClaim: Any,
    ToProto<TransactionBody_oneof_data>
);

impl Transaction<TransactionCryptoDeleteClaim> {
    pub fn crypto_delete_claim(client: &Client, account_id_to_delete_from: AccountId) -> Self {
        Self::new(
            client,
            TransactionCryptoDeleteClaim {
                account_id_to_delete_from,
                hash_to_delete: None,
            },
        )
    }

    pub fn hash_to_delete(&mut self, hash: Vec<u8>) -> &mut Self {
        self.inner().hash_to_delete = Some(hash);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoDeleteClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoDeleteClaim::CryptoDeleteClaimTransactionBody::new();

        data.set_accountIDToDeleteFrom(self.account_id_to_delete_from.to_proto()?);

        match &self.hash_to_delete {
            Some(hash) => data.set_hashToDelete(hash.to_vec()),
            None => Err(ErrorKind::MissingField("hashToDelete"))?,
        };

        Ok(TransactionBody_oneof_data::cryptoDeleteClaim(data))
    }
}
