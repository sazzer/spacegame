package main

import (
	"log"

	"github.com/kelseyhightower/envconfig"
)

// Config represents the configuration of the application, as loaded from the environment
type Config struct {
	Port uint16
}

// LoadConfig loads the configuration from the environment ready for us to use
func LoadConfig() Config {
	var config Config

	err := envconfig.Process("", &config)

	if err != nil {
		log.Fatal(err.Error())
	}

	return config
}
