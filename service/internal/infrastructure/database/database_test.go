package database_test

import (
	"testing"

	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	testdatabase "github.com/sazzer/spacegame/service/testing/database"
	"github.com/stretchr/testify/assert"
)

// The test database to work with
type TestDatabase struct {
	wrapper testdatabase.Wrapper
	db      database.PostgresDatabase
}

// NewTestDatabase will create a new test database, starting the database wrapper,
// migrating the schema and opening the connection pool
func NewTestDatabase(t *testing.T) TestDatabase {
	wrapper, err := testdatabase.NewDatabaseWrapper()
	assert.NoError(t, err)

	url, err := wrapper.URL()
	assert.NoError(t, err)

	err = database.MigrateDatabaseSchema(url)
	assert.NoError(t, err)

	db := database.NewPostgresDatabase(url)

	return TestDatabase{
		wrapper: wrapper,
		db:      db,
	}
}

// Close will close the test database when we're finished
func (t TestDatabase) Close() {
	t.wrapper.Close()
}
