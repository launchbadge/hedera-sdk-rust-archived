#ifndef HEDERA_QUERY_GET_TRANSACTION_RECEIPT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_QUERY_GET_TRANSACTION_RECEIPT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include <stdint.h>
#include "hedera-account-id.h"
#include "hedera-transaction-id.h"
#include "hedera-client.h"
#include "hedera-query.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint8_t  precheck;
    uint8_t  kind;
    uint64_t cost;
    struct {
        uint8_t status;
        HederaAccountId* account_id;
        // unsupported: HederaContractId* contract_id;
        // unsupported: HederaFileId* file_id;
    } answer;
} HederaQueryGetTransactionReceiptResponse;

extern HederaQuery* hedera_query__get_transaction_receipt__new(
    HederaClient*,
    HederaTransactionId transaction_id
);

extern HederaQueryGetTransactionReceiptResponse hedera_query__get_transaction_receipt__send(HederaQuery*);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_QUERY_GET_TRANSACTION_RECEIPT_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
