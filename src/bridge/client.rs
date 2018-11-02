use crate::Client;
use libc::c_char;
use std::ffi::CStr;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_client_dial(address: *const c_char) -> *mut Client {
    debug_assert!(!address.is_null());

    let address = CStr::from_ptr(address);
    let address = address.to_string_lossy();

    let client = Client::new(address);
    let client = Box::new(client);

    Box::into_raw(client)
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_client_close(client: *mut Client) {
    debug_assert!(!client.is_null());

    // Take and drop the client causing resource de-allocation
    let _ = Box::from_raw(client);
}
