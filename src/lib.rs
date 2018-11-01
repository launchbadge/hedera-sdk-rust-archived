#![feature(test)]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter)]

#[cfg(test)]
extern crate test;

mod bridge;
mod client;
mod duration;
mod error;
mod id;
mod key;
mod proto;
mod query;
mod timestamp;
mod transaction;
mod transaction_id;

pub use self::{
    bridge::*,
    client::Client,
    duration::Duration,
    error::ErrorKind,
    id::*,
    key::{KeyPair, PublicKey, SecretKey},
    query::*,
    timestamp::Timestamp,
    transaction::*,
    transaction_id::TransactionId,
};
