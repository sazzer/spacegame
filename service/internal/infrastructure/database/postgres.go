package database

import (
	"github.com/jmoiron/sqlx"
	"github.com/sirupsen/logrus"

	// Postgres drivers
	_ "github.com/lib/pq"
)

// PostgresDatabase represents an actual connection to a Postgres Database
type PostgresDatabase struct {

}

// NewPostgresDatabase creates a new connection to a Postgres database
func NewPostgresDatabase(url string) Database {
	logrus.WithField("url", url).Info("Creating database connection")

	_, err := sqlx.Connect("postgres", url)
	if err != nil {
		logrus.WithField("url", url).WithError(err).Fatal("Failed to connect to database")
	}

	return PostgresDatabase{}
}