use crate::proto::{self, ToProto};
use ed25519_dalek;
use failure::{bail, Error};
use failure_derive::Fail;
use hex;
use num::BigUint;
use once_cell::{sync::Lazy, sync_lazy};
use rand::{thread_rng, CryptoRng, Rng};
use sha2::Sha512;
use simple_asn1::{
    der_decode, der_encode, oid, to_der, ASN1Block, ASN1Class, ASN1DecodeErr, ASN1EncodeErr,
    FromASN1, ToASN1, OID,
};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

// Types used for (de-)serializing public and secret keys from ASN.1 byte
// streams.
//

#[derive(Debug, Fail)]
enum ASN1Error {
    #[fail(display = "{:?}", _0)]
    Decode(ASN1DecodeErr),

    #[fail(display = "{:?}", _0)]
    Encode(ASN1EncodeErr),

    #[fail(display = "expected `{}`; found: `{}`", expected, found)]
    UnexpectedType {
        expected: &'static str,
        found: String,
    },
}

impl From<ASN1DecodeErr> for ASN1Error {
    fn from(err: ASN1DecodeErr) -> Self {
        ASN1Error::Decode(err)
    }
}

impl From<ASN1EncodeErr> for ASN1Error {
    fn from(err: ASN1EncodeErr) -> Self {
        ASN1Error::Encode(err)
    }
}

// [https://tools.ietf.org/id/draft-ietf-curdle-pkix-01.html#rfc.section.3]
static OID_ED25519: Lazy<OID> = sync_lazy! { oid!(1, 3, 101, 112) };

// [https://www.ietf.org/rfc/rfc3280.txt]
// AlgorithmIdentifier ::= SEQUENCE {
//      algorithm               OBJECT IDENTIFIER,
//      parameters              ANY DEFINED BY algorithm OPTIONAL }

#[derive(Debug)]
struct AlgorithmIdentifier {
    algorithm: OID,
}

impl FromASN1 for AlgorithmIdentifier {
    type Error = ASN1Error;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Self::Error> {
        let algorithm = if let Some(ASN1Block::Sequence(_, _, blocks)) = v.get(0) {
            if let Some(ASN1Block::ObjectIdentifier(_, _, id)) = blocks.get(0) {
                id
            } else {
                return Err(ASN1Error::UnexpectedType {
                    expected: "OBJECT IDENTIFIER",
                    found: format!("{:?}", blocks.get(0)),
                });
            }
        } else {
            return Err(ASN1Error::UnexpectedType {
                expected: "SEQUENCE",
                found: format!("{:?}", v.get(0)),
            });
        };

        Ok((
            Self {
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                algorithm: algorithm.clone(),
            },
            &v[1..],
        ))
    }
}

// [https://www.ietf.org/rfc/rfc3280.txt]
// SubjectPublicKeyInfo ::= SEQUENCE {
//      algorithm            AlgorithmIdentifier,
//      subjectPublicKey     BIT STRING }

#[derive(Debug)]
struct SubjectPublicKeyInfo {
    algorithm: AlgorithmIdentifier,
    subject_public_key: Vec<u8>,
}

impl ToASN1 for SubjectPublicKeyInfo {
    type Error = ASN1Error;

    fn to_asn1_class(&self, c: ASN1Class) -> Result<Vec<ASN1Block>, Self::Error> {
        Ok(vec![ASN1Block::Sequence(
            c,
            0,
            vec![
                // AlgorithmIdentifier
                ASN1Block::Sequence(
                    c,
                    0,
                    vec![
                        // Algorithm
                        // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                        ASN1Block::ObjectIdentifier(c, 0, self.algorithm.algorithm.clone()),
                    ],
                ),
                // subjectPublicKey
                ASN1Block::BitString(
                    c,
                    0,
                    self.subject_public_key.len() * 8,
                    // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                    self.subject_public_key.clone(),
                ),
            ],
        )])
    }
}

impl FromASN1 for SubjectPublicKeyInfo {
    type Error = ASN1Error;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Self::Error> {
        let (algorithm, subject_public_key) =
            if let Some(ASN1Block::Sequence(_, _, blocks)) = v.get(0) {
                // Parse: algorithm
                let (algorithm, blocks): (AlgorithmIdentifier, _) = FromASN1::from_asn1(blocks)?;

                // Parse: subject_public_key
                if let Some(ASN1Block::BitString(_, _, _, bytes)) = blocks.get(0) {
                    (algorithm, bytes)
                } else {
                    return Err(ASN1Error::UnexpectedType {
                        expected: "BIT STRING",
                        found: format!("{:?}", blocks.get(0)),
                    });
                }
            } else {
                return Err(ASN1Error::UnexpectedType {
                    expected: "SEQUENCE",
                    found: format!("{:?}", v.get(0)),
                });
            };

        Ok((
            Self {
                algorithm,
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                subject_public_key: subject_public_key.clone(),
            },
            &v[1..],
        ))
    }
}

// [https://www.ietf.org/rfc/rfc5208.txt]
// PrivateKeyInfo ::= SEQUENCE {
//      version                   INTEGER,
//      privateKeyAlgorithm       AlgorithmIdentifier,
//      privateKey                OCTET STRING,
//      attributes           [0]  IMPLICIT Attributes OPTIONAL }

struct PrivateKeyInfo {
    algorithm: AlgorithmIdentifier,
    private_key: Vec<u8>,
}

impl ToASN1 for PrivateKeyInfo {
    type Error = ASN1Error;

    fn to_asn1_class(&self, c: ASN1Class) -> Result<Vec<ASN1Block>, Self::Error> {
        Ok(vec![ASN1Block::Sequence(
            c,
            0,
            vec![
                // Version
                ASN1Block::Integer(c, 0, 0.into()),
                // AlgorithmIdentifier
                ASN1Block::Sequence(
                    c,
                    0,
                    vec![
                        // Algorithm
                        // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                        ASN1Block::ObjectIdentifier(c, 0, self.algorithm.algorithm.clone()),
                    ],
                ),
                // PrivateKey
                ASN1Block::OctetString(
                    c,
                    0,
                    // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                    to_der(&ASN1Block::OctetString(c, 0, self.private_key.clone()))?,
                ),
            ],
        )])
    }
}

impl FromASN1 for PrivateKeyInfo {
    type Error = ASN1Error;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Self::Error> {
        let (algorithm, key) = if let Some(ASN1Block::Sequence(_, _, blocks)) = v.get(0) {
            // Parse: algorithm
            let (algorithm, blocks): (AlgorithmIdentifier, _) = FromASN1::from_asn1(&blocks[1..])?;

            // Parse: subject_public_key
            if let Some(ASN1Block::OctetString(_, _, bytes)) = blocks.get(0) {
                (algorithm, bytes)
            } else {
                return Err(ASN1Error::UnexpectedType {
                    expected: "OCTET STRING",
                    found: format!("{:?}", blocks.get(0)),
                });
            }
        } else {
            return Err(ASN1Error::UnexpectedType {
                expected: "SEQUENCE",
                found: format!("{:?}", v.get(0)),
            });
        };

        Ok((
            Self {
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                algorithm,
                // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                private_key: key.clone(),
            },
            &v[1..],
        ))
    }
}

// Public Key
//

#[derive(Debug, PartialEq)]
pub struct PublicKey(ed25519_dalek::PublicKey);

impl PublicKey {
    /// Construct a `PublicKey` from a slice of bytes.
    /// Bytes are expected to be either a raw key or encoded in ASN.1.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes.len() == ed25519_dalek::PUBLIC_KEY_LENGTH {
            // If the buffer is exactly the length of a public key; assume that this is
            // a raw key and return it directly
            return Ok(PublicKey(ed25519_dalek::PublicKey::from_bytes(bytes)?));
        }

        let info: SubjectPublicKeyInfo = der_decode(&bytes)?;

        if info.algorithm.algorithm != *OID_ED25519 {
            bail!(
                "ed25519: unknown public key algorithm: {:?}",
                info.algorithm.algorithm
            );
        }

        if info.subject_public_key.len() != ed25519_dalek::PUBLIC_KEY_LENGTH {
            bail!("ed25519: public key length mismatch");
        }

        Ok(PublicKey(ed25519_dalek::PublicKey::from_bytes(
            &info.subject_public_key,
        )?))
    }

    /// Format a `PublicKey` as a vec of bytes in ASN.1 format.
    pub fn to_bytes(&self) -> Vec<u8> {
        // NOTE: Not possible to fail. Only fail case the library has is if OIDs are
        // given       incorrectly.
        der_encode(&SubjectPublicKeyInfo {
            algorithm: AlgorithmIdentifier {
                algorithm: OID_ED25519.clone(),
            },
            subject_public_key: self.0.to_bytes().to_vec(),
        })
        .unwrap()
    }

    /// Verify a signature on a message with this `PublicKey`.
    pub fn verify(&self, message: impl AsRef<[u8]>, signature: &Signature) -> Result<bool, Error> {
        match self.0.verify::<Sha512>(message.as_ref(), &signature.0) {
            Ok(_) => Ok(true),
            Err(error) => {
                if error.to_string() == "Verification equation was not satisfied" {
                    Ok(false)
                } else {
                    Err(error)?
                }
            }
        }
    }
}

/// Construct a `PublicKey` from a hex representation of a raw or ASN.1 encoded
/// key.
impl FromStr for PublicKey {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(&hex::decode(s.as_bytes())?)
    }
}

/// Format a `PublicKey` as a hex representation of its bytes in ASN.1 format.
impl Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.to_bytes()))
    }
}

impl ToProto<proto::BasicTypes::Key> for PublicKey {
    fn to_proto(&self) -> Result<proto::BasicTypes::Key, Error> {
        let mut key = proto::BasicTypes::Key::new();
        key.set_ed25519(self.0.to_bytes().to_vec());
        Ok(key)
    }
}

// Secret Key
//

#[derive(Debug)]
pub struct SecretKey(ed25519_dalek::SecretKey);

impl SecretKey {
    /// Generate a `SecretKey` from cryptographically secure random number
    /// generator.
    pub fn generate() -> Self {
        Self::generate_from(&mut thread_rng())
    }

    /// Generate a `SecretKey` from cryptographically secure random number
    /// generator.
    pub fn generate_from<R: CryptoRng + Rng>(rng: &mut R) -> Self {
        SecretKey(ed25519_dalek::SecretKey::generate(rng))
    }

    /// Construct a `SecretKey` from a slice of bytes.
    /// Bytes are expected to be either a raw key or encoded in ASN.1.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();

        if bytes.len() == ed25519_dalek::SECRET_KEY_LENGTH + ed25519_dalek::PUBLIC_KEY_LENGTH
            || bytes.len() == ed25519_dalek::SECRET_KEY_LENGTH
        {
            // If the buffer looks like a {secret}{public} byte string; just pull the secret
            // key bytes off of it
            return Ok(SecretKey(ed25519_dalek::SecretKey::from_bytes(
                &bytes[..ed25519_dalek::SECRET_KEY_LENGTH],
            )?));
        }

        let info: PrivateKeyInfo = der_decode(&bytes)?;

        if info.algorithm.algorithm != *OID_ED25519 {
            bail!(
                "ed25519: PKCS#8 wrapping contained private key with unknown algorithm: {:?}",
                info.algorithm.algorithm
            );
        }

        Ok(SecretKey(ed25519_dalek::SecretKey::from_bytes(
            &info.private_key[2..],
        )?))
    }

    /// Format a `SecretKey` as a vec of bytes in ASN.1 format.
    pub fn to_bytes(&self) -> Vec<u8> {
        // NOTE: Not possible to fail. Only fail case the library has is if OIDs are
        // given       incorrectly.
        der_encode(&PrivateKeyInfo {
            algorithm: AlgorithmIdentifier {
                algorithm: OID_ED25519.clone(),
            },
            private_key: self.0.to_bytes().to_vec(),
        })
        .unwrap()
    }

    /// Derive a `PublicKey` from this `SecretKey`.
    #[inline]
    pub fn public(&self) -> PublicKey {
        PublicKey(ed25519_dalek::PublicKey::from_secret::<Sha512>(&self.0))
    }

    /// Sign a message with this `SecretKey`.
    #[inline]
    pub fn sign(&self, message: impl AsRef<[u8]>) -> Signature {
        Signature(
            self.0
                .expand::<Sha512>()
                .sign::<Sha512>(message.as_ref(), &self.public().0),
        )
    }
}

/// Construct a `SecretKey` from a hex representation of a raw or ASN.1 encoded
/// key.
impl FromStr for SecretKey {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(&hex::decode(s.as_bytes())?)
    }
}

/// Format a `SecretKey` as a hex representation of its bytes in ASN.1 format.
impl Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.to_bytes()))
    }
}

// KeyPair
//

#[derive(Debug)]
pub struct KeyPair(ed25519_dalek::Keypair);

impl KeyPair {
    pub fn generate() -> Self {
        Self::generate_from(&mut thread_rng())
    }

    pub fn generate_from<R: CryptoRng + Rng>(rng: &mut R) -> Self {
        KeyPair(ed25519_dalek::Keypair::generate::<Sha512, _>(rng))
    }

    pub fn sign(&self, message: impl AsRef<[u8]>) -> Signature {
        Signature(self.0.sign::<Sha512>(message.as_ref()))
    }

    pub fn verify(&self, message: impl AsRef<[u8]>, signature: &Signature) -> Result<bool, Error> {
        match self.0.verify::<Sha512>(message.as_ref(), &signature.0) {
            Ok(_) => Ok(true),
            Err(error) => {
                if error.to_string() == "Verification equation was not satisfied" {
                    Ok(false)
                } else {
                    Err(error)?
                }
            }
        }
    }
}

// Signature
//

#[derive(Debug)]
pub struct Signature(ed25519_dalek::Signature);

impl Signature {
    /// Construct a `Signature` from a slice of bytes.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(Signature(ed25519_dalek::Signature::from_bytes(
            bytes.as_ref(),
        )?))
    }
}

/// Construct a `Signature` from a hex representation of the signature.
impl FromStr for Signature {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(&hex::decode(s.as_bytes())?)
    }
}

/// Format a `Signature` as a hex representation of its bytes.
impl Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.0.to_bytes()[..]))
    }
}

impl ToProto<proto::BasicTypes::Signature> for Signature {
    fn to_proto(&self) -> Result<proto::BasicTypes::Signature, Error> {
        let mut signature = proto::BasicTypes::Signature::new();
        signature.set_ed25519(self.0.to_bytes().to_vec());
        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyPair, PublicKey, SecretKey, Signature};
    use crate::test::{black_box, Bencher};

    const KEY_PUBLIC_ASN1_HEX: &str =
        "302a300506032b6570032100e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7";

    const KEY_PUBLIC_HEX: &str = "e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7";

    const KEY_SECRET_ASN1_HEX: &str =
        "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10";

    const KEY_SECRET_HEX: &str = "db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10\
                                  e0c8ec2758a5879ffac226a13c0c516b799e72e35141a0dd828f94d37988a4b7";

    const MESSAGE: &str = "This is a message about the world.";
    const SIGNATURE: &str = "73bea53f31ca9c42a422ecb7516ec08d0bbd1a6bfd630ccf10ec1872454814d29f4a8011129cd007eab544af01a75f508285b591e5bed24b68f927751e49e30e";

    #[test]
    fn test_parse() {
        let public_key1: PublicKey = KEY_PUBLIC_ASN1_HEX.parse().unwrap();
        let public_key2: PublicKey = KEY_PUBLIC_HEX.parse().unwrap();

        let secret_key1: SecretKey = KEY_SECRET_ASN1_HEX.parse().unwrap();
        let secret_key2: SecretKey = KEY_SECRET_HEX.parse().unwrap();

        assert_eq!(public_key1, public_key2);
        assert_eq!(secret_key1.0.as_bytes(), secret_key2.0.as_bytes());
        assert_eq!(public_key1, secret_key1.public());
        assert_eq!(public_key2, secret_key2.public());
        assert_eq!(secret_key2.public(), secret_key1.public());
    }

    #[test]
    fn test_verify() {
        let key: PublicKey = KEY_PUBLIC_ASN1_HEX.parse().unwrap();
        let signature: Signature = SIGNATURE.parse().unwrap();
        let verified = key.verify(MESSAGE.as_bytes(), &signature).unwrap();

        assert!(verified);
    }

    #[test]
    fn test_sign() {
        let key: SecretKey = KEY_SECRET_ASN1_HEX.parse().unwrap();
        let signature = key.sign(MESSAGE.as_bytes());

        assert_eq!(SIGNATURE, signature.to_string());
    }

    #[test]
    fn test_generate() {
        let key = SecretKey::generate();
        let signature = key.sign(MESSAGE.as_bytes());
        let verified = key.public().verify(MESSAGE.as_bytes(), &signature).unwrap();

        assert!(verified);
    }

    #[test]
    fn test_display() {
        let public_key1: PublicKey = KEY_PUBLIC_ASN1_HEX.parse().unwrap();
        let public_key2: PublicKey = public_key1.to_string().parse().unwrap();

        let secret_key1: SecretKey = KEY_SECRET_ASN1_HEX.parse().unwrap();
        let secret_key2: SecretKey = secret_key1.to_string().parse().unwrap();

        assert_eq!(public_key1, public_key2);
        assert_eq!(secret_key1.0.as_bytes(), secret_key2.0.as_bytes());
    }

    #[test]
    fn test_key_pair_generate() {
        let key_pair = KeyPair::generate();
        let signature = key_pair.sign(MESSAGE.as_bytes());
        let verified = key_pair.verify(MESSAGE.as_bytes(), &signature).unwrap();

        assert!(verified);
    }

    #[bench]
    fn bench_generate(b: &mut Bencher) {
        b.iter(|| {
            let secret = SecretKey::generate();
            let public = secret.public();

            black_box(public);
        });
    }

    #[bench]
    fn bench_sign(b: &mut Bencher) {
        let key: SecretKey = KEY_SECRET_ASN1_HEX.parse().unwrap();

        b.iter(|| {
            black_box(key.sign(MESSAGE.as_bytes()));
        });
    }

    #[bench]
    fn bench_verify(b: &mut Bencher) {
        let key = SecretKey::generate();
        let public = key.public();
        let message = MESSAGE.as_bytes();
        let signature = key.sign(message);

        b.iter(|| {
            black_box(public.verify(message, &signature).unwrap());
        });
    }
}
