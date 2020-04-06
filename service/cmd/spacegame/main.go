package main

import (
	// Load the environment properties
	_ "github.com/joho/godotenv/autoload"
	"github.com/sazzer/spacegame/service/internal/infrastructure/service"
	"github.com/sirupsen/logrus"
)

func main() {
	logrus.SetFormatter(&logrus.TextFormatter{
		FullTimestamp: true,
	})

	config := loadConfig()

	settings := service.Settings{
		DatabaseURL: config.DatabaseURL,
	}
	service := service.NewService(settings)
	service.Start()
}
