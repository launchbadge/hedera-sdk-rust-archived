#ifndef HEDERA_TRANSACTION_CREATE_ACCOUNT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_TRANSACTION_CREATE_ACCOUNT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include "hedera-transaction.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__create_account__new(HederaClient*);

extern void hedera_transaction__create_account__set_initial_balance(HederaTransaction*, uint64_t balance);

extern void hedera_transaction__create_account__set_key(HederaTransaction*, HederaPublicKey key);

extern HederaTransactionResponse hedera_transaction__create_account__execute(HederaTransaction*);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_TRANSACTION_CREATE_ACCOUNT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
