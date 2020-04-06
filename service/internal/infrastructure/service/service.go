package service

import (
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sirupsen/logrus"
)

// Service is the actual overall service to run
type Service struct {
}

// NewService builds a new instance of the Service to work with
func NewService(settings Settings) Service {
	logrus.Info("Building Service")

	database.NewPostgresDatabase(settings.DatabaseURL)

	return Service{}
}

// Start the service running
func (s *Service) Start() {
	logrus.Info("Starting Service")
}
