use failure::Error;
use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use std::ptr::null_mut;
use slotmap::{DefaultKey, SlotMap, KeyData};
use mbox::MString;
use libc::c_char;

pub(crate) static ERRORS: Lazy<Mutex<SlotMap<DefaultKey, Error>>> = sync_lazy! {
    Mutex::new(SlotMap::new())
};

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_error_message(error: u64) -> *mut c_char {
    let error_key = KeyData::from_ffi(error);
    match ERRORS.lock().remove(error_key.into()) {
        Some(err) => {
            let message = err.to_string();

            MString::from_str(&message.to_string())
                .into_mbox_with_sentinel()
                .into_raw() as *mut c_char
        }

        None => {
            null_mut()
        }
    }
}
