package internal

import (
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sazzer/spacegame/service/internal/infrastructure/server"
	"github.com/sirupsen/logrus"
)

// Service encapsulates the entire service
type Service struct {
	server server.Server
}

// New builds the entire service ready to work with
func New(databaseURL string) Service {
	_ = database.NewPostgresDatabase(databaseURL)
	server := server.New()
	return Service{server}
}

// Start will start the service listening on the given port
func (service Service) Start(port uint16) {
	logrus.WithField("port", port).Info("Starting...")
	service.server.Start(port)
}
