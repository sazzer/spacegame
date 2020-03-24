package server

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/sirupsen/logrus"
)

// Server encapsulates the web server to use
type Server struct {
	server *echo.Echo
}

// New creates a new instance of the web server
func New(routes []Router) Server {
	e := echo.New()
	e.Use(middleware.Logger())
	e.Use(middleware.Gzip())
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowCredentials: true,
	}))
	e.Use(middleware.Recover())

	for _, router := range routes {
		router.RegisterRoutes(e)
	}
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})

	for _, route := range e.Routes() {
		logrus.WithField("method", route.Method).WithField("path", route.Path).Info("Route")
	}
	return Server{server: e}
}

// Start will start the web server listening on the given port
func (server Server) Start(port uint16) {
	err := server.server.Start(fmt.Sprintf(":%d", port))
	if err != nil {
		logrus.WithError(err).Fatal("Failed to start web server")
	}
}
