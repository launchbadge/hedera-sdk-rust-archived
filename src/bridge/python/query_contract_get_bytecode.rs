use super::errors::PyException;
use crate::{
    query::{Query, QueryContractGetBytecode},
    Client, ContractId,
};
use pyo3::prelude::*;

#[pyclass(name = QueryContractGetBytecode)]
pub(crate) struct PyQueryContractGetBytecode {
    inner: Query<Vec<u8>>,
}

impl PyQueryContractGetBytecode {
    pub(crate) fn new(client: &Client, contract: ContractId) -> Self {
        Self {
            inner: QueryContractGetBytecode::new(client, contract),
        }
    }
}

#[pymethods]
impl PyQueryContractGetBytecode {
    fn get(&mut self) -> PyResult<Vec<u8>> {
        self.inner.get().map_err(PyException)
    }

    fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}
