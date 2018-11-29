use crate::timestamp::Timestamp;
use chrono::Utc;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_timestamp_now() -> Timestamp {
    Utc::now().into()
}
