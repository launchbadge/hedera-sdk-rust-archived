#![feature(try_from)]
#![cfg_attr(test, feature(test))]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter)]

#[cfg(test)]
extern crate test;

#[cfg(feature = "bridge-c")]
mod bridge;

mod client;
mod duration;
mod error;
mod id;
mod key;
mod proto;
mod query;
mod query_get_account_balance;
mod query_get_transaction_receipt;
mod timestamp;
mod transaction;
mod transaction_create_account;
mod transaction_crypto_delete;
mod transaction_crypto_transfer;
mod transaction_crypto_update;
mod transaction_id;

#[cfg(feature = "bridge-c")]
pub use self::bridge::*;

pub use self::{
    client::Client,
    error::ErrorKind,
    id::*,
    key::{KeyPair, PublicKey, SecretKey},
    query::Query,
    query_get_account_balance::*,
    query_get_transaction_receipt::*,
    transaction::*,
    transaction_create_account::TransactionCreateAccount,
    transaction_crypto_delete::TransactionCryptoDelete,
    transaction_crypto_transfer::TransactionCryptoTransfer,
    transaction_crypto_update::TransactionCryptoUpdate,
    transaction_id::TransactionId,
};
