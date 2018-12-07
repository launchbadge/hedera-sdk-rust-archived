use std::any::Any;

use failure::Error;
use query_interface::{interfaces, vtable_for};

use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Client, ContractId, FileId,
};

/// Recover a file. Requires the operator to be the Hedera administrator.
/// File will be recovered as long as it is still pending deletion.
pub struct TransactionAdminFileRecover {
    id: FileId,
}

interfaces!(
    TransactionAdminFileRecover: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionAdminFileRecover {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(client, Self { id })
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionAdminFileRecover {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::AdminUndelete::AdminUndeleteTransactionBody::new();
        data.set_fileID(self.id.to_proto()?);

        Ok(TransactionBody_oneof_data::adminUndelete(data))
    }
}

/// Recover a contract. Requires the operator to be the Hedera administrator.
/// Contract will recovered-deleted as long as it is still pending deletion.
pub struct TransactionAdminContractRecover {
    id: ContractId,
}

interfaces!(
    TransactionAdminContractRecover: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionAdminContractRecover {
    pub fn new(client: &Client, id: ContractId) -> Transaction<Self> {
        Transaction::new(client, Self { id })
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionAdminContractRecover {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::AdminUndelete::AdminUndeleteTransactionBody::new();
        data.set_contractID(self.id.to_proto()?);

        Ok(TransactionBody_oneof_data::adminUndelete(data))
    }
}
