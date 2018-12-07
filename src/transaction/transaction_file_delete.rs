use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
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
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionFileDelete {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(client, Self { id })
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionFileDelete {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::FileDelete::FileDeleteTransactionBody::new();

        data.set_fileID(self.id.to_proto()?);

        Ok(TransactionBody_oneof_data::fileDelete(data))
    }
}
