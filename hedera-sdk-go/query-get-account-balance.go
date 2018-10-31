package hedera

// #include "hedera-query-get-account-balance.h"
import "C"
import "unsafe"

type QueryGetAccountBalance struct {
	inner *C.HederaQuery
}

type QueryGetAccountBalanceResponse struct {
	Precheck uint8
	Kind     uint8
	Cost     uint64
	Balance  uint64
}

func newQueryGetAccountBalance(
	client *Client,
	account AccountID,
) QueryGetAccountBalance {
	return QueryGetAccountBalance{
		C.hedera_query__get_account_balance__new(client.inner, account.c())}
}

func (query QueryGetAccountBalance) Send() QueryGetAccountBalanceResponse {
	response := C.hedera_query__get_account_balance__send(query.inner)
	return *((*QueryGetAccountBalanceResponse)(unsafe.Pointer(&response)))
}
