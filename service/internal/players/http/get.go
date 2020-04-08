package http

import (
	"net/http"

	"github.com/labstack/echo"
	"github.com/sazzer/spacegame/service/internal/players/service"
)

// GetPlayerByID builds the handler function for getting a single player by their unique ID
func GetPlayerByID(service service.PlayerService) func(echo.Context) error {
	return func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello")
	}
}
