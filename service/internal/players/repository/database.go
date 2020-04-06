package repository

import (
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sazzer/spacegame/service/internal/players/service"
)

// DatabasePlayerRepository is a Player Repository backed by a Database
type DatabasePlayerRepository struct {
	database database.Database
}

// NewPlayerRepository creates a new repository to use for dealing with player records
func NewPlayerRepository(database database.Database) service.PlayerRepository {
	return DatabasePlayerRepository{
		database: database,
	}
}
