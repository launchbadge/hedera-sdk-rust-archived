use crate::{
    proto::{self, ToProto, TransactionBody::TransactionBody_oneof_data},
    transaction::Transaction,
    AccountId, Client,
};
use failure::Error;
use protobuf::RepeatedField;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

impl From<proto::CryptoTransfer::TransferList> for Vec<(AccountId, i64)> {
    fn from(mut transfers: proto::CryptoTransfer::TransferList) -> Self {
        transfers
            .take_accountAmounts()
            .into_iter()
            .map(|mut a| (a.take_accountID().into(), a.get_amount()))
            .collect()
    }
}

pub struct TransactionCryptoTransfer {
    transfers: Vec<(AccountId, i64)>,
}

interfaces!(
    TransactionCryptoTransfer: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionCryptoTransfer {
    pub fn new(client: &Client) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                transfers: Vec::new(),
            },
        )
    }
}

impl Transaction<TransactionCryptoTransfer> {
    #[inline]
    pub fn transfer(&mut self, id: AccountId, amount: i64) -> &mut Self {
        self.inner().transfers.push((id, amount));
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoTransfer {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let amounts: Result<Vec<proto::CryptoTransfer::AccountAmount>, Error> = self
            .transfers
            .iter()
            .map(|(id, amount)| {
                let mut pb = proto::CryptoTransfer::AccountAmount::new();
                pb.set_accountID(id.to_proto()?);
                pb.set_amount(*amount);
                Ok(pb)
            })
            .collect();

        let mut transfers = proto::CryptoTransfer::TransferList::new();
        transfers.set_accountAmounts(RepeatedField::from_vec(amounts?));

        let mut data = proto::CryptoTransfer::CryptoTransferTransactionBody::new();
        data.set_transfers(transfers);

        Ok(TransactionBody_oneof_data::cryptoTransfer(data))
    }
}
