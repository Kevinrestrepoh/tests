package main

import (
	"context"
	"encoding/json"
	"log"

	amqp "github.com/rabbitmq/amqp091-go"
)

func main() {
	ch, conn := ConnectRabbitMQ("amqp://admin:admin@localhost:5672/test")
	defer func() {
		ch.Close()
		conn.Close()
	}()

	q, err := Declare(ch)
	if err != nil {
		log.Fatalf("Failed to declar exchange or qeue: %v", err)
	}

	item := Item{
		ID:       1,
		Quantity: 10,
	}
	data, err := json.Marshal(item)
	if err != nil {
		log.Fatal(err)
	}

	err = ch.PublishWithContext(context.Background(), "test_exchange", q.Name, false, false, amqp.Publishing{
		ContentType: "application/json",
		Body:        data,
	})
	if err != nil {
		log.Fatal(err)
	}
}
