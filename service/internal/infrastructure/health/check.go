package health

// Component is an interface implemented by anything that can report on it's own health
type Component interface {
	// CheckComponentHealth is used to check the health of the one component
	CheckComponentHealth() error
}
