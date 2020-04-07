package database

import (
	"context"
	"fmt"

	"github.com/sirupsen/logrus"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

// Container represents an actual Docker container running Postgres
type Container struct {
	container testcontainers.Container
}

// NewDatabaseContainer creates a new Container to work with
func NewDatabaseContainer() (Container, error) {
	ctx := context.Background()
	req := testcontainers.ContainerRequest{
		Image:        "postgres:11.6-alpine",
		ExposedPorts: []string{"5432/tcp"},
		WaitingFor:   wait.ForListeningPort("5432/tcp"),
	}
	container, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})

	if err != nil {
		return Container{}, err
	}
	return Container{container: container}, nil
}

// Shutdown will shut down the running container
func (c *Container) Shutdown() {
	ctx := context.Background()
	c.container.Terminate(ctx)
}

// Url will get the URL of the running database
func (c *Container) Url() string {
	ctx := context.Background()

	ip, err := c.container.Host(ctx)
	if err != nil {
		logrus.WithError(err).Fatal("Failed to get container host")
	}

	port, err := c.container.MappedPort(ctx, "5432")
	if err != nil {
		logrus.WithError(err).Fatal("Failed to get container port")
	}

	return fmt.Sprintf("postgres://postgres:postgres@%s:%d/postgres?sslmode=disable", ip, port.Int())
}
