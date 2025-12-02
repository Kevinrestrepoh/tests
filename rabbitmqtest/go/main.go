package main

import (
	"context"
	"encoding/json"
	"log"

	amqp "github.com/rabbitmq/amqp091-go"
)

func main() {
	ch, close := ConnectRabbitMQ("amqp://admin:admin@localhost:5672/")
	defer func() {
		ch.Close()
		close()
	}()

	q, err := ch.QueueDeclare("item_qeue", true, false, false, false, nil)
	if err != nil {
		log.Fatal(err)
	}

	item := Item{
		ID:       1,
		Quantity: 10,
	}
	data, err := json.Marshal(item)
	if err != nil {
		log.Fatal(err)
	}

	err = ch.PublishWithContext(context.Background(), "exchange", q.Name, false, false, amqp.Publishing{
		ContentType: "application/json",
		Body:        data,
	})
	if err != nil {
		log.Fatal(err)
	}
}
