#ifndef HEDERA_QUERY_GET_ACCOUNT_BALANCE_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_QUERY_GET_ACCOUNT_BALANCE_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include <stdint.h>
#include "hedera-account-id.h"
#include "hedera-client.h"
#include "hedera-query.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint8_t  precheck;
    uint8_t  kind;
    uint64_t cost;
    uint64_t balance;
} HederaQueryGetAccountBalanceResponse;

extern HederaQuery* hedera_query__get_account_balance__new(
    HederaClient*,
    HederaAccountId account
);

extern HederaQueryGetAccountBalanceResponse hedera_query__get_account_balance__send(HederaQuery*);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_QUERY_GET_ACCOUNT_BALANCE_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
