package hedera

// #include "hedera-account-id.h"
import "C"

type AccountID struct {
	Realm   int64 `json:"realm"`
	Shard   int64 `json:"shard"`
	Account int64 `json:"account"`
}

func NewAccountID(realm, shard, account int64) AccountID {
	return AccountID{Realm: realm, Shard: shard, Account: account}
}

func AccountIDFromString(s string) AccountID {
	return *accountIDFromC(C.hedera_account_id_from_str(C.CString(s)))
}

func accountIDFromC(id C.HederaAccountId) *AccountID {
	return &AccountID{
		Realm:   int64(id.realm),
		Shard:   int64(id.shard),
		Account: int64(id.account),
	}
}

func (id AccountID) c() C.HederaAccountId {
	return C.HederaAccountId{
		realm:   C.int64_t(id.Realm),
		shard:   C.int64_t(id.Shard),
		account: C.int64_t(id.Account),
	}
}
