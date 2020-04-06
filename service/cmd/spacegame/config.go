package main

import (
	"github.com/kelseyhightower/envconfig"
	"github.com/sirupsen/logrus"
)

// Config reprsents the configuration of the application
type Config struct {
	DatabaseURL string `envconfig:"DATABASE_URL" required:"true"`
}

func loadConfig() Config {
	var c Config
	err := envconfig.Process("", &c)
	if err != nil {
		logrus.WithError(err).Fatal("Failed to load config")
	}
	logrus.WithField("config", c).Info("Loaded config")

	return c
}
