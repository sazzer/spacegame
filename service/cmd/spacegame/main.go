package main

import (
	"fmt"

	_ "github.com/joho/godotenv/autoload"
	"github.com/sirupsen/logrus"
)

func main() {
	logrus.SetFormatter(&logrus.TextFormatter{
		FullTimestamp: true,
	})
	logrus.SetLevel(logrus.DebugLevel)

	config := LoadConfig()
	logrus.WithField("config", config).Debug("Loaded application config")

	fmt.Printf("hello, world\n")
}
