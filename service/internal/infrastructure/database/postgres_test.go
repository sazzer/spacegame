package database_test

import (
	"testing"

	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	testdatabase "github.com/sazzer/spacegame/service/testing/database"
	"github.com/stretchr/testify/assert"
)

func TestConnect(t *testing.T) {
	container, err := testdatabase.NewDatabaseContainer()
	assert.NoError(t, err)
	defer container.Shutdown()

	database.NewPostgresDatabase(container.Url())
}

func TestCheckHealth(t *testing.T) {
	container, err := testdatabase.NewDatabaseContainer()
	assert.NoError(t, err)
	defer container.Shutdown()

	db := database.NewPostgresDatabase(container.Url())

	err = db.CheckHealth()
	assert.NoError(t, err)
}
