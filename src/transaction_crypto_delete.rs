use std::any::Any;
use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    AccountId, Client, Transaction, ErrorKind
};
use failure::Error;
use query_interface::{interfaces, vtable_for};

pub struct TransactionCryptoDelete {
    transfer_account_id: Option<AccountId>,
    delete_account_id: Option<AccountId>,
}

interfaces!(
    TransactionCryptoDelete: Any,
    ToProto<TransactionBody_oneof_data>
);

impl Transaction<TransactionCryptoDelete> {
    pub fn crypto_delete(client: &Client) -> Self {
        Self::new(
            client,
            TransactionCryptoDelete {
                transfer_account_id: None,
                delete_account_id: None,
            }
        )
    }
    
    pub fn transfer_account_id(&mut self, id: AccountId) {
        self.inner().transfer_account_id = Some(id);
    }
        
    pub fn delete_account_id(&mut self, id: AccountId) {
        self.inner().delete_account_id = Some(id);
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoDelete {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoDelete::CryptoDeleteTransactionBody::new();

        match self.transfer_account_id {
            Some(id) => data.set_transferAccountID(id.to_proto()?),
            None => Err(ErrorKind::MissingField("transferAccountID"))?,
        }

        match self.delete_account_id {
            Some(id) => data.set_deleteAccountID(id.to_proto()?),
            None => Err(ErrorKind::MissingField("deleteAccountID"))?,
        }

        Ok(TransactionBody_oneof_data::cryptoDelete(data))
    }
}