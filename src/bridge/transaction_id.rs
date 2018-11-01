use crate::{AccountId, TransactionId};
use libc::c_char;
use mbox::MString;
use std::mem;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_id_new(account_id: AccountId) -> TransactionId {
    TransactionId::new(account_id)
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_id_to_str(p: *mut TransactionId) -> *mut c_char {
    debug_assert!(!p.is_null());

    let id: &TransactionId = unsafe { mem::transmute(&*p) };

    MString::from_str(&id.to_string())
        .into_mbox_with_sentinel()
        .into_raw() as _
}
