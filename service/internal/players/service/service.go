package service

// PlayerService is a service structure for interacting with players
type PlayerService struct {
	repository PlayerRepository
}

// NewPlayerService creates a new Player Service
func NewPlayerService(repo PlayerRepository) PlayerService {
	return PlayerService{
		repository: repo,
	}
}
