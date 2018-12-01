use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Client, FileId,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

pub struct TransactionFileAppend {
    id: FileId,
    contents: Vec<u8>,
}

interfaces!(
    TransactionFileAppend: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionFileAppend {
    pub fn new(client: &Client, id: FileId, contents: Vec<u8>) -> Transaction<Self> {
        Transaction::new(client, Self { id, contents })
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionFileAppend {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::FileAppend::FileAppendTransactionBody::new();

        data.set_fileID(self.id.to_proto()?);
        data.set_contents(self.contents.clone());

        Ok(TransactionBody_oneof_data::fileAppend(data))
    }
}
