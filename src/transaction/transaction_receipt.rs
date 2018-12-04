use crate::{proto, AccountId, ContractId, FileId};

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TransactionStatus {
    /// Hasn't yet reached consensus, or has already expired.
    Unknown = 0,

    /// The transaction succeeded.
    Success = 1,

    /// The transaction failed because it is invalid.
    FailInvalid = 2,

    /// The transaction fee was insufficient.
    FailFee = 3,

    /// The paying account had insufficient crypto-currency.
    FailBalance = 4,
}

impl From<proto::TransactionReceipt::TransactionStatus> for TransactionStatus {
    fn from(code: proto::TransactionReceipt::TransactionStatus) -> Self {
        use self::proto::TransactionReceipt::TransactionStatus::*;

        match code {
            UNKNOWN => TransactionStatus::Unknown,
            SUCCESS => TransactionStatus::Success,
            FAIL_INVALID => TransactionStatus::FailInvalid,
            FAIL_FEE => TransactionStatus::FailFee,
            FAIL_BALANCE => TransactionStatus::FailBalance,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TransactionReceipt {
    pub status: TransactionStatus,
    pub account_id: Option<Box<AccountId>>,
    pub contract_id: Option<Box<ContractId>>,
    pub file_id: Option<Box<FileId>>,
}

impl From<proto::TransactionReceipt::TransactionReceipt> for TransactionReceipt {
    fn from(mut receipt: proto::TransactionReceipt::TransactionReceipt) -> Self {
        let account_id = if receipt.has_accountID() {
            Some(Box::new(receipt.take_accountID().into()))
        } else {
            None
        };

        let file_id = if receipt.has_fileID() {
            Some(Box::new(receipt.take_fileID().into()))
        } else {
            None
        };

        let contract_id = if receipt.has_contractID() {
            Some(Box::new(receipt.take_contractID().into()))
        } else {
            None
        };

        Self {
            status: receipt.get_status().into(),
            account_id,
            contract_id,
            file_id,
        }
    }
}
