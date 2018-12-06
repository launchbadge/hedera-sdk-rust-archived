#![allow(non_snake_case)]

use pyo3::{types::exceptions, PyErr};
use std::fmt::Display;

pub(crate) fn PyException(err: impl Display) -> PyErr {
    exceptions::Exception::py_err(err.to_string())
}

pub(crate) fn PyValueError(err: impl Display) -> PyErr {
    exceptions::ValueError::py_err(err.to_string())
}
