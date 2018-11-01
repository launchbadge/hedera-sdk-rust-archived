#![feature(test)]
#![warn(clippy::pedantic)]
#![allow(clippy::stutter)]

#[cfg(test)]
extern crate test;

mod bridge;
mod client;
mod duration;
mod key;
mod proto;
mod query;
mod timestamp;
mod transaction;
mod transaction_id;

#[macro_use]
mod id;

define_id!(
    account,
    AccountId,
    AccountID,
    set_accountNum,
    get_accountNum
);
define_id!(file, FileId, FileID, set_fileNum, get_fileNum);
define_id!(
    contract,
    ContractId,
    ContractID,
    set_contractNum,
    get_contractNum
);

pub use self::{
    bridge::*,
    client::Client,
    duration::Duration,
    key::{KeyPair, PublicKey, SecretKey},
    query::*,
    timestamp::Timestamp,
    transaction::*,
    transaction_id::TransactionId,
};
