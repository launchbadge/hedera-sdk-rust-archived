#![feature(async_await, await_macro, futures_api)]
#![warn(clippy::pedantic, future_incompatible, unreachable_pub)]
#![allow(clippy::stutter, clippy::new_ret_no_self, clippy::module_inception)]

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
mod status;
mod timestamp;
pub mod transaction;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;

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

use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use tokio::runtime::Runtime;

// Used to provide a blocking API for Query and Transaction execution
static RUNTIME: Lazy<Mutex<Runtime>> = sync_lazy! { Mutex::new(Runtime::new().unwrap()) };
