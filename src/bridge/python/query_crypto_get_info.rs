use super::{errors::PyException, timestamp::py_date_time};
use crate::{
    claim::Claim,
    query::{Query, QueryCryptoGetInfo, QueryCryptoGetInfoResponse},
    crypto::PublicKey,
    Client, AccountId,
};

use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDelta};

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
    fn get(&mut self) -> PyResult<QueryPyCryptoGetInfoResponse> {
        self.inner
            .get()
            .map(|response| QueryPyCryptoGetInfoResponse { inner: response })
            .map_err(PyException)
    }

    fn cost(&mut self) -> PyResult<u64> {
        self.inner.cost().map_err(PyException)
    }
}

#[pyclass(name = TransactionReceipt)]
pub(crate) struct QueryPyCryptoGetInfoResponse {
    pub(crate) inner: QueryCryptoGetInfoResponse,
}

#[pymethods]
impl QueryPyCryptoGetInfoResponse {
    fn account_id(&self) -> PyResult<String> {
        Ok(self.inner.account_id.to_string())
    }

    #[getter]
    fn contract_account_id(&self) -> PyResult<String> {
        Ok(self.inner.contract_account_id.to_string())
    }

    #[getter]
    fn deleted(&self) -> PyResult<bool> {
        Ok(self.inner.deleted as bool)
    }

    #[getter]
    fn proxy_account_id(&self) -> PyResult<String> {
        Ok(self.inner.proxy_account_id.to_string())
    }

    #[getter]
    fn proxy_fraction(&self) -> PyResult<i32> {
        Ok(self.inner.proxy_fraction as i32)
    }

    #[getter]
    fn proxy_received(&self) -> PyResult<i64> {
        Ok(self.inner.proxy_received as i64)
    }

    #[getter]
    fn key(&self) -> PyResult<PyPublicKey> {
        Ok(PyPublicKey { inner: self.inner.key.clone() })
    }

    #[getter]
    fn balance(&self) -> PyResult<u64> {
        Ok(self.inner.balance as u64)
    }

    #[getter]
    fn generate_send_record_threshold(&self) -> PyResult<u64> {
        Ok(self.inner.generate_send_record_threshold as u64)
    }

    #[getter]
    fn generate_receive_record_threshold(&self) -> PyResult<u64> {
        Ok(self.inner.generate_receive_record_threshold as u64)
    }

    #[getter]
    fn receiver_signature_required(&self) -> PyResult<bool> {
        Ok(self.inner.receiver_signature_required as bool)
    }


    fn get_expiration_time(&self, py: Python) -> PyResult<Py<PyDateTime>> {
        Ok(py_date_time(self.inner.expiration_time, py)?)
    }


    fn get_auto_renew_period(&self, py: Python) -> PyResult<Py<PyDelta>> {
        let renew_period = self.inner.auto_renew_period;
        let seconds = renew_period.as_secs() as i32;
        let microseconds = renew_period.subsec_micros() as i32;

        Ok(PyDelta::new(
            py,
            0,
            seconds,
            microseconds,
            false
        )?)
    }

    #[getter]
    fn claims(&self) -> PyResult<Vec<PyClaim>> {
        let claims = self.inner.claims.clone().into_iter().map(|claim| {
            PyClaim {
                inner: claim
            }
        }).collect();

        Ok(claims)
    }
}

#[pyclass(name = PublicKey)]
pub struct PyPublicKey{
    inner: PublicKey
}

#[pyclass(name = Claim)]
pub struct PyClaim {
    inner: Claim
}

#[pymethods]
impl PyClaim {
    #[getter]
    pub fn account(&self) -> PyResult<String> {
        Ok(self.inner.account.to_string())
    }

    #[getter]
    pub fn hash(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.hash.clone())
    }

    #[getter]
    pub fn keys(&self) -> PyResult<Vec<PyPublicKey>> {
        let keys = self.inner.keys.clone().into_iter().map(|key| {
            PyPublicKey {
                inner: key
            }
        }).collect();
        Ok(keys)
    }

}
