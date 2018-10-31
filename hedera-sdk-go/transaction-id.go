package hedera

// #include <stdlib.h>
// #include "hedera-transaction-id.h"
import "C"
import "unsafe"

type TransactionID struct {
	AccountID AccountID
	TransactionValidStart Timestamp
}

func NewTransactionID(accountID AccountID) TransactionID {
	response := C.hedera_transaction_id_new(accountID.c())
	return *((*TransactionID)(unsafe.Pointer(&response)))
}

func (id TransactionID) c() C.HederaTransactionId {
	p := (*C.HederaTransactionId)(unsafe.Pointer(&id))
	return *p
}

func (id TransactionID) String() string {
	p := (*C.HederaTransactionId)(unsafe.Pointer(&id))
	bytes := C.hedera_transaction_id_to_str(p)
	defer C.free(unsafe.Pointer(bytes))

	return C.GoString(bytes)
}
