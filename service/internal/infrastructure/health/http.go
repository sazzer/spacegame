package health

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

// Routes represents the routes for checking the health of the system
type Routes struct {
	healthchecker Healthchecker
}

// NewRoutes creates a new Routes instance wrapping the given Healthchecker
func NewRoutes(healthchecker Healthchecker) *Routes {
	return &Routes{healthchecker}
}

// RegisterRoutes will register some routes with the provided router
func (r *Routes) RegisterRoutes(router *echo.Echo) error {
	router.GET("/health", r.CheckHealth)

	return nil
}

type componentHealthModel struct {
	Health  Status `json:"health"`
	Message string `json:"message,omitempty"`
}

type systemHealthModel struct {
	Health     Status                          `json:"health"`
	Components map[string]componentHealthModel `json:"components"`
}

// CheckHealth is the HTTP route for actually checking the health of the system
func (r *Routes) CheckHealth(c echo.Context) error {
	result := r.healthchecker.CheckSystemHealth()

	statusCode := http.StatusOK
	if result.Health() == HealthFailure {
		statusCode = http.StatusServiceUnavailable
	}

	components := map[string]componentHealthModel{}
	for name, component := range result.Components() {
		components[name] = componentHealthModel{
			Health:  component.Health(),
			Message: component.Message(),
		}
	}

	return c.JSON(statusCode, systemHealthModel{
		Health:     result.Health(),
		Components: components,
	})
}
