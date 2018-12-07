use crate::proto;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum PreCheckCode {
    /// The transaction passed the pre-check.
    Ok = 0,

    /// The transaction had incorrect syntax or other errors.
    InvalidTransaction = 1,

    // The operator account or node account isn't a valid account number.
    InvalidAccount = 2,

    // The transaction fee is insufficient for this type of transaction.
    InsufficientFee = 3,

    // The operator account has insufficient crypto-currency to pay the transaction fee.
    InsufficientBalance = 4,

    /// This transaction ID is a duplicate of one that was submitted to this node or reached
    /// consensus in the last 180 seconds (receipt period).
    Duplicate = 5,

    /// Too many requests against the API
    Busy = 6,

    /// API is not supported
    NotSupported = 7,
}

impl From<proto::TransactionResponse::NodeTransactionPrecheckCode> for PreCheckCode {
    fn from(code: proto::TransactionResponse::NodeTransactionPrecheckCode) -> Self {
        use self::proto::TransactionResponse::NodeTransactionPrecheckCode::*;

        match code {
            OK => PreCheckCode::Ok,
            INVALID_TRANSACTION => PreCheckCode::InvalidTransaction,
            INVALID_ACCOUNT => PreCheckCode::InvalidAccount,
            INSUFFICIENT_FEE => PreCheckCode::InsufficientFee,
            INSUFFICIENT_BALANCE => PreCheckCode::InsufficientBalance,
            DUPLICATE => PreCheckCode::Duplicate,
            BUSY => PreCheckCode::Busy,
            NOT_SUPPORTED => PreCheckCode::NotSupported,
        }
    }
}
