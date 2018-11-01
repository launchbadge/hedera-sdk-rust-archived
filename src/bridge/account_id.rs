use crate::AccountId;
use libc::c_char;
use std::ffi::CStr;
use std::mem;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_account_id_from_str(s: *const c_char) -> AccountId {
    debug_assert!(!s.is_null());

    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str().unwrap();

    let key: AccountId = s.parse().unwrap();

    unsafe { mem::transmute(key) }
}
