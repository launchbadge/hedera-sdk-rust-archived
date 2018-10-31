#ifndef HEDERA_CLIENT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_CLIENT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include "hedera-account-id.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    /// The transaction passed the precheck.
    PRECHECK_OK = 0,

    /// The transaction had incorrect syntax or other errors.
    PRECHECK_INVALID_TRANSACTION = 1,

    /// The payer account or node account isn't a valid account number.
    PRECHECK_INVALID_ACCOUNT = 2,

    /// The transaction fee is insufficient for this type of transaction.
    PRECHECK_INSUFFICIENT_FEE = 3,

    /// The payer account has an insufficient balance to pay the transaction fee.
    PRECHECK_INSUFFICIENT_BALANCE = 4,

    /// This transaction ID is a duplicate of one that was submitted to this node or
    /// reached consensus in the last 180 seconds (receipt period).
    PRECHECK_DUPLICATE = 5,

    /// If API is throttled out.
    PRECHECK_BUSY = 6,

    /// Unsupported request.
    PRECHECK_NOT_SUPPORTED = 7,
} HederaPrecheckCode;

typedef struct HederaClient HederaClient;

/// Establish a connection to a Hedera node.
/// Must be closed with [hedera_client_close].
extern HederaClient* hedera_client_dial(const char* address);

/// Close and releases resources for a [HederaClient].
extern void hedera_client_close(HederaClient*);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_CLIENT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
