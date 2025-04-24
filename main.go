package main

import "fmt"

func main() {
	fmt.Println("horizon api")

	client := hClient.DefaultPublicNetClient
	accountRequest := hClient.AccountRequest{AccountID: "GCLWGQPMKXQSPF776IU33AH4PZNOOWNAWGGKVTBQMIC5IMKUNP3E6NVU"}

	account, err := client.AccountDetail(accountRequest)
    if err != nil {
        fmt.Println(err)
        return
    }
    
    fmt.Print(account)


}
