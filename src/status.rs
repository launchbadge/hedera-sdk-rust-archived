use crate::proto;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Status {
    // the transaction passed the precheck
    Ok = 0,

    // For any error not handled by specific error codes listed below.
    InvalidTransaction = 1,

    // Payer account does not exist.
    PayerAccountNotFound = 2,

    // Node Account provided does not match the node account of the node the transaction was submitted to.
    InvalidNodeAccount = 3,

    // Pre-Check TransactionValidStart + transactionValidDuration is less than current consensus time.
    TransactionExpired = 4,

    // Transaction start time is greater than current consensus time
    InvalidTransactionStart = 5,

    // valid transaction duration is a positive non zero number that does not exceed 120 seconds
    InvalidTransactionDuration = 6,

    // the transaction signature is not valid
    InvalidSignature = 7,

    // Transaction memo size exceeded 100 bytes
    MemoTooLong = 8,

    // the transaction fee is insufficient for this type of transaction
    InsufficientTxFee = 9,

    // the payer account has insufficient cryptocurrency to pay the transaction fee
    InsufficientPayerBalance = 10,

    // This transaction ID is a duplicate of one that was submitted to this node or reached consensus in the last 180 seconds (receipt period).
    DuplicateTransaction = 11,

    // If API is throttled out
    Busy = 12,

    // not supported API
    NotSupported = 13,

    // the file id is invalid or does not exist
    InvalidFileId = 14,

    //the account id is invalid or does not exist
    InvalidAccountId = 15,

    //the contract id is invalid or does ont exist
    InvalidContractId = 16,

    //transaction id is not valid
    InvalidTransactionId = 17,

    //receipt for given transaction id does not exist
    ReceiptNotFound = 18,

    //record for given transaction id does not exist
    RecordNotFound = 19,

    //the solidity id is invalid or entity with this solidity id does not exist
    InvalidSolidityId = 20,

    // hasn't yet reached consensus, or has already expired
    Unknown = 21,

    // the transaction succeeded
    Success = 22,

    // the transaction failed because it is invalid
    FailInvalid = 23,

    // the transaction fee was insufficient
    FailFee = 24,

    // the paying account had insufficient cryptocurrency
    FailBalance = 25,

    // Key not provided in the transaction body
    KeyRequired = 26,

    // Unsupported algorithm/encoding used for keys in the transaction
    BadEncoding = 27,

    // When the account balance is not sufficient for the transfer
    InsufficientAccountBalance = 28,

    //During an update transaction when the system is not able to find the Users Solidity address
    InvalidSolidityAddress = 29,

    //Not enough gas was supplied to execute tranasction
    InsufficientGas = 30,

    //contract byte code size is over the limit
    ContractSizeLimitExceeded = 31,

    //local execution (query) is requested for a function which changes state
    LocalCallModificationException = 32,

    //Contract REVERT OPCODE executed
    ContractRevertExecuted = 33,

    //For any contract execution related error not handled by specific error codes listed above.
    ContractExecutionException = 34,

    //In Query validation, account with +ve(amount) value should be Receiving node account, the receiver account should be only one account in the list
    InvalidReceivingNodeAccount = 35,

    // Header is missing in Query request
    MissingQueryHeader = 36,

    // the update of the account failed
    AccountUpdateFailed = 37,

    InvalidKeyEncoding = 38,
    // null solidity address
    NullSolidityAddress = 39,

    // update of the contract failed
    ContractUpdateFailed = 40,

    // the query header is invalid
    InvalidQueryHeader = 41,

    // Invalid fee submitted*/
    InvalidFeeSubmitted = 42,

    //  payer signature is invalid
    InvalidPayerSignature = 43,

    KeyNotProvided = 44,
    InvalidExpirationTime = 45,
    NoWaclKey = 46,
    FileContentEmpty = 47,

    // The crypto transfer credit and debit don't equal to 0
    InvalidAccountAmounts = 48,

    // transaction body is empty
    EmptyTransactionBody = 49,

    // invalid transaction body
    InvalidTransactionBody = 50,
}

impl From<proto::ResponseCode::ResponseCodeEnum> for Status {
    fn from(code: proto::ResponseCode::ResponseCodeEnum) -> Self {
        use self::proto::ResponseCode::ResponseCodeEnum::*;

        match code {
            OK => Status::Ok,
            INVALID_TRANSACTION => Status::InvalidTransaction,
            PAYER_ACCOUNT_NOT_FOUND => Status::PayerAccountNotFound,
            INVALID_NODE_ACCOUNT => Status::InvalidNodeAccount,
            TRANSACTION_EXPIRED => Status::TransactionExpired,
            INVALID_TRANSACTION_START => Status::InvalidTransactionStart,
            INVALID_TRANSACTION_DURATION => Status::InvalidTransactionDuration,
            INVALID_SIGNATURE => Status::InvalidSignature,
            MEMO_TOO_LONG => Status::MemoTooLong,
            INSUFFICIENT_TX_FEE => Status::InsufficientTxFee,
            INSUFFICIENT_PAYER_BALANCE => Status::InsufficientPayerBalance,
            DUPLICATE_TRANSACTION => Status::DuplicateTransaction,
            BUSY => Status::Busy,
            NOT_SUPPORTED => Status::NotSupported,
            INVALID_FILE_ID => Status::InvalidFileId,
            INVALID_ACCOUNT_ID => Status::InvalidAccountId,
            INVALID_CONTRACT_ID => Status::InvalidContractId,
            INVALID_TRANSACTION_ID => Status::InvalidTransactionId,
            RECEIPT_NOT_FOUND => Status::ReceiptNotFound,
            RECORD_NOT_FOUND => Status::RecordNotFound,
            INVALID_SOLIDITY_ID => Status::InvalidSolidityId,
            UNKNOWN => Status::Unknown,
            SUCCESS => Status::Success,
            FAIL_INVALID => Status::FailInvalid,
            FAIL_FEE => Status::FailFee,
            FAIL_BALANCE => Status::FailBalance,
            KEY_REQUIRED => Status::KeyRequired,
            BAD_ENCODING => Status::BadEncoding,
            INSUFFICIENT_ACCOUNT_BALANCE => Status::InsufficientAccountBalance,
            INVALID_SOLIDITY_ADDRESS => Status::InvalidSolidityAddress,
            INSUFFICIENT_GAS => Status::InsufficientGas,
            CONTRACT_SIZE_LIMIT_EXCEEDED => Status::ContractSizeLimitExceeded,
            LOCAL_CALL_MODIFICATION_EXCEPTION => Status::LocalCallModificationException,
            CONTRACT_REVERT_EXECUTED => Status::ContractRevertExecuted,
            CONTRACT_EXECUTION_EXCEPTION => Status::ContractExecutionException,
            INVALID_RECEIVING_NODE_ACCOUNT => Status::InvalidReceivingNodeAccount,
            MISSING_QUERY_HEADER => Status::MissingQueryHeader,
            ACCOUNT_UPDATE_FAILED => Status::AccountUpdateFailed,
            INVALID_KEY_ENCODING => Status::InvalidKeyEncoding,
            NULL_SOLIDITY_ADDRESS => Status::NullSolidityAddress,
            CONTRACT_UPDATE_FAILED => Status::ContractUpdateFailed,
            INVALID_QUERY_HEADER => Status::InvalidQueryHeader,
            INVALID_FEE_SUBMITTED => Status::InvalidFeeSubmitted,
            INVALID_PAYER_SIGNATURE => Status::InvalidPayerSignature,
            KEY_NOT_PROVIDED => Status::KeyNotProvided,
            INVALID_EXPIRATION_TIME => Status::InvalidExpirationTime,
            NO_WACL_KEY => Status::NoWaclKey,
            FILE_CONTENT_EMPTY => Status::FileContentEmpty,
            INVALID_ACCOUNT_AMOUNTS => Status::InvalidAccountAmounts,
            EMPTY_TRANSACTION_BODY => Status::EmptyTransactionBody,
            INVALID_TRANSACTION_BODY => Status::InvalidTransactionBody,
        }
    }
}
