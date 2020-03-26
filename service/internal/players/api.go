package players

import (
	"github.com/sazzer/spacegame/service/internal/players/entity"
	"github.com/sazzer/spacegame/service/internal/players/usecase"
)

// Player represents a player in the game
type Player entity.Player

// AuthenticationProvider represents the details needed to authenticate at a given provider
type AuthenticationProvider entity.AuthenticationProvider

// NewPlayer constructs a new Player instance that has not yet been persisted
var NewPlayer = entity.NewPlayer

// NewAuthenticationProvider constructs a new Authentication Provider
var NewAuthenticationProvider = entity.NewAuthenticationProvider

// PlayerRetriever describes ways to fetch player details
type PlayerRetriever = usecase.PlayerRetriever
