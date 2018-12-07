#![feature(try_from, specialization)]
#![cfg_attr(test, feature(test))]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter, clippy::new_ret_no_self)]

#[cfg(test)]
extern crate test;

#[cfg(feature = "bridge-python")]
#[allow(unused_imports)]
#[macro_use]
extern crate pyo3;

#[cfg(any(
    feature = "bridge-c",
    feature = "bridge-python",
    feature = "bridge-java"
))]
mod bridge;

mod claim;
pub mod client;
pub mod crypto;
mod duration;
mod error;
mod id;
mod proto;
pub mod query;
mod timestamp;
pub mod transaction;

#[cfg(any(
    feature = "bridge-c",
    feature = "bridge-python",
    feature = "bridge-java"
))]
pub use self::bridge::*;

pub use self::{
    client::Client,
    error::ErrorKind,
    id::*,
    transaction::{PreCheckCode, TransactionId, TransactionStatus},
    claim::Claim,
};
