package main

import (
	"fmt"
	"github.com/hedera/hedera-sdk/hedera-sdk-go"
)

func main() {
	client := hedera.Dial("testnet.hedera.com:50001")
	defer client.Close()

	accountID := hedera.NewAccountID(0, 0, 2)

	// TODO: Allow strings to be passed for account ID here
	// TODO: Remove [.Kind(..)] and replace with [.Answer()] and [.Cost()]
	// balance := client.GetAccountBalance(accountID).Answer()
	response := client.GetAccountBalance(accountID).Send()

	// fmt.Printf("balance = %v\n", balance)
	fmt.Printf("balance = %v\n", response.Balance)
}
