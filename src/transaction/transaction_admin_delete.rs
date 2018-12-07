use chrono::{DateTime, Duration, Utc};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Client, ContractId, FileId,
};

/// Delete a file. Requires the operator to be the Hedera administrator.
/// File will be permanently deleted in 1 minute (by default). Before then, it can be recovered
/// with [`TransactionAdminFileRecover`](struct.TransactionAdminFileRecover.html).
pub struct TransactionAdminFileDelete {
    id: FileId,
    exp: DateTime<Utc>,
}

interfaces!(
    TransactionAdminFileDelete: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionAdminFileDelete {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                id,
                exp: Utc::now() + Duration::minutes(1),
            },
        )
    }
}

impl Transaction<TransactionAdminFileDelete> {
    pub fn expire_at(&mut self, time: DateTime<Utc>) -> &mut Self {
        self.inner().exp = time;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionAdminFileDelete {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::AdminDelete::AdminDeleteTransactionBody::new();
        data.set_fileID(self.id.to_proto()?);
        data.set_expirationTime(self.exp.to_proto()?);

        Ok(TransactionBody_oneof_data::adminDelete(data))
    }
}

/// Delete a contract. Requires the operator to be the Hedera administrator.
/// Contract will be permanently deleted in 1 minute (by default). Before then, it can be recovered
/// with [`TransactionAdminContractRecover`](struct.TransactionAdminContractRecover.html).
pub struct TransactionAdminContractDelete {
    id: ContractId,
    exp: DateTime<Utc>,
}

interfaces!(
    TransactionAdminContractDelete: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionAdminContractDelete {
    pub fn new(client: &Client, id: ContractId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                id,
                exp: Utc::now() + Duration::minutes(1),
            },
        )
    }
}

impl Transaction<TransactionAdminContractDelete> {
    pub fn expire_at(&mut self, time: DateTime<Utc>) -> &mut Self {
        self.inner().exp = time;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionAdminContractDelete {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::AdminDelete::AdminDeleteTransactionBody::new();
        data.set_contractID(self.id.to_proto()?);
        data.set_expirationTime(self.exp.to_proto()?);

        Ok(TransactionBody_oneof_data::adminDelete(data))
    }
}
