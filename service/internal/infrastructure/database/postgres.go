package database

import (
	"context"

	"github.com/jmoiron/sqlx"
	// Postgres database drivers
	_ "github.com/lib/pq"
	"github.com/sirupsen/logrus"
)

// PostgresDatabase represents a connection to the database
type PostgresDatabase struct {
	db *sqlx.DB
}

// NewPostgresDatabase opens a new database connection pool to access the database with
func NewPostgresDatabase(url string) PostgresDatabase {
	db, err := sqlx.Open("postgres", url)
	if err != nil {
		logrus.WithField("url", url).WithError(err).Fatal("Failed to open database connection")
	}

	logrus.WithField("url", url).Debug("Connected to database")
	return PostgresDatabase{db: db}
}

// CheckComponentHealth checks if the database connection is healthy by performing a ping to the database
// and seeing what comes back
func (p PostgresDatabase) CheckComponentHealth() error {
	return p.db.Ping()
}

// QueryOne executes a SQL query and returns the single row
func (p PostgresDatabase) QueryOne(context context.Context, query Query, output interface{}) error {
	logrus.WithField("query", query).Debug("Executing query")

	rows, err := p.db.NamedQueryContext(context, query.sql, query.binds)
	if err != nil {
		logrus.WithError(err).WithField("query", query).Error("Failed to execute query")
		return err
	}
	defer rows.Close()

	if !rows.Next() {
		logrus.WithField("query", query).Warn("Expected 1 row but got none")
		return ErrNoRows
	}

	err = rows.StructScan(output)
	if err != nil {
		logrus.WithError(err).WithField("query", query).Error("Failed to parse database row")
		return err
	}

	if rows.Next() {
		logrus.WithField("query", query).Warn("Expected 1 row but got 2 or more")
		return ErrMultipleRows
	}

	return nil
}
