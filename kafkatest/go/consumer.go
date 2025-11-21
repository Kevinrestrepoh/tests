package main

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/confluentinc/confluent-kafka-go/kafka"
)

func consumer(url string) {
	consumer, err := kafka.NewConsumer(&kafka.ConfigMap{
		"bootstrap.servers": url,
		"group.id":          "rust",
		"auto.offset.reset": "earliest",
	})
	if err != nil {
		panic(err)
	}
	defer consumer.Close()

	err = consumer.SubscribeTopics([]string{"rust_topic"}, nil)
	if err != nil {
		log.Fatal(err)
	}

	for {
		msg, err := consumer.ReadMessage(-1)
		if err == nil {
			var ev Event
			if err := json.Unmarshal(msg.Value, &ev); err == nil {
				fmt.Printf("Received struct: %v\n", ev)
			} else {
				fmt.Println("Invalid JSON")
			}
		} else {
			fmt.Printf("Consumer error: %v (%v)\n", err, msg)
		}
	}
}
