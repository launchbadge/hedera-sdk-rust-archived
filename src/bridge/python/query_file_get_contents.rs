use crate::{
    bridge::python::errors::PyException,
    query::{Query, QueryFileGetContents},
    Client, FileId,
};
use pyo3::prelude::*;

#[pyclass(name = QueryFileGetContents)]
pub(crate) struct PyQueryFileGetContents {
    inner: Query<Vec<u8>>,
}

impl PyQueryFileGetContents {
    pub(crate) fn new(client: &Client, file: FileId) -> Self {
        Self {
            inner: QueryFileGetContents::new(client, file),
        }
    }
}

#[pymethods]
impl PyQueryFileGetContents {
    fn get(&mut self) -> PyResult<Vec<u8>> {
        self.inner.get().map_err(PyException)
    }

    fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}
