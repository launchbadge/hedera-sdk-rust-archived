use crate::proto;
//use crate::status::Status::EmptyClaimHash;
//use test::TestFn::{StaticBenchFn, StaticTestFn};

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

    // invalid signature type
    InvalidSignatureTypeMismatch = 51,

    // amount of signatures does not match
    InvalidSignatureCountMismatch = 52,

    // empty claim bocy
    EmptyClaimBody = 53,

    // empty claim hash
    EmptyClaimHash = 54,

    // empty claim keys
    EmptyClaimKeys = 55,

    // invalid claim hash size
    InvalidClaimHashSize = 56,

    // empty query body
    EmptyQueryBody = 57,

    // claim query is empty
    EmptyClaimQuery = 58,

    // claim does not exist
    ClaimNotFound = 59,

    // account id passed doesn't exist
    AccountIdDoesNotExist = 60,

    // claim has already been created
    ClaimAlreadyExists = 61,

    // file WACL keys are invalid
    InvalidFileWACL = 62,

    // serialization failed
    SerializationFailed = 63,

    // transaction size greater than byte limit
    TransactionOversize = 64,

    // transaction has more than 50 levels
    TransactionTooManyLayers = 65,

    // contract was deleted
    ContractDeleted = 66,

    // platfoem is either disconnected or lagging
    PlatformNotActive = 67,

    // one public key matches multiple signature prefixes
    KeyPrefixMismatch = 68,

    // transaction not created by platform because of backlog or oversize
    TransactionNotCreated = 69,

    // auto renew value must be positive integer
    InvalidRenewalPeriod = 70,

    // smart contract id was passed for crypto tx
    InvalidPayerAccount = 71,

    // account has been deleted
    AccountDeleted = 72,

    // file has been deleted
    FileDeleted = 73,

    // multiple of the same account in the transfer list
    AccountRepeatedInAccountAmounts = 74,

    // attempting to set negative account balance
    SettingNegativeAccountBalance = 75,

    // when deleting smart contract with an account balance either an account or contract is needed
    // obtain the outstanding balance
    ObtainerRequired = 76,

    // cannot use the contract that is being deleted for the obtainer address when delting contract
    ObtainerSameContractId = 77,

    // id passed for obtainer account doesn't exist
    ObtainerDoesNotExist = 78,

    // attempting to modify an immutable contract (ie. created without admin key)
    ModifyingImmutableContract = 79,

    // unexpected occurred during filesystem operation
    FileSystemException = 80,

    // the duration is not a subset of [MINIMUM_AUTORENEW_DURATION,MAXIMUM_AUTORENEW_DURATION]
    AutorenewDurationNotInRange = 81,

    // decoding contract binary to byte array failed, verify input is a valid hex string
    ErrorDecodingBytestring = 82,

    // file to create contract is empty
    ContractFileEmpty = 83,

    // contract file bytecode is empty
    ContractBytecodeEmpty = 84,

    // initial balance must be positive value
    InvalidInitialBalance = 85,

    // receive record threshold must be positive
    InvalidReceiveRecordThreshold = 86,

    // send record threashold must be positive
    InvalidSendRecordThreshold = 87,

    // Special Account Operations must occur from the Genesis Account
    AccountIsNotGenesisAccount = 88,

    // payer account is not authorized for this tx type
    PayerAccountUnauthorized = 89,

    // tx body is invalid
    InvalidFreezeTransactionBody = 90,

    // freeze tx body is empty
    FreezeTransactionBodyNotFound = 91,

    // exceeded the number of accounts (both from and to) allowed for crypto transfer list
    TransferListSizeLimitExceeded = 92,

    // contract result size greater than max limit
    ResultSizeLimitExceeded = 93,

    // not account 0:0:55
    NotSpecialAccount = 94,

    // contract tx gas value must be positive
    ContractNegativeGas = 95,

    // negative value or initial balance was set for tx, value must be positive
    ContractNegativeValue = 96,

    InvalidFeeFile = 97,

    InvalidExchangeRateFile = 98,

    InsufficientLocalCallGas = 99,

    EntityNotAllowedToDelete = 100,

    AuthorizationFailed = 101,

    FileUploadedProtoInvalid = 102,

    FileUploadedProtoNotSavedToDisk = 103,

    FeeScheduleFilePartUploaded = 104,

    ExchangeRateChangeLimitExceeded = 105,
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
            INVALID_SIGNATURE_TYPE_MISMATCHING_KEY => Status::InvalidSignatureTypeMismatch,
            INVALID_SIGNATURE_COUNT_MISMATCHING_KEY => Status::InvalidSignatureCountMismatch,
            EMPTY_CLAIM_BODY => Status::EmptyClaimBody,
            EMPTY_CLAIM_HASH => Status::EmptyClaimHash,
            EMPTY_CLAIM_KEYS => Status::EmptyClaimKeys,
            INVALID_CLAIM_HASH_SIZE => Status::InvalidClaimHashSize,
            EMPTY_QUERY_BODY => Status::EmptyQueryBody,
            EMPTY_CLAIM_QUERY => Status::EmptyClaimQuery,
            CLAIM_NOT_FOUND => Status::ClaimNotFound,
            ACCOUNT_ID_DOES_NOT_EXIST => Status::AccountIdDoesNotExist,
            CLAIM_ALREADY_EXISTS => Status::ClaimAlreadyExists,
            INVALID_FILE_WACL => Status::InvalidFileWACL,
            SERIALIZATION_FAILED => Status::SerializationFailed,
            TRANSACTION_OVERSIZE => Status::TransactionOversize,
            TRANSACTION_TOO_MANY_LAYERS => Status::TransactionTooManyLayers,
            CONTRACT_DELETED => Status::ContractDeleted,
            PLATFORM_NOT_ACTIVE => Status::PlatformNotActive,
            KEY_PREFIX_MISMATCH => Status::KeyPrefixMismatch,
            PLATFORM_TRANSACTION_NOT_CREATED => Status::PlatformNotActive,
            INVALID_RENEWAL_PERIOD => Status::InvalidRenewalPeriod,
            INVALID_PAYER_ACCOUNT_ID => Status::InvalidPayerAccount,
            ACCOUNT_DELETED => Status::AccountDeleted,
            FILE_DELETED => Status::FileDeleted,
            ACCOUNT_REPEATED_IN_ACCOUNT_AMOUNTS => Status::AccountRepeatedInAccountAmounts,
            SETTING_NEGATIVE_ACCOUNT_BALANCE => Status::SettingNegativeAccountBalance,
            OBTAINER_REQUIRED => Status::ObtainerRequired,
            OBTAINER_SAME_CONTRACT_ID => Status::ObtainerSameContractId,
            OBTAINER_DOES_NOT_EXIST => Status::ObtainerDoesNotExist,
            MODIFYING_IMMUTABLE_CONTRACT => Status::ModifyingImmutableContract,
            FILE_SYSTEM_EXCEPTION => Status::FileSystemException,
            AUTORENEW_DURATION_NOT_IN_RANGE => Status::AutorenewDurationNotInRange,
            ERROR_DECODING_BYTESTRING => Status::ErrorDecodingBytestring,
            CONTRACT_FILE_EMPTY => Status::ContractFileEmpty,
            CONTRACT_BYTECODE_EMPTY => Status::ContractBytecodeEmpty,
            INVALID_INITIAL_BALANCE => Status::InvalidInitialBalance,
            INVALID_RECEIVE_RECORD_THRESHOLD => Status::InvalidReceiveRecordThreshold,
            INVALID_SEND_RECORD_THRESHOLD => Status::InvalidSendRecordThreshold,
            ACCOUNT_IS_NOT_GENESIS_ACCOUNT => Status::AccountIsNotGenesisAccount,
            PAYER_ACCOUNT_UNAUTHORIZED => Status::PayerAccountUnauthorized,
            INVALID_FREEZE_TRANSACTION_BODY => Status::InvalidFreezeTransactionBody,
            FREEZE_TRANSACTION_BODY_NOT_FOUND => Status::FreezeTransactionBodyNotFound,
            TRANSFER_LIST_SIZE_LIMIT_EXCEEDED => Status::TransferListSizeLimitExceeded,
            RESULT_SIZE_LIMIT_EXCEEDED => Status::ResultSizeLimitExceeded,
            NOT_SPECIAL_ACCOUNT => Status::NotSpecialAccount,
            CONTRACT_NEGATIVE_GAS => Status::ContractNegativeGas,
            CONTRACT_NEGATIVE_VALUE => Status::ContractNegativeValue,
            INVALID_FEE_FILE => Status::InvalidFeeFile,
            INVALID_EXCHANGE_RATE_FILE => Status::InvalidExchangeRateFile,
            INSUFFICIENT_LOCAL_CALL_GAS => Status::InsufficientLocalCallGas,
            ENTITY_NOT_ALLOWED_TO_DELETE => Status::EntityNotAllowedToDelete,
            AUTHORIZATION_FAILED => Status::AuthorizationFailed,
            FILE_UPLOADED_PROTO_INVALID => Status::FileUploadedProtoInvalid,
            FILE_UPLOADED_PROTO_NOT_SAVED_TO_DISK => Status::FileUploadedProtoNotSavedToDisk,
            FEE_SCHEDULE_FILE_PART_UPLOADED => Status::FeeScheduleFilePartUploaded,
            EXCHANGE_RATE_CHANGE_LIMIT_EXCEEDED => Status::ExchangeRateChangeLimitExceeded
        }
    }
}
