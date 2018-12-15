use crate::{
    claim::Claim,
    id::AccountId,
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Client,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

#[derive(Debug)]
pub struct TransactionCryptoAddClaim {
    account: AccountId,
    claim: Option<Claim>,
}

interfaces!(
    TransactionCryptoAddClaim: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionCryptoAddClaim {
    pub fn new(client: &Client, account: AccountId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                account,
                claim: None,
            },
        )
    }
}

impl Transaction<TransactionCryptoAddClaim> {
    #[inline]
    pub fn claim(&mut self, claim: Claim) -> &mut Self {
        self.inner().claim = Some(claim);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoAddClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoAddClaim::CryptoAddClaimTransactionBody::new();
        data.set_accountID(self.account.to_proto()?);

        if let Some(claim) = &self.claim {
            data.set_claim(claim.to_proto()?);
        }

        Ok(TransactionBody_oneof_data::cryptoAddClaim(data))
    }
}
