package usecase

import "github.com/sazzer/spacegame/service/internal/players/entity"

// PlayerRepository represents the way to interact with the data store
type PlayerRepository interface {
	// GetPlayerByID allows us to search the repository for the player that has the given ID
	GetPlayerByID(id entity.PlayerID) *entity.Player
}
