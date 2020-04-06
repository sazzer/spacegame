package database

import "github.com/sirupsen/logrus"

// Migrate will cause the database schema to be updated to the latest version
func Migrate(db Database) error {
	logrus.Info("Migrating database to latest schema version")

	return nil
}
