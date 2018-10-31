#ifndef HEDERA_TIMESTAMP_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_TIMESTAMP_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int64_t seconds;
    int32_t nanos;
} HederaTimestamp;

extern HederaTimestamp hedera_timestamp_new();

#ifdef __cplusplus
}
#endif

#endif // HEDERA_TIMESTAMP_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
