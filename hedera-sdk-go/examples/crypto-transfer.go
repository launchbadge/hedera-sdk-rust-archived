package main

import (
	"fmt"
	"github.com/mehcode/hedera-sdk/hedera-sdk-go"
	"os"
	"time"
)

func main() {
	// Read and decode the operator secret key
	operatorSecret := hedera.SecretKeyFromString(os.Getenv("OPERATOR_SECRET"))

	// Read and decode target account
	targetAccountId := hedera.AccountIDFromString(os.Getenv("TARGET"))

	//
	// Connect to Hedera
	//

	client := hedera.Dial("testnet.hedera.com:50001")
	// TODO: client.SetRetryOnFailure(0) // default: 5
	defer client.Close()

	//
	// Get balance for target account
	//

	balance := client.GetAccountBalance(targetAccountId).Send()
	fmt.Printf("account balance = %v\n", balance.Balance)

	//
	// Transfer 100 cryptos to target
	//

	nodeAccountId := hedera.NewAccountID(0, 0, 3)
	operatorAccountID := hedera.NewAccountID(0, 0, 2)
	response := client.CryptoTransfer().
		Operator(operatorAccountID).
		Node(nodeAccountId).
		Memo("[test] hedera-sdk-go v2").
		// Move 100 out of operator account
		Transfer(operatorAccountID, -100).
		// And place in our new account
		Transfer(targetAccountId, 100).
		Sign(operatorSecret). // Sign it once as operator
		Sign(operatorSecret). // And again as sender
		Execute()

	transactionID := response.ID
	fmt.Printf("transferred; transaction = %v\n", transactionID)

	//
	// Get receipt to prove we sent ok
	//

	fmt.Printf("wait for 2s...\n")
	time.Sleep(2 * time.Second)

	receiptResponse := client.GetTransactionReceipt(transactionID).Send()

	if receiptResponse.Precheck != 0 {
		fmt.Printf("receiptResponse:pre-check != OK (%v)\n", receiptResponse.Precheck)
		return
	}

	fmt.Printf("wait for 2s...\n")
	time.Sleep(2 * time.Second)

	//
	// Get balance for target account (again)
	//

	balance = client.GetAccountBalance(targetAccountId).Send()
	if balance.Precheck != 0 {
		panic(fmt.Sprintf("balance:pre-check != OK (%v)", balance.Precheck))
	}

	fmt.Printf("account balance = %v\n", balance.Balance)
}
