package entity

// AuthenticationProvider represents the details needed to authenticate at a given provider
type AuthenticationProvider struct {
	provider    string
	providerID  string
	displayName string
}

// NewAuthenticationProvider constructs a new Authentication Provider
func NewAuthenticationProvider(provider, providerID, displayName string) AuthenticationProvider {
	return AuthenticationProvider{
		provider:    provider,
		providerID:  providerID,
		displayName: displayName,
	}
}

// Provider returns the identity of the provider that this data represents
func (a AuthenticationProvider) Provider() string {
	return a.provider
}

// ProviderID returns the identity of the Player with this Authentication Provider
func (a AuthenticationProvider) ProviderID() string {
	return a.providerID
}

// DisplayName returns the friendly name of the Player with this Authentication Provider
func (a AuthenticationProvider) DisplayName() string {
	return a.displayName
}
