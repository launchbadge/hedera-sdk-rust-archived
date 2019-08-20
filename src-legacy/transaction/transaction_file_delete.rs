use crate::{
    proto::{self, ToProto, Transaction::Transaction_oneof_bodyData},
    transaction::Transaction,
    Client, FileId,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

// Delete the given file. After deletion, it will be marked as deleted and will have no contents.
pub struct TransactionFileDelete {
    id: FileId,
}

interfaces!(
    TransactionFileDelete: dyn Any,
    dyn ToProto<Transaction_oneof_bodyData>
);

impl TransactionFileDelete {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(client, Self { id })
    }
}

impl ToProto<Transaction_oneof_bodyData> for TransactionFileDelete {
    fn to_proto(&self) -> Result<Transaction_oneof_bodyData, Error> {
        let mut data = proto::FileDelete::FileDeleteTransactionBody::new();

        data.set_fileID(self.id.to_proto()?);

        Ok(Transaction_oneof_bodyData::fileDelete(data))
    }
}
