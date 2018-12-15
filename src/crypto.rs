use crate::proto::{self, ToProto};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use ed25519_dalek;
use failure::{bail, err_msg, Error};
use failure_derive::Fail;
use hex;
use num::BigUint;
use once_cell::{sync::Lazy, sync_lazy};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use sha2::Sha512;
use simple_asn1::{
    der_decode, der_encode, oid, to_der, ASN1Block, ASN1Class, ASN1DecodeErr, ASN1EncodeErr,
    FromASN1, ToASN1, OID,
};
use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};
use try_from::{TryFrom, TryInto};

// Types used for (de-)serializing public and secret keys from ASN.1 byte
// streams.

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
        let algorithm = if let Some(ASN1Block::Sequence(_, blocks)) = v.get(0) {
            if let Some(ASN1Block::ObjectIdentifier(_, id)) = blocks.get(0) {
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

    fn to_asn1_class(&self, _c: ASN1Class) -> Result<Vec<ASN1Block>, Self::Error> {
        Ok(vec![ASN1Block::Sequence(
            0,
            vec![
                // AlgorithmIdentifier
                ASN1Block::Sequence(
                    0,
                    vec![
                        // Algorithm
                        // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                        ASN1Block::ObjectIdentifier(0, self.algorithm.algorithm.clone()),
                    ],
                ),
                // subjectPublicKey
                ASN1Block::BitString(
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
            if let Some(ASN1Block::Sequence(_, blocks)) = v.get(0) {
                // Parse: algorithm
                let (algorithm, blocks): (AlgorithmIdentifier, _) = FromASN1::from_asn1(blocks)?;

                // Parse: subject_public_key
                if let Some(ASN1Block::BitString(_, _, bytes)) = blocks.get(0) {
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

    fn to_asn1_class(&self, _c: ASN1Class) -> Result<Vec<ASN1Block>, Self::Error> {
        Ok(vec![ASN1Block::Sequence(
            0,
            vec![
                // Version
                ASN1Block::Integer(0, 0.into()),
                // AlgorithmIdentifier
                ASN1Block::Sequence(
                    0,
                    vec![
                        // Algorithm
                        // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                        ASN1Block::ObjectIdentifier(0, self.algorithm.algorithm.clone()),
                    ],
                ),
                // PrivateKey
                ASN1Block::OctetString(
                    0,
                    // FIXME: Rewrite or improve the ASN.1 lib to remove allocation requirement
                    to_der(&ASN1Block::OctetString(0, self.private_key.clone()))?,
                ),
            ],
        )])
    }
}

impl FromASN1 for PrivateKeyInfo {
    type Error = ASN1Error;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Self::Error> {
        let (algorithm, key) = if let Some(ASN1Block::Sequence(_, blocks)) = v.get(0) {
            // Parse: algorithm
            let (algorithm, blocks): (AlgorithmIdentifier, _) = FromASN1::from_asn1(&blocks[1..])?;

            // Parse: subject_public_key
            if let Some(ASN1Block::OctetString(_, bytes)) = blocks.get(0) {
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

/// An ed25519 public key.
#[repr(C)]
#[derive(PartialEq, Clone)]
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
        der_encode(&SubjectPublicKeyInfo {
            algorithm: AlgorithmIdentifier {
                algorithm: OID_ED25519.clone(),
            },
            subject_public_key: self.0.to_bytes().to_vec(),
        })
        // NOTE: Not possible to fail. Only fail case the library has is if OIDs are
        //       given incorrectly.
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

impl Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
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

impl TryFrom<proto::BasicTypes::Key> for PublicKey {
    type Err = Error;

    fn try_from(mut key: proto::BasicTypes::Key) -> Result<Self, Self::Err> {
        if key.has_ed25519() {
            let bytes = key.take_ed25519();
            if bytes.len() == 64 {
                // This is hex-encoded
                // CryptoGetInfo returns the public key like this
                Self::from_bytes(hex::decode(&bytes)?)
            } else {
                Self::from_bytes(bytes)
            }
        } else if key.has_keyList() && key.get_keyList().keys.len() == 1 {
            Ok(key.take_keyList().keys.remove(0).try_into()?)
        } else {
            Err(err_msg("Only ed25519 public keys are currently supported"))
        }
    }
}

/// An EdDSA secret key.
#[repr(C)]
pub struct SecretKey(ed25519_dalek::SecretKey);

impl SecretKey {
    /// Generate a `SecretKey` with a BIP-39 mnemonic using a cryptographically
    /// secure random number generator.
    ///
    /// The `password` is required with the mnemonic to reproduce the secret key.
    pub fn generate(password: &str) -> (Self, String) {
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);

        let secret = Self::generate_with_mnemonic(&mnemonic, password);

        (secret, mnemonic.into_phrase())
    }

    fn generate_with_mnemonic(mnemonic: &Mnemonic, password: &str) -> Self {
        let mut seed: [u8; 32] = Default::default();

        seed.copy_from_slice(&Seed::new(&mnemonic, password).as_bytes()[0..32]);

        let mut rng = ChaChaRng::from_seed(seed);
        SecretKey(ed25519_dalek::SecretKey::generate(&mut rng))
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

    /// Re-construct a `SecretKey` from the supplied mnemonic and password.
    pub fn from_mnemonic(mnemonic: &str, password: &str) -> Result<Self, Error> {
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English)?;

        Ok(Self::generate_with_mnemonic(&mnemonic, password))
    }

    /// Format a `SecretKey` as a vec of bytes in ASN.1 format.
    pub fn to_bytes(&self) -> Vec<u8> {
        der_encode(&PrivateKeyInfo {
            algorithm: AlgorithmIdentifier {
                algorithm: OID_ED25519.clone(),
            },
            private_key: self.0.to_bytes().to_vec(),
        })
        // NOTE: Not possible to fail. Only fail case the library has is if OIDs are
        //       given incorrectly.
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

impl Clone for SecretKey {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_bytes(self.0.as_bytes()).unwrap()
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

impl<E> TryFrom<Result<SecretKey, E>> for SecretKey {
    type Err = E;

    #[inline]
    fn try_from(res: Result<Self, E>) -> Result<Self, Self::Err> {
        res
    }
}

impl<E> TryFrom<Result<String, E>> for SecretKey
where
    E: Sync + Send + 'static + fmt::Debug + fmt::Display,
{
    type Err = Error;

    #[inline]
    fn try_from(res: Result<String, E>) -> Result<Self, Error> {
        res.map_err(err_msg)?.parse()
    }
}

impl TryFrom<SecretKey> for SecretKey {
    type Err = Error;

    #[inline]
    fn try_from(self_: Self) -> Result<Self, Error> {
        Ok(self_)
    }
}

impl Debug for SecretKey {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

/// Format a `SecretKey` as a hex representation of its bytes in ASN.1 format.
impl Display for SecretKey {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.to_bytes()))
    }
}

/// An EdDSA signature.
#[derive(Debug)]
pub struct Signature(ed25519_dalek::Signature);

impl Signature {
    /// Construct a `Signature` from a slice of bytes.
    #[inline]
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

impl<'a> ToProto<proto::BasicTypes::Signature> for &'a [&'a Signature] {
    fn to_proto(&self) -> Result<proto::BasicTypes::Signature, Error> {
        let mut list = proto::BasicTypes::SignatureList::new();

        for signature in self.iter() {
            list.sigs.push(signature.to_proto()?);
        }

        let mut wrapper = proto::BasicTypes::Signature::new();
        wrapper.set_signatureList(list);

        Ok(wrapper)
    }
}

#[cfg(test)]
mod tests {
    use super::{PublicKey, SecretKey, Signature};
    use failure::Error;

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
    fn test_parse() -> Result<(), Error> {
        let public_key1: PublicKey = KEY_PUBLIC_ASN1_HEX.parse()?;
        let public_key2: PublicKey = KEY_PUBLIC_HEX.parse()?;

        let secret_key1: SecretKey = KEY_SECRET_ASN1_HEX.parse()?;
        let secret_key2: SecretKey = KEY_SECRET_HEX.parse()?;

        assert_eq!(public_key1, public_key2);
        assert_eq!(secret_key1.0.as_bytes(), secret_key2.0.as_bytes());
        assert_eq!(public_key1, secret_key1.public());
        assert_eq!(public_key2, secret_key2.public());
        assert_eq!(secret_key2.public(), secret_key1.public());

        Ok(())
    }

    #[test]
    fn test_verify() -> Result<(), Error> {
        let key: PublicKey = KEY_PUBLIC_ASN1_HEX.parse()?;
        let signature: Signature = SIGNATURE.parse()?;
        let verified = key.verify(MESSAGE.as_bytes(), &signature)?;

        assert!(verified);

        Ok(())
    }

    #[test]
    fn test_sign() -> Result<(), Error> {
        let key: SecretKey = KEY_SECRET_ASN1_HEX.parse()?;
        let signature = key.sign(MESSAGE.as_bytes());

        assert_eq!(SIGNATURE, signature.to_string());

        Ok(())
    }

    #[test]
    fn test_generate() -> Result<(), Error> {
        let (key, _mnemonic) = SecretKey::generate("");
        let signature = key.sign(MESSAGE.as_bytes());
        let verified = key.public().verify(MESSAGE.as_bytes(), &signature)?;

        assert!(verified);

        Ok(())
    }

    #[test]
    fn test_display() -> Result<(), Error> {
        let public_key1: PublicKey = KEY_PUBLIC_ASN1_HEX.parse()?;
        let public_key2: PublicKey = public_key1.to_string().parse()?;

        let secret_key1: SecretKey = KEY_SECRET_ASN1_HEX.parse()?;
        let secret_key2: SecretKey = secret_key1.to_string().parse()?;

        assert_eq!(public_key1, public_key2);
        assert_eq!(secret_key1.0.as_bytes(), secret_key2.0.as_bytes());

        Ok(())
    }

    #[test]
    fn test_reconstruct() -> Result<(), Error> {
        let (secret1, mnemonic) = SecretKey::generate("this-is-not-a-password");
        let secret2 = SecretKey::from_mnemonic(&mnemonic, "this-is-not-a-password")?;

        assert_eq!(secret1.to_bytes(), secret2.to_bytes());

        Ok(())
    }
}
