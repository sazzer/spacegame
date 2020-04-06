package service

import (
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sazzer/spacegame/service/internal/players"
	"github.com/sirupsen/logrus"
)

// Service is the actual overall service to run
type Service struct {
}

// NewService builds a new instance of the Service to work with
func NewService(settings Settings) Service {
	logrus.Info("Building Service")

	db := database.NewPostgresDatabase(settings.DatabaseURL)
	err := database.Migrate(db)
	if err != nil {
		logrus.WithError(err).Fatal("Failed to migrate database")
	}

	players.BuildPlayersService(db)
	return Service{}
}

// Start the service running
func (s *Service) Start() {
	logrus.Info("Starting Service")
}
