use super::errors::PyException;
use crate::{
    query::{Query, QueryCryptoGetInfo, QueryCryptoGetInfoResponse},
    Client, AccountId,
};
use pyo3::prelude::*;

#[pyclass(name = QueryCryptoGetInfo)]
pub(crate) struct PyQueryCryptoGetInfo {
    inner: Query<QueryCryptoGetInfoResponse>,
}

impl PyQueryCryptoGetInfo {
    pub(crate) fn new(client: &Client, account: AccountId) -> Self {
        Self {
            inner: QueryCryptoGetInfo::new(client, account),
        }
    }
}

#[pymethods]
impl PyQueryCryptoGetInfo {
    fn get(&mut self) -> PyResult<QueryCryptoGetInfoResponse> {
        self.inner.get().map_err(PyException)
    }

    fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}
