package database

import (
	"testing"

	testdatabase "github.com/sazzer/spacegame/service/testing/database"
	"github.com/stretchr/testify/assert"
)

// TestDatabase is the test database to work with
type TestDatabase struct {
	wrapper testdatabase.Wrapper
	db      PostgresDatabase
}

// NewTestDatabase will create a new test database, starting the database wrapper,
// migrating the schema and opening the connection pool
func NewTestDatabase(t *testing.T) TestDatabase {
	wrapper, err := testdatabase.NewDatabaseWrapper()
	assert.NoError(t, err)

	url, err := wrapper.URL()
	assert.NoError(t, err)

	err = MigrateDatabaseSchema(url)
	assert.NoError(t, err)

	db := NewPostgresDatabase(url)

	return TestDatabase{
		wrapper: wrapper,
		db:      db,
	}
}

// Close will close the test database when we're finished
func (t *TestDatabase) Close() {
	t.wrapper.Close()
}

// DB returns the actual database connection
func (t *TestDatabase) DB() PostgresDatabase {
	return t.db
}
