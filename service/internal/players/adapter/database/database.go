package database

import db "github.com/sazzer/spacegame/service/internal/infrastructure/database"

// PlayerDatabase represents the access to the database for working with players
type PlayerDatabase struct {
	db db.Database
}

// NewPlayerDatabase will construct a new PlayerDatabase instance
func NewPlayerDatabase(db db.Database) PlayerDatabase {
	return PlayerDatabase{db}
}
