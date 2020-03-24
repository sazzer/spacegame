package health

// Status indicates whether a component or the entire system is health or not
type Status int

const (
	// HealthSuccess indicates that the component or system is healthy
	HealthSuccess Status = iota

	// HealthFailure indicates that the component or system is unhealthy
	HealthFailure Status = iota
)

// ComponentHealth represents the health of a single component
type ComponentHealth struct {
	health  Status
	message string
}

// Health returns the health status of this one component
func (c ComponentHealth) Health() Status {
	return c.health
}

// Message returns the health message of this one component
func (c ComponentHealth) Message() string {
	return c.message
}

// SystemHealth represents the health of the entire system
type SystemHealth struct {
	health     Status
	components map[string]ComponentHealth
}

// Health returns the health status of the system
func (s SystemHealth) Health() Status {
	return s.health
}

// Components returns the component health for the entire system
func (s SystemHealth) Components() map[string]ComponentHealth {
	return s.components
}

// Healthchecker represents the way to check the health of the entire system
type Healthchecker struct {
	components map[string]Component
}

// CheckSystemHealth checks the health of the entire system by checking the components that make it up
func (h *Healthchecker) CheckSystemHealth() SystemHealth {
	result := SystemHealth{
		health:     HealthSuccess,
		components: map[string]ComponentHealth{},
	}

	for key, component := range h.components {
		err := component.CheckComponentHealth()

		componentHealth := ComponentHealth{}
		if err == nil {
			componentHealth.health = HealthSuccess
		} else {
			componentHealth.health = HealthFailure
			componentHealth.message = err.Error()
			result.health = HealthFailure
		}

		result.components[key] = componentHealth
	}

	return result
}

// NewHealthchecker is used to create a new Healthchecker instance from the components
func NewHealthchecker() Healthchecker {
	return Healthchecker{components: map[string]Component{}}
}

// AddComponent is used to add a new component to the healthchecker
func (h Healthchecker) AddComponent(name string, component Component) Healthchecker {
	h.components[name] = component
	return h
}
