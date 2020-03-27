package db

import "github.com/sazzer/spacegame/service/internal/infrastructure/database"

// PlayerDatabase represents the access to the database for working with players
type PlayerDatabase struct {
	db database.Database
}

// NewPlayerDatabase will construct a new PlayerDatabase instance
func NewPlayerDatabase(db database.Database) PlayerDatabase {
	return PlayerDatabase{db}
}
