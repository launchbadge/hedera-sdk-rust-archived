#![warn(clippy::pedantic, future_incompatible, unreachable_pub)]
#![allow(clippy::stutter, clippy::new_ret_no_self)]

#[macro_use]
mod macros;

mod claim;
pub mod client;
mod crypto;
mod duration;
mod entity;
mod error;
mod id;
mod info;
mod proto;
pub mod query;
mod response;
mod timestamp;
pub mod transaction;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;
mod transaction_status;

pub use self::{
    claim::Claim,
    client::Client,
    crypto::{PublicKey, SecretKey, Signature},
    entity::Entity,
    error::ErrorKind,
    id::*,
    info::{AccountInfo, ContractInfo, FileInfo},
    response::PreCheckCode,
    transaction_id::TransactionId,
    transaction_receipt::TransactionReceipt,
    transaction_record::TransactionRecord,
    transaction_status::TransactionStatus,
};
