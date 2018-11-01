use crate::{
    AccountId, Client, Query, QueryGetAccountBalance, QueryGetAccountBalanceAnswer,
    QueryGetTransactionReceipt, QueryGetTransactionReceiptAnswer, QueryResponse, TransactionId,
};
use std::mem::forget;

// TODO: Use a macro to one-line the `_send` functions

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_query__get_account_balance__send(
    query: *mut Query<QueryGetAccountBalance>,
) -> QueryResponse<QueryGetAccountBalanceAnswer> {
    debug_assert!(!query.is_null());

    let query = unsafe { Box::from_raw(query) };
    query.send()
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_query__get_transaction_receipt__send(
    query: *mut Query<QueryGetTransactionReceipt>,
) -> QueryResponse<QueryGetTransactionReceiptAnswer> {
    debug_assert!(!query.is_null());

    let query = unsafe { Box::from_raw(query) };
    query.send()
}

// QueryGetAccountBalance
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_query__get_account_balance__new(
    client: *mut Client,
    account: AccountId,
) -> *mut Query<QueryGetAccountBalance> {
    debug_assert!(!client.is_null());

    let client = unsafe { Box::from_raw(client) };

    let query = Query::<QueryGetAccountBalance>::new(&*client, account);
    let query = Box::new(query);

    forget(client);

    Box::into_raw(query)
}

// QueryGetTransactionReceipt
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn hedera_query__get_transaction_receipt__new(
    client: *mut Client,
    transaction_id: TransactionId,
) -> *mut Query<QueryGetTransactionReceipt> {
    debug_assert!(!client.is_null());

    let client = unsafe { Box::from_raw(client) };

    let query = Query::<QueryGetTransactionReceipt>::new(&*client, transaction_id);
    let query = Box::new(query);

    forget(client);

    Box::into_raw(query)
}
