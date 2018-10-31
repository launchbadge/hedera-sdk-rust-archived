#ifndef HEDERA_TRANSACTION_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_TRANSACTION_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include "hedera-key.h"
#include "hedera-client.h"
#include "hedera-account-id.h"
#include "hedera-transaction-id.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct HederaTransaction HederaTransaction;

typedef struct {
    HederaTransactionId id;
    HederaPrecheckCode precheck;
} HederaTransactionResponse;

extern void hedera_transaction_set_operator(HederaTransaction*, HederaAccountId operator_);

extern void hedera_transaction_set_node(HederaTransaction*, HederaAccountId node);

extern void hedera_transaction_set_memo(HederaTransaction*, const char* memo);

extern void hedera_transaction_sign(HederaTransaction*, HederaSecretKey);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_TRANSACTION_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
