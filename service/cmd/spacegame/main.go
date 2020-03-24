package main

import (
	_ "github.com/joho/godotenv/autoload"
	"github.com/sazzer/spacegame/service/internal"
	"github.com/sirupsen/logrus"
)

func main() {
	logrus.SetFormatter(&logrus.TextFormatter{
		FullTimestamp: true,
	})
	logrus.SetLevel(logrus.DebugLevel)

	config := LoadConfig()
	logrus.WithField("config", config).Debug("Loaded application config")

	service := internal.New(config.DatabaseURL, "file://./migrations")
	service.Start(config.Port)
}
