package health_test

import (
	"net/http"
	"testing"

	"github.com/bradleyjkemp/cupaloy"
	"github.com/sazzer/spacegame/service/integration"
	"github.com/stretchr/testify/assert"
)

func TestHealth(t *testing.T) {
	integration.RunTest(t, func(service integration.TestableService) {
		req, err := http.NewRequest("GET", "/health", nil)
		if err != nil {
			t.Fatal(err)
		}
		res := service.ServeHTTP(req)

		assert.Equal(t, http.StatusOK, res.Code)
		assert.NoError(t, cupaloy.SnapshotMulti("headers", res.HeaderMap))
		assert.NoError(t, cupaloy.SnapshotMulti("body", res.Body))
	})
}
