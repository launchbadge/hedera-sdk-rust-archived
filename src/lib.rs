#![feature(try_from, specialization)]
#![cfg_attr(test, feature(test))]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter, clippy::new_ret_no_self)]

#[cfg(test)]
extern crate test;

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

pub use self::{
    client::Client,
    error::ErrorKind,
    id::*,
    transaction::{PreCheckCode, TransactionId, TransactionStatus},
    claim::Claim,
};
