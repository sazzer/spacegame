package database_test

import (
	"testing"

	"github.com/jmoiron/sqlx"
	"github.com/sazzer/spacegame/service/testing/database"
	"github.com/stretchr/testify/assert"

	// Postgres database drivers
	_ "github.com/lib/pq"
)

func TestContainerStarts(t *testing.T) {
	wrapper, err := database.NewDatabaseWrapper()
	assert.NoError(t, err)

	err = wrapper.Close()
	assert.NoError(t, err)
}

func TestContainerURL(t *testing.T) {
	wrapper, err := database.NewDatabaseWrapper()
	assert.NoError(t, err)

	defer wrapper.Close()

	url, err := wrapper.URL()
	assert.NoError(t, err)
	assert.NotEqual(t, "", url)
}

func TestContainerConnectable(t *testing.T) {
	wrapper, err := database.NewDatabaseWrapper()
	assert.NoError(t, err)
	defer wrapper.Close()

	url, err := wrapper.URL()
	assert.NoError(t, err)

	db, err := sqlx.Open("postgres", url)
	assert.NoError(t, err)

	err = db.Ping()
	assert.NoError(t, err)
}
