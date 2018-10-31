package hedera

// #include "hedera-client.h"
import "C"

type Client struct {
	inner *C.HederaClient
}

func Dial(address string) Client {
	return Client{C.hedera_client_dial(C.CString(address))}
}

func (client *Client) Close() {
	C.hedera_client_close(client.inner)
}

// Query
// ----------------------------------------------------------------------------

func (client *Client) GetAccountBalance(id AccountID) QueryGetAccountBalance {
	return newQueryGetAccountBalance(client, id)
}

func (client *Client) GetTransactionReceipt(id TransactionID) QueryGetTransactionReceipt {
	return newQueryGetTransactionReceipt(client, id)
}

// Transaction
// ----------------------------------------------------------------------------

func (client *Client) CreateAccount() TransactionCreateAccount {
	return newTransactionCreateAccount(client)
}

func (client *Client) CryptoTransfer() TransactionCryptoTransfer {
	return newTransactionCryptoTransfer(client)
}
