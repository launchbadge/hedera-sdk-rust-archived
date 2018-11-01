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
                .as_ffi();
            }
        }
    };
}
