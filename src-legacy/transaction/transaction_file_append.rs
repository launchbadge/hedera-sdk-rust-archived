use std::any::Any;

use failure::Error;
use query_interface::{interfaces, vtable_for};

use crate::{
    proto::{self, ToProto, Transaction::Transaction_oneof_bodyData},
    transaction::Transaction,
    Client, FileId,
};

pub struct TransactionFileAppend {
    id: FileId,
    contents: Vec<u8>,
}

interfaces!(
    TransactionFileAppend: dyn Any,
    dyn ToProto<Transaction_oneof_bodyData>
);

impl TransactionFileAppend {
    pub fn new(client: &Client, id: FileId, contents: Vec<u8>) -> Transaction<Self> {
        Transaction::new(
            client, 
            Self { 
                id, 
                contents 
            },
        )
    }
}

impl ToProto<Transaction_oneof_bodyData> for TransactionFileAppend {
    fn to_proto(&self) -> Result<Transaction_oneof_bodyData, Error> {
        let mut data = proto::FileAppend::FileAppendTransactionBody::new();

        data.set_fileID(self.id.to_proto()?);
        data.set_contents(self.contents.clone());

        Ok(Transaction_oneof_bodyData::fileAppend(data))
    }
}
