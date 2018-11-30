use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    AccountId, Client,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::{any::Any, vec::Vec};

pub struct TransactionCryptoDeleteClaim {
    account: AccountId,
    hash_to_delete: Vec<u8>,
}

interfaces!(
    TransactionCryptoDeleteClaim: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionCryptoDeleteClaim {
    pub fn new(client: &Client, account: AccountId, hash: Vec<u8>) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                account,
                hash_to_delete: hash,
            },
        )
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoDeleteClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoDeleteClaim::CryptoDeleteClaimTransactionBody::default();
        data.set_accountIDToDeleteFrom(self.account.to_proto()?);
        // fixme: it'd be nice to avoid the clone here
        data.set_hashToDelete(self.hash_to_delete.clone());

        Ok(TransactionBody_oneof_data::cryptoDeleteClaim(data))
    }
}
