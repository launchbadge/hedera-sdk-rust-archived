use super::{
    errors::PyValueError, query_crypto_get_account_balance::*, query_file_get_contents::*,
    query_get_transaction_receipt::*,
};
use crate::{AccountId, Client, TransactionId};
use pyo3::prelude::*;
use std::rc::Rc;

#[pyclass(name = Client)]
pub(crate) struct PyClient {
    inner: Rc<Client>,
}

#[pymethods]
impl PyClient {
    #[new]
    fn __new__(obj: &PyRawObject, address: String) -> PyResult<()> {
        let client = Client::new(&address).map_err(PyValueError)?;
        obj.init(move |_| PyClient {
            inner: Rc::new(client),
        })
    }

    fn account(&self, id: String) -> PyResult<PyPartialAccountMessage> {
        Ok(PyPartialAccountMessage {
            client: Rc::clone(&self.inner),
            account: id.parse().map_err(PyValueError)?,
        })
    }

    fn transaction(&self, id: String) -> PyResult<PyPartialTransactionMessage> {
        Ok(PyPartialTransactionMessage {
            client: Rc::clone(&self.inner),
            transaction: id.parse().map_err(PyValueError)?,
        })
    }

    fn file(&self, id: String) -> PyResult<PyPartialFileMessage> {
        Ok(PyPartialFileMessage)
    }
}

#[pyclass(name = PartialAccountMessage)]
pub(crate) struct PyPartialAccountMessage {
    client: Rc<Client>,
    account: AccountId,
}

#[pymethods]
impl PyPartialAccountMessage {
    fn balance(&self) -> PyResult<PyQueryCryptoGetAccountBalance> {
        Ok(PyQueryCryptoGetAccountBalance::new(
            &self.client,
            self.account,
        ))
    }
}

#[pyclass(name = PartialTransactionMessage)]
pub(crate) struct PyPartialTransactionMessage {
    client: Rc<Client>,
    transaction: TransactionId,
}

#[pymethods]
impl PyPartialTransactionMessage {
    fn receipt(&self) -> PyResult<PyQueryGetTransactionReceipt> {
        Ok(PyQueryGetTransactionReceipt::new(
            &self.client,
            self.transaction.clone(),
        ))
    }
}

#[pyclass(name = PartialFileMessage)]
pub(crate) struct PyPartialFileMessage {
    client: Rc<Client>,
    file: FileId,
}

#[pymethods]
impl PyPartialFileMessage {
    fn contents(&self) -> PyResult<PyQueryFileGetContents> {
        Ok(PyQueryFileGetContents::new(&self.client, self.file.clone()))
    }
}
