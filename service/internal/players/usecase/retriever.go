package usecase

import "github.com/sazzer/spacegame/service/internal/players/entity"

// PlayerRetriever describes ways to fetch player details
type PlayerRetriever interface {
	// GetPlayerByID allows us to search the repository for the player that has the given ID
	GetPlayerByID(id entity.PlayerID) *entity.Player
}
