use super::{CPublicKey, CSecretKey};
use crate::{AccountId, Client, Transaction, TransactionCreateAccount, TransactionResponse, TransactionCryptoTransfer};
use libc::c_char;
use std::{ffi::CStr, mem};

// Transaction
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_set_operator(tx: *mut Transaction<()>, operator: AccountId) {
    debug_assert!(!tx.is_null());

    let mut tx = unsafe { Box::from_raw(tx) };
    tx.operator(operator);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_set_node(tx: *mut Transaction<()>, node: AccountId) {
    debug_assert!(!tx.is_null());

    let mut tx = unsafe { Box::from_raw(tx) };
    tx.node(node);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_set_memo(tx: *mut Transaction<()>, memo: *const c_char) {
    debug_assert!(!tx.is_null());
    debug_assert!(!memo.is_null());

    let memo = unsafe { CStr::from_ptr(memo) };
    let memo = memo.to_str().unwrap();

    let mut tx = unsafe { Box::from_raw(tx) };
    tx.memo(memo);

    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction_sign(tx: *mut Transaction<()>, secret: CSecretKey) {
    debug_assert!(!tx.is_null());

    let mut tx = unsafe { Box::from_raw(tx) };
    let secret = unsafe { mem::transmute(secret) };

    tx.sign(secret);

    mem::forget(tx);
}

// TODO: Use a macro to generate the `_exec` functions

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction__create_account__execute(
    tx: *mut Transaction<TransactionCreateAccount>,
) -> TransactionResponse {
    debug_assert!(!tx.is_null());

    let tx = unsafe { Box::from_raw(tx) };
    tx.execute()
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction__crypto_transfer__execute(
    tx: *mut Transaction<TransactionCryptoTransfer>,
) -> TransactionResponse {
    debug_assert!(!tx.is_null());

    let tx = unsafe { Box::from_raw(tx) };
    tx.execute()
}

// TransactionCreateAccount
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction__create_account__new(
    client: *mut Client,
) -> *mut Transaction<TransactionCreateAccount> {
    debug_assert!(!client.is_null());

    let client = unsafe { Box::from_raw(client) };

    let tx = Transaction::create_account(&client);
    mem::forget(client);

    let tx = Box::new(tx);

    Box::into_raw(tx)
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction__create_account__set_key(
    tx: *mut Transaction<TransactionCreateAccount>,
    public: CPublicKey,
) {
    debug_assert!(!tx.is_null());

    let mut tx = unsafe { Box::from_raw(tx) };
    let public = unsafe { mem::transmute(public) };

    tx.key(public);
    mem::forget(tx);
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction__create_account__set_initial_balance(
    tx: *mut Transaction<TransactionCreateAccount>,
    balance: u64,
) {
    debug_assert!(!tx.is_null());

    let mut tx = unsafe { Box::from_raw(tx) };

    tx.initial_balance(balance);
    mem::forget(tx);
}

// TransactionCryptoTransfer
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction__crypto_transfer__new(
    client: *mut Client,
) -> *mut Transaction<TransactionCryptoTransfer> {
    debug_assert!(!client.is_null());

    let client = unsafe { Box::from_raw(client) };

    let tx = Transaction::crypto_transfer(&client);
    mem::forget(client);

    let tx = Box::new(tx);

    Box::into_raw(tx)
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_transaction__crypto_transfer__add_transfer(
    tx: *mut Transaction<TransactionCryptoTransfer>,
    id: AccountId,
    amount: i64,
) {
    debug_assert!(!tx.is_null());

    let mut tx = unsafe { Box::from_raw(tx) };

    tx.transfer(id, amount);
    mem::forget(tx);
}
