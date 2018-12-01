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
mod query_get_account_balance;
mod query_get_transaction_receipt;
mod timestamp;
pub mod transaction;
mod transaction_admin_delete;
mod transaction_admin_recover;
mod transaction_contract_call;
mod transaction_contract_create;
mod transaction_crypto_create;
mod transaction_crypto_delete;
mod transaction_crypto_delete_claim;
mod transaction_crypto_transfer;
mod transaction_crypto_update;
mod transaction_file_append;
mod transaction_file_create;
mod transaction_file_delete;
mod transaction_file_update;
mod transaction_id;
mod transaction_receipt;
mod transaction_response;

#[cfg(feature = "bridge-c")]
pub use self::bridge::*;

pub use self::{
    client::Client, error::ErrorKind, id::*, transaction_id::TransactionId,
    transaction_receipt::TransactionStatus, transaction_response::PreCheckCode,
};
