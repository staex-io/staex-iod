package main

import (
	"os"
	"os/signal"
	"syscall"
	"time"

	mqtt "github.com/mochi-mqtt/server/v2"
	"github.com/mochi-mqtt/server/v2/hooks/auth"
	"github.com/mochi-mqtt/server/v2/listeners"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	log.Logger = log.
		Output(zerolog.ConsoleWriter{Out: os.Stdout, TimeFormat: time.RFC3339}).
		With().Caller().Logger().
		Level(zerolog.TraceLevel)

	server := mqtt.New(&mqtt.Options{})
	if err := server.AddHook(new(auth.AllowHook), nil); err != nil {
		log.Fatal().Err(err).Msg("failed to add auth hook to mqtt broker")
	}
	defer server.Close()

	tcp := listeners.NewTCP(listeners.Config{
		ID:      "tcp1",
		Address: "0.0.0.0:6699",
	})
	err := server.AddListener(tcp)
	if err != nil {
		log.Fatal().Err(err).Msg("failed to add listener to mqtt broker")
	}

	go func() {
		log.Info().Msg("starting to serve mqtt broker")
		if err := server.Serve(); err != nil {
			log.Fatal().Err(err).Msg("failed to serve mqtt broker")
		}
	}()

	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM, syscall.SIGQUIT)
	<-sigs
	log.Info().Msg("received termination signal")
}
