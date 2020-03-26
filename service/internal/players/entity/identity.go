package entity

import (
	"time"

	"github.com/google/uuid"
)

// PlayerID represents the unique ID of a player
type PlayerID uuid.UUID

// PlayerIdentity represents the identity of a player that has been saved to the data store
type PlayerIdentity struct {
	id      PlayerID
	version uuid.UUID
	created time.Time
	updated time.Time
}

// NewPlayerIdentity constructs the identity of a player
func NewPlayerIdentity(id PlayerID, version uuid.UUID, created time.Time, updated time.Time) PlayerIdentity {
	return PlayerIdentity{
		id:      id,
		version: version,
		created: created,
		updated: updated,
	}
}

// ID returns the ID of this Player
func (p PlayerIdentity) ID() PlayerID {
	return p.id
}

// Version returns the version tag of this Player
func (p PlayerIdentity) Version() uuid.UUID {
	return p.version
}

// Created returns when this Player created their account
func (p PlayerIdentity) Created() time.Time {
	return p.created
}

// Updated returns when this Player last updated their account
func (p PlayerIdentity) Updated() time.Time {
	return p.updated
}
