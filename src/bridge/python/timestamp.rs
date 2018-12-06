use pyo3::prelude::*;
use pyo3::types::PyDateTime;
use chrono::{DateTime, Utc};
use chrono::prelude::*;

pub fn py_date_time(date_time: DateTime<Utc>, py: Python) -> PyResult<Py<PyDateTime>> {
    let year = date_time.year();
    let month = date_time.month() as u8;
    let day = date_time.day() as u8;
    let hour = date_time.hour() as u8;
    let minute = date_time.minute() as u8;
    let second = date_time.second() as u8;
    let microsecond = date_time.timestamp_subsec_micros();

    PyDateTime::new(
        py,
        year,
        month,
        day,
        hour,
        minute,
        second,
        microsecond,
        None,
    )
}
