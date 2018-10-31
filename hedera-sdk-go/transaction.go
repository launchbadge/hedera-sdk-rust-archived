package hedera

// #include "hedera-transaction.h"
import "C"

type Transaction struct {
	inner *C.HederaTransaction
}

type TransactionResponse struct {
	ID TransactionID
	Precheck uint8
}
