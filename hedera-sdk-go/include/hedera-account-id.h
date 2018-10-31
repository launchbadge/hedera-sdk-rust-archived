#ifndef HEDERA_ACCOUNT_ID_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_ACCOUNT_ID_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    /// The shard number (non-negative).
    int64_t shard;

    /// The realm number (non-negative).
    int64_t realm;

    /// A non-negative number unique within its realm.
    int64_t account;
} HederaAccountId;

extern HederaAccountId hedera_account_id_from_str(const char* s);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_ACCOUNT_ID_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
