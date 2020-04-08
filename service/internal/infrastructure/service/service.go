package service

import (
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sazzer/spacegame/service/internal/infrastructure/health"
	"github.com/sazzer/spacegame/service/internal/infrastructure/server"
	"github.com/sazzer/spacegame/service/internal/players"
	"github.com/sirupsen/logrus"
)

// Service is the actual overall service to run
type Service struct {
	server server.Server
}

// NewService builds a new instance of the Service to work with
func NewService(settings Settings) Service {
	logrus.Info("Building Service")

	db := database.NewPostgresDatabase(settings.DatabaseURL)
	err := database.Migrate(db)
	if err != nil {
		logrus.WithError(err).Fatal("Failed to migrate database")
	}

	playerService := players.BuildPlayersService(db)

	healthchecker := health.NewHealthChecker(map[string]health.Component{
		"database": db,
	})

	startupHealth := healthchecker.CheckHealth()
	logrus.WithField("health", startupHealth).Info("System Health on startup")

	server := server.NewServer(
		health.BuildRoutes(healthchecker),
		players.BuildRoutes(playerService),
	)

	return Service{server: server}
}

// Start the service running
func (s *Service) Start(port uint16) {
	logrus.Info("Starting Service")
	s.server.Start(port)
}
