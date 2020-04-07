package health

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/stretchr/testify/assert"

	"github.com/sazzer/spacegame/service/integration"
)

func TestHealth(t *testing.T) {
	service := integration.NewTestService(t)
	defer service.Shutdown()

	response := service.Request(httptest.NewRequest("GET", "/health", nil))
	assert.Equal(t, http.StatusOK, response.Result().StatusCode)

	cupaloy.SnapshotT(t, response.Body)
}
