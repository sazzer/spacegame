package server

import "github.com/labstack/echo/v4"

// Router represents any part of the system that can contribute routes to the webapp
type Router interface {
	// RegisterRoutes will register some routes with the provided router
	RegisterRoutes(router *echo.Echo) error
}
