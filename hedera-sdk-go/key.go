package hedera
import "C"

// #include <stdlib.h>
// #include "hedera-key.h"
import "C"
import "unsafe"

// An EdDSA secret key.
type SecretKey struct {
	inner C.HederaSecretKey
}

// Generate a new [SecretKey] from a cryptographically secure pseudo-random number generator (CSPRNG).
func GenerateSecretKey() SecretKey {
	return SecretKey{C.hedera_secret_key_generate()}
}

// Parse a [HederaSecretKey] from a hex-encoded string.
func SecretKeyFromString(s string) SecretKey {
	return SecretKey{C.hedera_secret_key_from_str(C.CString(s))}
}

// Format this [SecretKey] as a hex-encoded string of the secret key encoded with a PKCS #8 wrapper (
// defined in RFC 5208).
func (key SecretKey) String() string {
	bytes := C.hedera_secret_key_to_str(&key.inner)
	defer C.free(unsafe.Pointer(bytes))

	return C.GoString(bytes)
}

// Derive a [PublicKey] from this [SecretKey].
func (key SecretKey) Public() PublicKey {
	return PublicKey{C.hedera_public_key_from_secret_key(&key.inner)}
}

// An ed25519 public key.
type PublicKey struct {
	inner C.HederaPublicKey
}

// Format this [PublicKey] as a hex-encoded string of the secret key encoded with a PKIX wrapper (
// defined in RFC 3280).
func (key PublicKey) String() string {
	bytes := C.hedera_public_key_to_str(&key.inner)
	defer C.free(unsafe.Pointer(bytes))

	return C.GoString(bytes)
}
