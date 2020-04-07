package health

// Healthchecker is responsible for checking the overall health of the system
type Healthchecker struct {
	components map[string]Component
}

// HealthPass Indicates that the component was healthy
const HealthPass = "Healthy"

// HealthFail Indicates that the component was unhealthy
const HealthFail = "Unhealthy"

// ComponentHealth represents the health of one particular component
type ComponentHealth struct {
	Status  string
	Message string
}

// SystemHealth represents the health of the entire system
type SystemHealth struct {
	Status     string
	Components map[string]ComponentHealth
}

// NewHealthChecker will create a new healthchecker for checking the system health
func NewHealthChecker(components map[string]Component) Healthchecker {
	return Healthchecker{
		components: components,
	}
}

// CheckHealth will check the health of the system
func (h Healthchecker) CheckHealth() SystemHealth {
	results := map[string]ComponentHealth{}
	systemHealth := HealthPass

	for name, component := range h.components {
		result := component.CheckHealth()
		if result == nil {
			results[name] = ComponentHealth{
				Status: HealthPass,
			}
		} else {
			results[name] = ComponentHealth{
				Status:  HealthFail,
				Message: result.Error(),
			}
			systemHealth = HealthFail
		}
	}

	return SystemHealth{
		Status:     systemHealth,
		Components: results,
	}
}
