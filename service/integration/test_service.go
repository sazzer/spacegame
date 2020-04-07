package integration

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/sazzer/spacegame/service/internal/infrastructure/service"
	testdatabase "github.com/sazzer/spacegame/service/testing/database"
	"github.com/stretchr/testify/assert"
)

// TestService wraps the running service for us to test
type TestService struct {
	container testdatabase.Container
	service   service.Service
}

// NewTestService creates a new test service to test against
func NewTestService(t *testing.T) TestService {
	container, err := testdatabase.NewDatabaseContainer()
	assert.NoError(t, err)

	settings := service.Settings{
		DatabaseURL: container.Url(),
	}
	service := service.NewService(settings)

	return TestService{
		container: container,
		service:   service,
	}
}

// Request allows us to send an HTTP Request to the service and get the response
func (t *TestService) Request(req *http.Request) *httptest.ResponseRecorder {
	rec := httptest.NewRecorder()

	t.service.TestRequest(rec, req)

	return rec
}

// Shutdown will stop the test service running
func (t *TestService) Shutdown() {
	t.container.Shutdown()
}
