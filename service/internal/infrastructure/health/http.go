package health

import (
	"net/http"

	"github.com/labstack/echo"
)

type componentHealthModel struct {
	Status  string `json:"status"`
	Message string `json:"message,omitempty"`
}

type systemHealthModel struct {
	Status     string                          `json:"status"`
	Components map[string]componentHealthModel `json:"components"`
}

// BuildRoutes will build the HTTP Route handlers for providing system health
func BuildRoutes(healthchecker Healthchecker) func(*echo.Echo) {
	return func(e *echo.Echo) {
		e.GET("/health", func(c echo.Context) error {
			health := healthchecker.CheckHealth()

			result := systemHealthModel{
				Status:     health.Status,
				Components: map[string]componentHealthModel{},
			}

			for key, value := range health.Components {
				result.Components[key] = componentHealthModel{
					Status:  value.Status,
					Message: value.Message,
				}
			}

			statusCode := http.StatusOK
			if result.Status == HealthFail {
				statusCode = http.StatusServiceUnavailable
			}

			return c.JSON(statusCode, result)
		})
	}
}
