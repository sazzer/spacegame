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

// CheckHealth is the HTTP route for actually checking the health of the system
func (r *Routes) CheckHealth(c echo.Context) error {
	return c.String(http.StatusOK, "Hello, World!")
}

// RegisterRoutes will register some routes with the provided router
func (r *Routes) RegisterRoutes(router *echo.Echo) error {
	router.GET("/health", r.CheckHealth)

	return nil
}
