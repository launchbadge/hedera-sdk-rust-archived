use crate::{
    id::AccountId,
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Claim, Client,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

#[derive(Debug)]
pub struct TransactionCryptoAddClaim {
    account: AccountId,
    claim: Claim,
}

interfaces!(
    TransactionCryptoAddClaim: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionCryptoAddClaim {
    pub fn new(client: &Client, account: AccountId, claim: Claim) -> Transaction<Self> {
        Transaction::new(client, Self { account, claim })
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoAddClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoAddClaim::CryptoAddClaimTransactionBody::new();
        data.set_accountID(self.account.to_proto()?);
        data.set_claim(self.claim.to_proto()?);

        Ok(TransactionBody_oneof_data::cryptoAddClaim(data))
    }
}
