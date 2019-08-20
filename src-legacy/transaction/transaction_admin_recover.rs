use std::any::Any;

use failure::Error;
use query_interface::{interfaces, vtable_for};

use crate::{
    proto::{self, ToProto, Transaction::Transaction_oneof_bodyData},
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
    dyn ToProto<Transaction_oneof_bodyData>
);

impl TransactionAdminFileRecover {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(client, Self { id })
    }
}

impl ToProto<Transaction_oneof_bodyData> for TransactionAdminFileRecover {
    fn to_proto(&self) -> Result<Transaction_oneof_bodyData, Error> {
        let mut data = proto::AdminUndelete::AdminUndeleteTransactionBody::new();
        data.set_fileID(self.id.to_proto()?);

        Ok(Transaction_oneof_bodyData::adminUndelete(data))
    }
}

/// Recover a contract. Requires the operator to be the Hedera administrator.
/// Contract will recovered-deleted as long as it is still pending deletion.
pub struct TransactionAdminContractRecover {
    id: ContractId,
}

interfaces!(
    TransactionAdminContractRecover: dyn Any,
    dyn ToProto<Transaction_oneof_bodyData>
);

impl TransactionAdminContractRecover {
    pub fn new(client: &Client, id: ContractId) -> Transaction<Self> {
        Transaction::new(client, Self { id })
    }
}

impl ToProto<Transaction_oneof_bodyData> for TransactionAdminContractRecover {
    fn to_proto(&self) -> Result<Transaction_oneof_bodyData, Error> {
        let mut data = proto::AdminUndelete::AdminUndeleteTransactionBody::new();
        data.set_contractID(self.id.to_proto()?);

        Ok(Transaction_oneof_bodyData::adminUndelete(data))
    }
}
