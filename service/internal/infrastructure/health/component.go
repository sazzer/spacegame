package health

// Component represents any component that can report on it's own health
type Component interface {
	// CheckHealth reports on the health of the component and returns an error if the component is unhealthy
	CheckHealth() error
}
