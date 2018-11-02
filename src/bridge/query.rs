use crate::{
    AccountId, Client, Query, QueryGetAccountBalance, QueryGetAccountBalanceAnswer,
    QueryGetTransactionReceipt, QueryGetTransactionReceiptAnswer, TransactionId,
};
use std::mem;

macro_rules! impl_cost {
    ($name:ident($ty:ident)) => {
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(query: *mut Query<$ty>, out: *mut u64) -> u64 {
            debug_assert!(!query.is_null());

            *out = try_ffi!(Box::from_raw(query).cost());

            0
        }
    };
}

macro_rules! impl_answer {
    ($name:ident($ty:ident) -> $rty:ident) => {
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(query: *mut Query<$ty>, out: *mut $rty) -> u64 {
            debug_assert!(!query.is_null());

            *out = try_ffi!(Box::from_raw(query).answer());

            0
        }
    };
}

// QueryGetAccountBalance
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_query__get_account_balance__new(
    client: *mut Client,
    account: AccountId,
) -> *mut Query<QueryGetAccountBalance> {
    debug_assert!(!client.is_null());

    let client = Box::from_raw(client);

    let query = Query::get_account_balance(&*client, account);
    let query = Box::new(query);

    mem::forget(client);

    Box::into_raw(query)
}

impl_answer!(hedera_query__get_account_balance__answer(
    QueryGetAccountBalance
) -> QueryGetAccountBalanceAnswer);

impl_cost!(hedera_query__get_account_balance__cost(
    QueryGetAccountBalance
));

// QueryGetTransactionReceipt
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_query__get_transaction_receipt__new(
    client: *mut Client,
    transaction_id: TransactionId,
) -> *mut Query<QueryGetTransactionReceipt> {
    debug_assert!(!client.is_null());

    let client = Box::from_raw(client);

    let query = Query::get_transaction_receipt(&*client, transaction_id);
    let query = Box::new(query);

    mem::forget(client);

    Box::into_raw(query)
}

impl_answer!(hedera_query__get_transaction_receipt__answer(
    QueryGetTransactionReceipt
) -> QueryGetTransactionReceiptAnswer);

impl_cost!(hedera_query__get_transaction_receipt__cost(
    QueryGetTransactionReceipt
));
