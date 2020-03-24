package internal

import (
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sazzer/spacegame/service/internal/infrastructure/health"
	"github.com/sazzer/spacegame/service/internal/infrastructure/server"
	"github.com/sirupsen/logrus"
)

// Service encapsulates the entire service
type Service struct {
	server server.Server
}

// New builds the entire service ready to work with
func New(databaseURL string) Service {
	db := database.NewPostgresDatabase(databaseURL)
	database.MigrateDatabaseSchema(databaseURL, "file://./migrations")

	healthchecker := health.NewHealthchecker().AddComponent("database", &db)
	if systemHealth := healthchecker.CheckSystemHealth(); systemHealth.Health() != health.HealthSuccess {
		logrus.WithField("components", systemHealth.Components()).Fatal("System is not initially healthy")
	} else {
		logrus.Debug("System is initially healthy")
	}

	server := server.New([]server.Router{
		health.NewRoutes(healthchecker),
	})
	return Service{server}
}

// Start will start the service listening on the given port
func (service Service) Start(port uint16) {
	logrus.WithField("port", port).Info("Starting...")
	service.server.Start(port)
}
