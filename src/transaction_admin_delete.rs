use chrono::{DateTime, Utc};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    Client, ContractId, FileId, Transaction,
};

#[repr(C)]
pub enum TransactionAdminDeleteId {
    File(FileId),
    Contract(ContractId),
}

pub struct TransactionAdminDelete {
    id: TransactionAdminDeleteId,
    expiration_time: DateTime<Utc>,
}

interfaces!(
    TransactionAdminDelete: Any,
    ToProto<TransactionBody_oneof_data>
);

impl Transaction<TransactionAdminDelete> {
    pub fn admin_delete(client: &Client, id: TransactionAdminDeleteId) -> Self {
        // NOTE: This should never fail
        let future = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(1))
            .unwrap();

        Self::new(
            client,
            TransactionAdminDelete {
                id,
                expiration_time: future,
            },
        )
    }

    pub fn expiration(&mut self, time: DateTime<Utc>) -> &mut Self {
        self.inner().expiration_time = time;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionAdminDelete {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::AdminDelete::AdminDeleteTransactionBody::new();

        match self.id {
            TransactionAdminDeleteId::File(id) => data.set_fileID(id.to_proto()?),
            TransactionAdminDeleteId::Contract(id) => data.set_contractID(id.to_proto()?),
        }

        data.set_expirationTime(self.expiration_time.to_proto()?);

        Ok(TransactionBody_oneof_data::adminDelete(data))
    }
}
