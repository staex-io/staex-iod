package main

import (
	"context"
	"encoding/json"
	"fmt"
	"math/rand"
	"net/url"
	"os"
	"os/signal"
	"sync"
	"syscall"
	"time"

	"github.com/eclipse/paho.golang/autopaho"
	"github.com/eclipse/paho.golang/paho"
	mqtt "github.com/mochi-mqtt/server/v2"
	"github.com/mochi-mqtt/server/v2/hooks/auth"
	"github.com/mochi-mqtt/server/v2/listeners"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
)

type Topic string

const (
	TopicMeasurement Topic = "measurement"
)

type DataType string

const (
	DataTypeWindSpeed DataType = "wind_speed"
)

type Measurement[T any] struct {
	DataType DataType `json:"data_type"`
	Data     T        `json:"data"`
}

type WindSpeed struct {
	Value int    `json:"value"`
	Units string `json:"units"`
}

func main() {
	log.Logger = log.
		Output(zerolog.ConsoleWriter{Out: os.Stdout, TimeFormat: time.RFC3339}).
		With().Caller().Logger().
		Level(zerolog.TraceLevel)

	stopC := make(chan struct{})
	defer close(stopC)
	doneC := make(chan struct{})
	defer close(doneC)

	cmd := initCommands(stopC, doneC)
	if err := cmd.Execute(); err != nil {
		log.Error().Err(err).Msg("failed to execute root command")
		return
	}
}

func initCommands(stopC, doneC chan struct{}) *cobra.Command {
	root := &cobra.Command{Use: "weather-gatherer"}
	server := &cobra.Command{
		Use:   "server",
		Short: "Start weather gatherer server",
		Run: func(cmd *cobra.Command, args []string) {
			go func() {
				if err := startServer(stopC, doneC); err != nil {
					log.Fatal().Err(err).Msg("failed to process server")
				}
			}()
			wait(stopC, doneC)
		},
	}
	client := &cobra.Command{
		Use:   "client",
		Short: "Start weather gatherer client",
		Run: func(cmd *cobra.Command, args []string) {
			go func() {
				if err := startClient(stopC, doneC); err != nil {
					log.Fatal().Err(err).Msg("failed to process client")
				}
			}()
			wait(stopC, doneC)
		},
	}
	root.AddCommand(server)
	root.AddCommand(client)
	return root
}

func startServer(stopC, doneC chan struct{}) error {
	server := mqtt.New(&mqtt.Options{
		InlineClient: true,
	})
	if err := server.AddHook(new(auth.AllowHook), nil); err != nil {
		return fmt.Errorf("failed to add auth hook to mqtt broker: %w", err)
	}
	defer server.Close()
	tcp := listeners.NewTCP(listeners.Config{
		ID:      "tcp1",
		Address: "0.0.0.0:6699",
	})
	err := server.AddListener(tcp)
	if err != nil {
		return fmt.Errorf("failed to add listener to mqtt broker: %w", err)
	}
	log.Info().Msg("starting to serve mqtt broker")
	if err := server.Serve(); err != nil {
		return fmt.Errorf("failed to serve mqtt broker: %w", err)
	}

	wg := sync.WaitGroup{}
	stopSimulationC := make(chan struct{})
	defer close(stopSimulationC)
	wg.Add(1)
	go func() {
		defer wg.Done()
		log.Info().Msg("starting data simulation")
		for {
			timer := time.NewTimer(time.Second)
			select {
			case <-stopSimulationC:
				log.Info().Msg("data simulation was stopped")
				return
			case <-timer.C:
				log.Debug().Msg("simulation iteration")
				payload, err := json.Marshal(Measurement[WindSpeed]{
					DataType: DataTypeWindSpeed,
					Data: WindSpeed{
						Value: rand.Intn(253-0) + 0,
						Units: "mph",
					},
				})
				if err != nil {
					log.Error().Err(err).Msg("failed to marshal measurement")
					continue
				}
				if err := server.Publish(string(TopicMeasurement), payload, false, 0); err != nil {
					log.Error().Err(err).Msg("failed to publish new data")
				}
			}
		}
	}()

	<-stopC
	log.Debug().Msg("received a signal to stop server")

	stopSimulationC <- struct{}{}
	wg.Wait()

	doneC <- struct{}{}
	return nil
}

func startClient(stopC, doneC chan struct{}) error {
	u, err := url.Parse("mqtt://127.0.0.1:6699")
	if err != nil {
		return fmt.Errorf("failed to parse mqtt url: %w", err)
	}
	config := autopaho.ClientConfig{
		ServerUrls: []*url.URL{u},
		KeepAlive:  60,
		ClientConfig: paho.ClientConfig{
			OnPublishReceived: []func(paho.PublishReceived) (bool, error){
				func(event paho.PublishReceived) (bool, error) {
					log.Debug().
						Str("topic", event.Packet.Topic).Bytes("payload", event.Packet.Payload).
						Msg("received new event")
					return true, nil
				},
			},
		},
	}
	ctx := context.Background()
	conn, err := autopaho.NewConnection(ctx, config)
	if err != nil {
		return fmt.Errorf("failed to establish a connection with a server: %w", err)
	}
	awaitCtx, awaitCancel := context.WithTimeout(ctx, time.Second*10)
	defer awaitCancel()
	if err := conn.AwaitConnection(awaitCtx); err != nil {
		return fmt.Errorf("failed to wait for connection with server: %w", err)
	}
	if _, err := conn.Subscribe(ctx, &paho.Subscribe{
		Subscriptions: []paho.SubscribeOptions{{
			Topic: string(TopicMeasurement),
		}},
	}); err != nil {
		return fmt.Errorf("failed to subscribe for a topic: %w", err)
	}

	<-stopC
	log.Debug().Msg("received a signal to stop a client")

	ctx, cancel := context.WithTimeout(ctx, time.Second*10)
	defer cancel()
	if err := conn.Disconnect(ctx); err != nil {
		return fmt.Errorf("failed to disconnect from server: %w", err)
	}
	select {
	case <-conn.Done():
		log.Debug().Msg("successfully waited for client closing")
	case <-ctx.Done():
		return fmt.Errorf("failed to wait while client will be closed: %w", ctx.Err())
	}

	doneC <- struct{}{}
	return nil
}

func wait(stopC, doneC chan struct{}) {
	sigs := make(chan os.Signal, 1)
	defer close(sigs)
	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM, syscall.SIGQUIT)
	<-sigs
	log.Info().Msg("received termination signal")

	stopC <- struct{}{}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*30)
	defer cancel()
	select {
	case <-doneC:
		log.Info().Msg("process successfully stopped")
	case <-ctx.Done():
		log.Error().Msg("process is failed to stop by timeout")
	}
}
