use crate::{
    proto::{self, ToProto},
    timestamp::Timestamp,
    AccountId,
};
use failure::{err_msg, Error};
use itertools::Itertools;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct TransactionId {
    pub account_id: AccountId,
    pub transaction_valid_start: Timestamp,
}

impl TransactionId {
    pub fn new(account_id: AccountId) -> Self {
        TransactionId {
            account_id,
            transaction_valid_start: Timestamp::new(),
        }
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}@{}", self.account_id, self.transaction_valid_start)
    }
}

impl FromStr for TransactionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (account_id, timestamp) = s.split('@').next_tuple().ok_or_else(|| {
            err_msg("expected string of the format: {realm}:{shard}:{account}@{seconds}.{nanos}")
        })?;

        Ok(Self {
            account_id: account_id.parse()?,
            transaction_valid_start: timestamp.parse()?,
        })
    }
}

impl From<crate::proto::BasicTypes::TransactionID> for TransactionId {
    fn from(mut pb: crate::proto::BasicTypes::TransactionID) -> Self {
        Self {
            account_id: pb.take_accountID().into(),
            transaction_valid_start: pb.take_transactionValidStart().into(),
        }
    }
}

impl ToProto<proto::BasicTypes::TransactionID> for TransactionId {
    fn to_proto(&self) -> proto::BasicTypes::TransactionID {
        let mut id = proto::BasicTypes::TransactionID::new();
        id.set_transactionValidStart(self.transaction_valid_start.to_proto());
        id.set_accountID(self.account_id.to_proto());

        id
    }
}

#[cfg(test)]
mod tests {
    use super::TransactionId;
    use crate::{AccountId, Timestamp};

    #[test]
    fn test_display() {
        let account_id = AccountId::new(7, 5, 1001);
        let transaction_valid_start = Timestamp {
            seconds: 1234567,
            nanos: 10001,
        };
        let transaction_id = TransactionId {
            account_id,
            transaction_valid_start,
        };

        assert_eq!(format!("{}", transaction_id), "7:5:1001@1234567.10001");
    }

    #[test]
    fn test_parse() {
        let account_id = AccountId::new(7, 5, 1001);
        let transaction_valid_start = Timestamp {
            seconds: 1234567,
            nanos: 10001,
        };
        let transaction_id = TransactionId {
            account_id,
            transaction_valid_start,
        };

        assert_eq!(
            "7:5:1001@1234567.10001".parse::<TransactionId>().unwrap(),
            transaction_id
        );
    }
}
