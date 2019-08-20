macro_rules! try_precheck {
    ($response:expr) => {
        match $response.get_nodeTransactionPrecheckCode().into() {
            crate::Status::Ok => Ok($response),
            code => return Err(crate::ErrorKind::PreCheck(code))?,
        }
    };
}
