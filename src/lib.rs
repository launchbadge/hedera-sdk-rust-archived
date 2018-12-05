#![feature(try_from)]
#![cfg_attr(test, feature(test))]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter, clippy::new_ret_no_self)]

#[cfg(test)]
extern crate test;

#[cfg(feature = "bridge-c")]
mod bridge;

pub mod client;
pub mod crypto;
mod duration;
mod error;
mod id;
mod proto;
pub mod query;
mod query_crypto_get_account_balance;
mod query_crypto_get_account_records;
mod query_crypto_get_info;
mod query_file_get_contents;
mod query_file_get_info;
mod query_get_transaction_receipt;
mod query_transaction_get_record;
// todo: mod query_contract_get_info;
mod timestamp;
pub mod transaction;

#[cfg(feature = "bridge-c")]
pub use self::bridge::*;

pub use self::{
    client::Client,
    error::ErrorKind,
    id::*,
    transaction::{PreCheckCode, TransactionId, TransactionStatus},
};
