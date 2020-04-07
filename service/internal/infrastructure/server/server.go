package server

import (
	"fmt"

	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
	"github.com/sirupsen/logrus"
)

// Server represents the actual HTTP Server
type Server struct {
	server *echo.Echo
}

// NewServer constructs a new HTTP Server
func NewServer() Server {
	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{}))

	return Server{server: e}
}

// Start will start the server listening for connections
func (server Server) Start(port uint16) error {
	logrus.WithField("port", port).Info("Starting HTTP Server")

	return server.server.Start(fmt.Sprintf(":%d", port))
}
