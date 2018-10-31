package main

import (
	"fmt"
	"github.com/mehcode/hedera-sdk/hedera-sdk-go"
)

func main() {
	secret := hedera.GenerateSecretKey()
	fmt.Printf("secret = %v\n", secret)

	public := secret.Public()
	fmt.Printf("public = %v\n", public)
}
