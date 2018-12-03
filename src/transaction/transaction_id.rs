use chrono::{DateTime, Duration, Utc};
use failure::Error;
use itertools::Itertools;
use std::{fmt, str::FromStr};

use crate::{
    error::ErrorKind,
    proto::{self, ToProto},
    AccountId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionId {
    pub account_id: AccountId,
    pub transaction_valid_start: DateTime<Utc>,
}

impl TransactionId {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            account_id,
            // Allows the transaction to be accepted as long as the
            // server is not more than 10 seconds behind us
            transaction_valid_start: Utc::now() - Duration::seconds(10),
        }
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}@{}.{}",
            self.account_id,
            self.transaction_valid_start.timestamp(),
            self.transaction_valid_start.timestamp_subsec_nanos()
        )
    }
}

impl FromStr for TransactionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::timestamp::Timestamp;

        if let Some((account_id, timestamp)) = s.split('@').next_tuple() {
            Ok(Self {
                account_id: account_id.parse()?,
                transaction_valid_start: Timestamp::from_str(timestamp)?.into(),
            })
        } else {
            let b = hex::decode(s)?;

            let mut pb: crate::proto::BasicTypes::TransactionID =
                protobuf::parse_from_bytes(b.as_slice())
                    .map_err(|_| ErrorKind::Parse("{realm}:{shard}:{account}@{seconds}.{nanos}"))?;

            Ok(Self {
                account_id: pb.take_accountID().into(),
                transaction_valid_start: pb.take_transactionValidStart().into(),
            })
        }
    }
}

impl From<proto::BasicTypes::TransactionID> for TransactionId {
    fn from(mut pb: proto::BasicTypes::TransactionID) -> Self {
        let transaction_valid_start = pb.take_transactionValidStart().into();
        let account_id = pb.take_accountID().into();

        Self {
            transaction_valid_start,
            account_id,
        }
    }
}

impl ToProto<proto::BasicTypes::TransactionID> for TransactionId {
    fn to_proto(&self) -> Result<proto::BasicTypes::TransactionID, Error> {
        let mut id = proto::BasicTypes::TransactionID::new();
        id.set_transactionValidStart(self.transaction_valid_start.to_proto()?);
        id.set_accountID(self.account_id.to_proto()?);

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use failure::Error;

    use crate::{timestamp::Timestamp, AccountId};

    use super::TransactionId;

    #[test]
    fn test_display() {
        let account_id = AccountId::new(7, 5, 1001);
        let transaction_valid_start = Timestamp(1234567, 10001).into();
        let transaction_id = TransactionId {
            account_id,
            transaction_valid_start,
        };

        assert_eq!(format!("{}", transaction_id), "7:5:1001@1234567.10001");
    }

    #[test]
    fn test_parse() -> Result<(), Error> {
        let account_id = AccountId::new(7, 5, 1001);
        let transaction_valid_start = Timestamp(1234567, 10001).into();
        let transaction_id = TransactionId {
            account_id,
            transaction_valid_start,
        };

        assert_eq!(
            "7:5:1001@1234567.10001".parse::<TransactionId>()?,
            transaction_id
        );

        Ok(())
    }

    #[test]
    fn test_parse_encoded() -> Result<(), Error> {
        let account_id = AccountId::new(0, 0, 2);
        let transaction_valid_start = Timestamp(1539387985, 758025699).into();
        let transaction_id = TransactionId {
            account_id,
            transaction_valid_start,
        };

        assert_eq!(
            "0a0c08d1e484de0510e39bbae90212021802".parse::<TransactionId>()?,
            transaction_id
        );

        Ok(())
    }
}
