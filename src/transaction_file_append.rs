use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Client,
    FileId,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

// Delete the given file. After deletion, it will be marked as deleted and will have no contents.
pub struct TransactionFileAppend {
    id: FileId,
    bytes: Vec<u8>,
}

interfaces!(
    TransactionFileAppend: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionFileAppend {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                id,
                bytes: Vec::new(),
            },
        )
    }
}

impl Transaction<TransactionFileAppend> {
    pub fn file(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.inner().bytes = bytes;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionFileAppend {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::FileAppend::FileAppendTransactionBody::new();

        data.set_fileID(self.id.to_proto()?);
        data.set_contents(self.bytes.clone());

        Ok(TransactionBody_oneof_data::fileAppend(data))
    }
}
