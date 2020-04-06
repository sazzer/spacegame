package players

import (
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
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
