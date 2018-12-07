#[macro_export]
macro_rules! try_precheck {
    ($response:expr) => {
        match $response.get_nodeTransactionPrecheckCode().into() {
            crate::PreCheckCode::Ok => Ok($response),
            code => return Err(crate::ErrorKind::PreCheck(code))?,
        }
    };
}
