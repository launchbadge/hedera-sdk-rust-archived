use crate::{timestamp::Timestamp, AccountId, TransactionId};
use libc::c_char;
use mbox::MString;
use std::ffi::CStr;

#[repr(C)]
#[derive(Debug)]
pub struct CTransactionId {
    account_id: AccountId,
    transaction_valid_start: Timestamp,
}

impl From<TransactionId> for CTransactionId {
    #[inline]
    fn from(id: TransactionId) -> Self {
        Self {
            account_id: id.account_id,
            transaction_valid_start: id.transaction_valid_start.into(),
        }
    }
}

impl From<CTransactionId> for TransactionId {
    #[inline]
    fn from(id: CTransactionId) -> Self {
        Self {
            account_id: id.account_id,
            transaction_valid_start: id.transaction_valid_start.into(),
        }
    }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_id_new(account_id: AccountId) -> CTransactionId {
    TransactionId::new(account_id).into()
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_id_to_str(p: CTransactionId) -> *mut c_char {
    MString::from_str(&TransactionId::from(p).to_string())
        .into_mbox_with_sentinel()
        .into_raw() as *mut c_char
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_id_from_str(
    s: *const c_char,
    out: *mut CTransactionId,
) -> u64 {
    debug_assert!(!s.is_null());
    debug_assert!(!out.is_null());

    let s = CStr::from_ptr(s);
    let s = s.to_string_lossy();

    *out = try_ffi!(s.parse::<TransactionId>()).into();

    0
}
