use crate::{
    crypto::PublicKey,
    claim::Claim,
    id::AccountId,
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    transaction::Transaction,
    Client,
    ErrorKind
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

#[derive(Debug)]
pub struct TransactionCryptoAddClaim {
    account: AccountId,
    hash: Option<Vec<u8>>,
    keys: Vec<PublicKey>,
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
                hash: None,
                keys: Vec::new(),
            },
        )
    }
}

impl Transaction<TransactionCryptoAddClaim> {
    #[inline]
    pub fn hash(&mut self, hash: Vec<u8>) -> &mut Self {
        self.inner().hash = Some(hash);
        self
    }

    #[inline]
    pub fn add_key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().keys.push(key);
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionCryptoAddClaim {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoAddClaim::CryptoAddClaimTransactionBody::new();
        data.set_accountID(self.account.to_proto()?);

        let hash = match self.hash.clone() {
            Some(hash) => hash,
            None => Err(ErrorKind::MissingField("hash"))?
        };

        let claim = Claim{
            account: self.account,
            hash,
            keys: self.keys.clone()
        };

        data.set_claim(claim.to_proto()?);

        Ok(TransactionBody_oneof_data::cryptoAddClaim(data))
    }
}
