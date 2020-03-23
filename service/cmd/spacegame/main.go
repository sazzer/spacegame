package main

import (
	"fmt"

	_ "github.com/joho/godotenv/autoload"
)

func main() {
	_ = LoadConfig()

	fmt.Printf("hello, world\n")
}
