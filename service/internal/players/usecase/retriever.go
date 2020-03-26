package usecase

import "github.com/sazzer/spacegame/service/internal/players/entity"

// PlayerRetriever describes ways to fetch player details
type PlayerRetriever interface {
	GetPlayerByID(id entity.PlayerID) *entity.Player
}
