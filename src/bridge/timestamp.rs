use crate::Timestamp;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_timestamp_now() -> Timestamp {
    Timestamp::now()
}
