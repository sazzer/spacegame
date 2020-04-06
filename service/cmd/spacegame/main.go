package main

import (
	// Load the environment properties
	_ "github.com/joho/godotenv/autoload"
	"github.com/sirupsen/logrus"
)

func main() {
	logrus.SetFormatter(&logrus.TextFormatter{
		FullTimestamp: true,
	})
	logrus.WithField("user", "Tobo").Info("logged in")
}
