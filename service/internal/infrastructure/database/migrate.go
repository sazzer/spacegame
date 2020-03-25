package database

//go:generate parcello -r -d migrations

import (
	"github.com/golang-migrate/migrate/v4"

	// Migrate drivers for Postgres
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	// Migrate drivers for loading migrations from the filesystem
	_ "github.com/golang-migrate/migrate/v4/source/file"

	"github.com/sirupsen/logrus"
)

// MigrateDatabaseSchema will cause the database schema to be migrated to the latest version
func MigrateDatabaseSchema(url string) error {
	m, err := migrate.NewWithSourceInstance("parcello", newParcelloSource(), url)
	if err != nil {
		logrus.WithError(err).WithField("url", url).Fatal("Failed to load migrations")
		return err
	}

	version, dirty, err := m.Version()
	logrus.WithField("version", version).WithField("dirty", dirty).Info("Migration start point")

	err = m.Up()
	if err != nil && err != migrate.ErrNoChange {
		logrus.WithError(err).WithField("url", url).Fatal("Failed to apply migrations")
		return err
	}

	version, dirty, err = m.Version()
	logrus.WithField("version", version).WithField("dirty", dirty).Info("Migration end point")

	return nil
}
