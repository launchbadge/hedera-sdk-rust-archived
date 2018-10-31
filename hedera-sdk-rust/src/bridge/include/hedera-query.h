#ifndef HEDERA_QUERY_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_QUERY_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include "hedera-client.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct HederaQuery HederaQuery;

typedef enum {
    RESPONSE_ANSWER_ONLY = 0,
    // unsupported: RESPONSE_ANSWER_STATE_PROOF = 1,
    RESPONSE_COST_ANSWER = 2,
    RESPONSE_COST_ANSWER_STATE_PROOF = 3,
} HederaResponseKind;

#ifdef __cplusplus
}
#endif

#endif // HEDERA_QUERY_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
