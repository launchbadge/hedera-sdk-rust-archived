use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

use crate::{
    proto::{self, ToProto, TransactionBody::TransactionBody_oneof_data},
    transaction::Transaction,
    Client, ContractId, AccountId
};

pub struct TransactionContractDelete {
    id: ContractId,
    obtainer_account: Option<AccountId>,
}

interfaces!(
    TransactionContractDelete: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionContractDelete {
    pub fn new(client: &Client, id: ContractId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                id,
                obtainer_account: None
            },
        )
    }
}

impl Transaction<TransactionContractDelete> {
    #[inline]
    pub fn obtainer_account(&mut self, acct: AccountId) -> &mut Self {
        self.inner().obtainer_account = Some(acct);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionContractDelete {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::ContractDelete::ContractDeleteTransactionBody::new();
        data.set_contractID(self.id.to_proto()?);

        if let Some(account) = self.obtainer_account {
            data.set_transferAccountID(account.to_proto()?);
        }

        Ok(TransactionBody_oneof_data::contractDeleteInstance(data))
    }
}
