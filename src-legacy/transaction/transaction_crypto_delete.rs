use crate::{
    proto::{self, ToProto, Transaction::Transaction_oneof_bodyData},
    transaction::Transaction,
    AccountId, Client,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

/// Mark an account as deleted, moving all its current hbars to another account.
/// It will remain in the ledger, marked as deleted, until it expires.
pub struct TransactionCryptoDelete {
    id: AccountId,
    transfer_to: Option<AccountId>,
}

interfaces!(
    TransactionCryptoDelete: dyn Any,
    dyn ToProto<Transaction_oneof_bodyData>
);

impl TransactionCryptoDelete {
    pub fn new(client: &Client, id: AccountId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                transfer_to: None,
                id,
            },
        )
    }
}

impl Transaction<TransactionCryptoDelete> {
    pub fn transfer_to(&mut self, id: AccountId) {
        self.inner().transfer_to = Some(id);
    }
}

impl ToProto<Transaction_oneof_bodyData> for TransactionCryptoDelete {
    fn to_proto(&self) -> Result<Transaction_oneof_bodyData, Error> {
        let mut data = proto::CryptoDelete::CryptoDeleteTransactionBody::new();
        data.set_deleteAccountID(self.id.to_proto()?);

        if let Some(id) = self.transfer_to {
            // note: this is defaulted to the operator from inside [Transaction]
            data.set_transferAccountID(id.to_proto()?);
        }

        Ok(Transaction_oneof_bodyData::cryptoDelete(data))
    }
}
