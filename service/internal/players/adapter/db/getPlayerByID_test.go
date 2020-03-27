package db_test

import (
	"testing"

	"github.com/google/uuid"
	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/sazzer/spacegame/service/internal/players/adapter/db"
	"github.com/sazzer/spacegame/service/internal/players/entity"
	"github.com/stretchr/testify/assert"
)

func TestGetUnknownPlayerByID(t *testing.T) {
	testDatabase := database.NewTestDatabase(t)
	defer testDatabase.Close()

	playerDatabase := db.NewPlayerDatabase(testDatabase.DB())

	player := playerDatabase.GetPlayerByID(entity.PlayerID(uuid.New()))
	assert.Nil(t, player)
}
