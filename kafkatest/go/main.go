package main

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Fatal("Error loading .env")
	}
	url := os.Getenv("KAFKA_BROKER")

	producer(url)
	consumer(url)
}
