use crate::{PublicKey, SecretKey, key::Signature};
use libc::c_char;
use mbox::MString;
use std::ffi::CStr;
use std::slice;

//
// Secret Key
//

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_secret_key_generate() -> SecretKey {
    SecretKey::generate()
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_secret_key_to_str(p: *mut SecretKey) -> *mut c_char {
    debug_assert!(!p.is_null());

    MString::from_str(&(*p).to_string())
        .into_mbox_with_sentinel()
        .into_raw() as *mut c_char
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_secret_key_from_str(s: *const c_char, out: *mut SecretKey) -> u64 {
    debug_assert!(!s.is_null());
    debug_assert!(!out.is_null());

    let s = CStr::from_ptr(s);
    let s = s.to_string_lossy();

    *out = try_ffi!(s.parse());

    0
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_secret_key_sign(p: *mut SecretKey, message: *const u8, message_len: usize, out: *mut Signature) -> u64 {
    debug_assert!(!message.is_null());
    debug_assert!(!p.is_null());
    debug_assert!(!out.is_null());

    let message = slice::from_raw_parts(message, message_len);
    *out = (*p).sign(message);

    0
}

//
// Public Key
//

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_secret_key(p: *mut SecretKey) -> PublicKey {
    debug_assert!(!p.is_null());

    (*p).public()
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_to_str(p: *mut PublicKey) -> *mut c_char {
    debug_assert!(!p.is_null());

    MString::from_str(&(*p).to_string())
        .into_mbox_with_sentinel()
        .into_raw() as *mut c_char
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_from_str(s: *const c_char, out: *mut PublicKey) -> u64 {
    debug_assert!(!s.is_null());
    debug_assert!(!out.is_null());

    let s = CStr::from_ptr(s);
    let s = s.to_string_lossy();

    *out = try_ffi!(s.parse());

    0
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_public_key_verify(p: *mut PublicKey, s: *mut Signature, message: *const u8, message_len: usize) -> i8 {
    debug_assert!(!s.is_null());
    debug_assert!(!message.is_null());
    debug_assert!(!p.is_null());

    let message = slice::from_raw_parts(message, message_len);

    match (*p).verify(message, &(*s)) {
        Ok(b) => if b { 1 } else { 0 },
        Err(_) => 0
    }
}

