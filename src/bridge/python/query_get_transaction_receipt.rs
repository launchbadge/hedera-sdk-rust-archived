use super::{errors::PyException, transaction_receipt::PyTransactionReceipt};
use crate::{
    query::{Query, QueryGetTransactionReceipt},
    transaction::TransactionReceipt,
    Client, TransactionId,
};
use pyo3::prelude::*;

#[pyclass(name = QueryGetTransactionReceipt)]
pub(crate) struct PyQueryGetTransactionReceipt {
    inner: Query<TransactionReceipt>,
}

impl PyQueryGetTransactionReceipt {
    pub(crate) fn new(client: &Client, transaction: TransactionId) -> Self {
        Self {
            inner: QueryGetTransactionReceipt::new(client, transaction),
        }
    }
}

#[pymethods]
impl PyQueryGetTransactionReceipt {
    fn get(&mut self) -> PyResult<PyTransactionReceipt> {
        self.inner
            .get()
            .map(|receipt| PyTransactionReceipt { inner: receipt })
            .map_err(PyException)
    }

    fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}
