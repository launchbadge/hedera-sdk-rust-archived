#ifndef HEDERA_KEY_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
#define HEDERA_KEY_9999A0E8_2BD1_4C33_8071_D93A13B8A9E

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// TODO: Define a Keypair structure. Would be more efficient for signing.

/// An EdDSA secret key.
typedef struct { uint8_t bytes[32]; } HederaSecretKey;

/// Generate a new [HederaSecretKey] from a cryptographically secure pseudo-random number generator (CSPRNG).
extern HederaSecretKey hedera_secret_key_generate();

/// Parse a [HederaSecretKey] from a hex-encoded string.
extern HederaSecretKey hedera_secret_key_from_str(const char* s);

/// Format a [HederaSecretKey] as a hex-encoded string of the secret key encoded with a PKCS #8 wrapper (
/// defined in RFC 5208).
///
/// Returns ownership of the string. Must be freed with [free].
extern char* hedera_secret_key_to_str(HederaSecretKey*);

/// An ed25519 public key.
typedef struct { uint8_t bytes[32]; } HederaPublicKey;

/// Derive a [HederaPublicKey] from a [HederaSecretKey].
extern HederaPublicKey hedera_public_key_from_secret_key(HederaSecretKey*);

/// Format a [HederaPublicKey] as a hex-encoded string of the secret key encoded with a PKIX wrapper (
/// defined in RFC 3280).
///
/// Returns ownership of the string. Must be freed with [free].
extern char* hedera_public_key_to_str(HederaPublicKey*);

#ifdef __cplusplus
}
#endif

#endif // HEDERA_KEY_9999A0E8_2BD1_4C33_8071_D93A13B8A9E
