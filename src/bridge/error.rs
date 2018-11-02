use crate::ErrorKind;
use failure::Error;
use libc::c_char;
use mbox::MString;
use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use slotmap::{DefaultKey, KeyData, SlotMap};
use std::ptr::null_mut;

pub(crate) static ERRORS: Lazy<Mutex<SlotMap<DefaultKey, Error>>> = sync_lazy! {
    Mutex::new(SlotMap::new())
};

#[macro_export]
macro_rules! try_ffi {
    ($expr:expr) => {
        match $expr {
            Ok(expr) => expr,
            Err(error) => {
                return slotmap::KeyData::from(
                    crate::bridge::ERRORS
                        .lock()
                        .insert(failure::Error::from(error)),
                )
                .as_ffi()
            }
        }
    };
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_error_message(error: u64) -> *mut c_char {
    match ERRORS.lock().remove(KeyData::from_ffi(error).into()) {
        Some(err) => MString::from_str(&err.to_string())
            .into_mbox_with_sentinel()
            .into_raw() as *mut c_char,

        None => null_mut(),
    }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_error_pre_check(error: u64) -> i8 {
    let key = KeyData::from_ffi(error).into();
    let mut errors = ERRORS.lock();

    let code = match errors.get(key) {
        None => return -1,
        Some(err) => {
            if let Some(ErrorKind::PreCheck(code)) = err.downcast_ref() {
                *code as i8
            } else {
                return -1;
            }
        }
    };

    // Remove this error (that was a pre-check fail)
    let _ = errors.remove(key);

    code
}
