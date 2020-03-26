package database

import (
	"errors"
)

// ErrNoRows indicates that a query that should have returned rows in fact didn't
var (
	ErrNoRows       = errors.New("No rows were returned")
	ErrMultipleRows = errors.New("Expected 1 row but got 2+")
)

// Database represents the ways that we can access the database
type Database interface {
}
