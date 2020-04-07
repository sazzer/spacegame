package database

import (
	"github.com/jmoiron/sqlx"
	"github.com/sirupsen/logrus"

	// Postgres drivers
	_ "github.com/lib/pq"
)

// PostgresDatabase represents an actual connection to a Postgres Database
type PostgresDatabase struct {
	db *sqlx.DB
}

// NewPostgresDatabase creates a new connection to a Postgres database
func NewPostgresDatabase(url string) PostgresDatabase {
	logrus.WithField("url", url).Info("Creating database connection")

	db, err := sqlx.Connect("postgres", url)
	if err != nil {
		logrus.WithField("url", url).WithError(err).Fatal("Failed to connect to database")
	}

	return PostgresDatabase{db: db}
}

// CheckHealth reports on the health of the component and returns an error if the component is unhealthy
func (db PostgresDatabase) CheckHealth() error {
	_, err := db.db.Exec("SELECT 1")
	return err
}
