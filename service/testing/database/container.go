package database

import (
	"context"
	"fmt"

	"github.com/sirupsen/logrus"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

// Wrapper represents a wrapper around the docker database
type Wrapper struct {
	container testcontainers.Container
}

// NewDatabaseWrapper will create and start a new database, ready to use
func NewDatabaseWrapper() (Wrapper, error) {
	ctx := context.Background()

	req := testcontainers.ContainerRequest{
		Image:        "postgres:11.6-alpine",
		ExposedPorts: []string{"5432/tcp"},
		WaitingFor:   wait.ForListeningPort("5432"),
	}
	container, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})
	if err != nil {
		logrus.WithError(err).Error("Failed to start Postgres container")
		return Wrapper{}, err
	}

	return Wrapper{container: container}, nil
}

// URL returns the connection URL to use to connect to the database
func (w *Wrapper) URL() (string, error) {
	ctx := context.Background()

	host, err := w.container.Host(ctx)
	if err != nil {
		logrus.WithError(err).Error("Failed to get Postgres host")
		return "", err
	}

	ports, err := w.container.Ports(ctx)
	if err != nil {
		logrus.WithError(err).Error("Failed to get Postgres port")
		return "", err
	}

	port := ports["5432/tcp"][0]

	hostPort := port.HostPort

	return fmt.Sprintf("postgres://postgres:postgres@%s:%s?sslmode=disable", host, hostPort), nil
}

// Close will close down the database after use
func (w *Wrapper) Close() error {
	ctx := context.Background()

	err := w.container.Terminate(ctx)
	if err != nil {
		logrus.WithError(err).Error("Failed to stop Postgres container")
		return err
	}

	return nil
}
