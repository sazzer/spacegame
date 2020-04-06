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

	service := service.NewService()
	service.Start()
}
