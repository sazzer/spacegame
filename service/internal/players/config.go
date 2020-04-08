package players

import (
	"github.com/labstack/echo"
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sazzer/spacegame/service/internal/players/http"
	"github.com/sazzer/spacegame/service/internal/players/repository"
	"github.com/sazzer/spacegame/service/internal/players/service"
	"github.com/sirupsen/logrus"
)

// BuildPlayersService will create the Players Service to use
func BuildPlayersService(database database.Database) service.PlayerService {
	logrus.Info("Building Players Service")
	repo := repository.NewPlayerRepository(database)

	return service.NewPlayerService(repo)
}

// BuildRoutes will build the HTTP Routes for working with Players
func BuildRoutes(service service.PlayerService) func(*echo.Echo) {
	return func(e *echo.Echo) {
		e.GET("/players/:id", http.GetPlayerByID(service))
	}
}
