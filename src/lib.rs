#![warn(clippy::pedantic, future_incompatible, unreachable_pub)]
#![allow(clippy::stutter, clippy::new_ret_no_self, clippy::module_inception)]

#[macro_use]
mod macros;

mod argument;
pub mod call_params;
mod call_param_utils;
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
pub mod status;
pub mod solidity_util;
mod timestamp;
pub mod transaction;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;
pub mod function_result;
pub mod function_selector;

pub use self::{
    claim::Claim,
    client::Client,
    crypto::{PublicKey, SecretKey, Signature},
    entity::Entity,
    error::ErrorKind,
    id::*,
    info::{AccountInfo, ContractInfo, FileInfo},
    status::Status,
    transaction_id::TransactionId,
    transaction_receipt::TransactionReceipt,
    transaction_record::{TransactionRecord, TransactionRecordBody},
};

use once_cell::{sync::Lazy};
use parking_lot::Mutex;
use tokio::runtime::Runtime;

// Used to provide a blocking API for Query and Transaction execution
static RUNTIME: Lazy<Mutex<Runtime>> = Lazy::new(|| {
    Mutex::new(Runtime::new().unwrap())
});
