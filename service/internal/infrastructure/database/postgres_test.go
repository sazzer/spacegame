package database_test

import (
	"context"
	"testing"

	"github.com/sazzer/spacegame/service/internal/infrastructure/database"
	"github.com/stretchr/testify/assert"
)

type Output struct {
	One int `db:"one"`
}

func TestSimpleQuery(t *testing.T) {
	db := NewTestDatabase(t)
	defer db.Close()

	ctx := context.Background()

	var output Output

	err := db.db.QueryOne(ctx, database.NewQuery("SELECT 1 AS one"), &output)
	assert.NoError(t, err)
	assert.Equal(t, 1, output.One)
}

func TestSimpleQueryNoRows(t *testing.T) {
	db := NewTestDatabase(t)
	defer db.Close()

	ctx := context.Background()

	var output Output

	err := db.db.QueryOne(ctx, database.NewQuery("SELECT 1 AS one WHERE 1 = 2"), &output)
	assert.Equal(t, database.ErrNoRows, err)
}

func TestSimpleQueryMultipleRows(t *testing.T) {
	db := NewTestDatabase(t)
	defer db.Close()

	ctx := context.Background()

	var output Output

	err := db.db.QueryOne(ctx, database.NewQuery("SELECT 1 AS one UNION SELECT 2 AS one"), &output)
	assert.Equal(t, database.ErrMultipleRows, err)
}
