use crate::Status;
use failure_derive::Fail;

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "missing required field: `{}`", _0)]
    MissingField(&'static str),

    #[fail(display = "expected string of the format: {:?}", _0)]
    Parse(&'static str),

    #[fail(display = "pre-check failed with status: {:?}", _0)]
    PreCheck(Status),
}
