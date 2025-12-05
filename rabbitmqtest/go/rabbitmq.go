package main

import (
	"log"

	amqp "github.com/rabbitmq/amqp091-go"
)

func ConnectRabbitMQ(url string) (*amqp.Channel, *amqp.Connection) {
	conn, err := amqp.Dial(url)
	if err != nil {
		log.Fatalf("Failed to connect to RabbitMQ: %v", err)
	}

	ch, err := conn.Channel()
	if err != nil {
		log.Fatalf("Failed to connect to Channel: %v", err)
	}

	return ch, conn
}

func Declare(ch *amqp.Channel) (*amqp.Queue, error) {
	err := ch.ExchangeDeclare(
		"test_exchange",
		"direct",
		true,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		return nil, err
	}

	q, err := ch.QueueDeclare("item_qeue", true, false, false, false, nil)
	if err != nil {
		return nil, err
	}

	return &q, nil
}
