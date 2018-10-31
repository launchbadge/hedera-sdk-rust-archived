package hedera

// #include "hedera-query-get-transaction-receipt.h"
import "C"

type QueryGetTransactionReceipt struct {
	inner *C.HederaQuery
}

type QueryGetTransactionReceiptResponse struct {
	// TODO: Make enum for [Precheck]
	Precheck uint8
	// TODO: Make enum for [Kind]
	Kind    uint8
	Cost    uint64
	Receipt TransactionReceipt
}

type TransactionReceipt struct {
	// TODO: Make enum for [Status]
	Status    uint8
	AccountID *AccountID
	// unsupported: ContractID *C.HederaContractId
	// unsupported: FileID *C.HederaFileId
}

func newQueryGetTransactionReceipt(
	client *Client,
	transactionID TransactionID,
) QueryGetTransactionReceipt {
	return QueryGetTransactionReceipt{
		C.hedera_query__get_transaction_receipt__new(client.inner, transactionID.c())}
}

func (query QueryGetTransactionReceipt) Send() QueryGetTransactionReceiptResponse {
	cResponse := C.hedera_query__get_transaction_receipt__send(query.inner)

	receipt := TransactionReceipt{Status: uint8(cResponse.answer.status)}

	if cResponse.answer.account_id != nil {
		receipt.AccountID = accountIDFromC(*cResponse.answer.account_id)
	}

	return QueryGetTransactionReceiptResponse{
		Precheck: uint8(cResponse.precheck),
		Kind:     uint8(cResponse.kind),
		Cost:     uint64(cResponse.cost),
		Receipt:  receipt,
	}
}
