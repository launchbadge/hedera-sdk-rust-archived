#ifndef HEDERA_TRANSACTION_ID_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_TRANSACTION_ID_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include "hedera-timestamp.h"
#include "hedera-account-id.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    HederaAccountId account_id;
    HederaTimestamp transaction_valid_start;
} HederaTransactionId;

extern HederaTransactionId hedera_transaction_id_new(HederaAccountId account);

extern char* hedera_transaction_id_to_str(HederaTransactionId*);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_TRANSACTION_ID_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
