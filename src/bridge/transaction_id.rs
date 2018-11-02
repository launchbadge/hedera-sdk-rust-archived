use crate::{AccountId, TransactionId};
use libc::c_char;
use mbox::MString;

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
