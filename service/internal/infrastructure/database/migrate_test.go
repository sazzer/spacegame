package database_test

import (
	"testing"

	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	testdatabase "github.com/sazzer/spacegame/service/testing/database"
	"github.com/stretchr/testify/assert"
)

func TestMigrate(t *testing.T) {
	wrapper, err := testdatabase.NewDatabaseWrapper()
	assert.NoError(t, err)
	defer wrapper.Close()

	url, err := wrapper.URL()
	assert.NoError(t, err)

	err = database.MigrateDatabaseSchema(url)
	assert.NoError(t, err)
}

func TestMigrateAgain(t *testing.T) {
	wrapper, err := testdatabase.NewDatabaseWrapper()
	assert.NoError(t, err)
	defer wrapper.Close()

	url, err := wrapper.URL()
	assert.NoError(t, err)

	err = database.MigrateDatabaseSchema(url)
	assert.NoError(t, err)

	err = database.MigrateDatabaseSchema(url)
	assert.NoError(t, err)
}
