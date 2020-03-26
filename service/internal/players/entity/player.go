package entity

// Player represents a player in the game
type Player struct {
	identity                PlayerIdentity
	name                    string
	avatar                  string
	authenticationProviders []AuthenticationProvider
}

// NewPlayer constructs a new Player instance that has not yet been persisted
func NewPlayer(name string, avatar string, authenticationProviders []AuthenticationProvider) Player {
	return Player{
		name:                    name,
		avatar:                  avatar,
		authenticationProviders: authenticationProviders,
	}
}

// NewPersistedPlayer constructs a new Player instance from a persisted data store record
func NewPersistedPlayer(identity PlayerIdentity, name string, avatar string, authenticationProviders []AuthenticationProvider) Player {
	return Player{
		identity:                identity,
		name:                    name,
		avatar:                  avatar,
		authenticationProviders: authenticationProviders,
	}
}

// Identity returns the persisted identity of this Player
func (p Player) Identity() PlayerIdentity {
	return p.identity
}

// Name returns the display name of this Player
func (p Player) Name() string {
	return p.name
}

// Avatar returns the URL to the Avatar for this Player
func (p Player) Avatar() string {
	return p.avatar
}

// AuthenticationProviders returns the set of Authentication Providers that this Player can use to log in
func (p Player) AuthenticationProviders() []AuthenticationProvider {
	return p.authenticationProviders
}
