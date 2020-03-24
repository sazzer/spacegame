package database

import (
	"github.com/jmoiron/sqlx"
	// Postgres database drivers
	_ "github.com/lib/pq"
	"github.com/sirupsen/logrus"
)

// PostgresDatabase represents a connection to the database
type PostgresDatabase struct {
	url string
	db  *sqlx.DB
}

// NewPostgresDatabase opens a new database connection pool to access the database with
func NewPostgresDatabase(url string) PostgresDatabase {
	db, err := sqlx.Open("postgres", url)
	if err != nil {
		logrus.WithField("url", url).WithError(err).Fatal("Failed to open database connection")
	}

	logrus.WithField("url", url).Debug("Connected to database")
	return PostgresDatabase{db: db, url: url}
}

// CheckComponentHealth checks if the database connection is healthy by performing a ping to the database
// and seeing what comes back
func (p PostgresDatabase) CheckComponentHealth() error {
	return p.db.Ping()
}
