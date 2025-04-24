package main

import (
	"fmt"
	"log"

	"github.com/stellar/go/clients/horizonclient"
)

func main() {
	fmt.Println("Horizon API")

	client := horizonclient.DefaultPublicNetClient

	accountRequest := horizonclient.AccountRequest{
		AccountID: "GCLWGQPMKXQSPF776IU33AH4PZNOOWNAWGGKVTBQMIC5IMKUNP3E6NVU",
	}

	account, err := client.AccountDetail(accountRequest)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Account ID: %s\n", account.AccountID)
	fmt.Printf("Balances:\n")
	for _, b := range account.Balances {
		fmt.Printf("Type: %s, Code: %s, Balance: %s\n", b.Asset.Type, b.Asset.Code, b.Balance)
	}
}
