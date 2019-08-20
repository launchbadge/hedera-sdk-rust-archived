use crate::{
    crypto::PublicKey,
    proto::{self, ToProto},
    AccountId,
};
use failure::Error;
use protobuf::RepeatedField;
use try_from::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct Claim {
    pub account: AccountId,
    pub hash: Vec<u8>,
    pub keys: Vec<PublicKey>,
}

impl TryFrom<proto::CryptoAddClaim::Claim> for Claim {
    type Err = Error;

    fn try_from(mut claim: proto::CryptoAddClaim::Claim) -> Result<Self, Error> {
        Ok(Self {
            account: claim.take_accountID().into(),
            hash: claim.take_hash(),
            keys: claim
                .take_keys()
                .take_keys()
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl ToProto<proto::CryptoAddClaim::Claim> for Claim {
    fn to_proto(&self) -> Result<proto::CryptoAddClaim::Claim, Error> {
        let mut claim = proto::CryptoAddClaim::Claim::new();
        claim.set_accountID(self.account.to_proto()?);
        claim.set_hash(self.hash.clone());

        let mut keys = proto::BasicTypes::KeyList::new();
        keys.set_keys(RepeatedField::from_vec(
            self.keys
                .iter()
                .map(ToProto::to_proto)
                .collect::<Result<Vec<_>, _>>()?,
        ));
        claim.set_keys(keys);

        Ok(claim)
    }
}
