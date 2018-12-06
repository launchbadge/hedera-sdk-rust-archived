mod client;
mod errors;
mod timestamp;
mod query_contract_get_bytecode;
mod query_crypto_get_account_balance;
mod query_file_get_contents;
mod query_get_transaction_receipt;
mod query_crypto_get_info;
mod transaction_receipt;

use self::client::PyClient;
use pyo3::prelude::*;

#[pymodinit]
fn hedera(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyClient>()?;

    Ok(())
}
