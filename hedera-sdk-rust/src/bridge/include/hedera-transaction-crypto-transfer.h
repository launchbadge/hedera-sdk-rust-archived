

#ifndef HEDERA_TRANSACTION_CRYPTO_TRANSFER_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_TRANSACTION_CRYPTO_TRANSFER_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include "hedera-transaction.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaTransaction* hedera_transaction__crypto_transfer__new(HederaClient*);

extern void hedera_transaction__crypto_transfer__add_transfer(HederaTransaction*, HederaAccountId id, int64_t amount);

extern HederaTransactionResponse hedera_transaction__crypto_transfer__execute(HederaTransaction*);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_TRANSACTION_CRYPTO_TRANSFER_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
