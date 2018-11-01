use crate::{PublicKey, SecretKey};
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH};
use libc::c_char;
use mbox::MString;
use std::{ffi::CStr, mem};

// Secret Key
// ----------------------------------------------------------------------------

pub(crate) type CSecretKey = [u8; SECRET_KEY_LENGTH];

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_secret_key_generate() -> CSecretKey {
    unsafe { mem::transmute(SecretKey::generate()) }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_secret_key_to_str(p: *mut CSecretKey) -> *mut c_char {
    debug_assert!(!p.is_null());

    let key: &SecretKey = unsafe { mem::transmute(&*p) };

    MString::from_str(&key.to_string())
        .into_mbox_with_sentinel()
        .into_raw() as _
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_secret_key_from_str(s: *const c_char) -> CSecretKey {
    debug_assert!(!s.is_null());

    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str().unwrap();

    // FIXME: Handle errors
    let key: SecretKey = s.parse().unwrap();

    unsafe { mem::transmute(key) }
}

// Public Key
// ----------------------------------------------------------------------------

pub(crate) type CPublicKey = [u8; PUBLIC_KEY_LENGTH];

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_public_key_from_secret_key(p: *mut CSecretKey) -> CPublicKey {
    debug_assert!(!p.is_null());

    let secret: &SecretKey = unsafe { mem::transmute(&*p) };
    let public = secret.public();

    unsafe { mem::transmute(public) }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_public_key_to_str(p: *mut CPublicKey) -> *mut c_char {
    debug_assert!(!p.is_null());

    let key: &PublicKey = unsafe { mem::transmute(&*p) };

    MString::from_str(&key.to_string())
        .into_mbox_with_sentinel()
        .into_raw() as _
}
