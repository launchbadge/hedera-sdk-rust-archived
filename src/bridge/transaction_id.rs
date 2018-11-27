use crate::{AccountId, TransactionId};
use libc::c_char;
use mbox::MString;
use std::ffi::CStr;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_id_new(account_id: AccountId) -> TransactionId {
    TransactionId::new(account_id)
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_id_to_str(p: *mut TransactionId) -> *mut c_char {
    debug_assert!(!p.is_null());

    MString::from_str(&(*p).to_string())
        .into_mbox_with_sentinel()
        .into_raw() as *mut c_char
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_id_from_str(s: *const c_char, out: *mut TransactionId) -> u64 {
    debug_assert!(!s.is_null());
    debug_assert!(!out.is_null());

    let s = CStr::from_ptr(s);
    let s = s.to_string_lossy();

    *out = try_ffi!(s.parse());

    0
}
