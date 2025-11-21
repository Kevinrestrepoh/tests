package main

import (
	"encoding/json"
	"fmt"

	"github.com/confluentinc/confluent-kafka-go/kafka"
)

func producer(url string) {
	p, err := kafka.NewProducer(&kafka.ConfigMap{
		"bootstrap.servers": url,
	})
	if err != nil {
		panic(err)
	}
	defer p.Close()

	topic := "go_topic"

	go func() {
		for e := range p.Events() {
			switch ev := e.(type) {
			case *kafka.Message:
				if ev.TopicPartition.Error != nil {
					fmt.Printf("Delivery failed: %v\n", ev.TopicPartition)
				} else {
					fmt.Printf("Delivered: %v\n", ev.TopicPartition)
				}
			}
		}
	}()

	for i := range 20 {
		event := Event{
			ID:  uint32(i),
			Msg: fmt.Sprintf("Event #%d", i),
		}

		data, err := json.Marshal(event)
		if err != nil {
			panic(err)
		}

		p.Produce(&kafka.Message{
			TopicPartition: kafka.TopicPartition{Topic: &topic, Partition: kafka.PartitionAny},
			Value:          data,
		}, nil)
	}

	p.Flush(5000)
}
