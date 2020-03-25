package integration

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/sazzer/spacegame/service/internal"
	testdatabase "github.com/sazzer/spacegame/service/testing/database"
	"github.com/sirupsen/logrus"
	"github.com/stretchr/testify/assert"
)

// TestableService represents a service that we can send HTTP Requests to and get the response
type TestableService interface {
	ServeHTTP(r *http.Request) *httptest.ResponseRecorder
}

// RunTest is a helper to actually run an integration test.
// This will set up the database, build a server against it and then execute the provided
// lambda to perform the actual test
func RunTest(t *testing.T, test func(TestableService)) {
	logrus.SetFormatter(&logrus.TextFormatter{
		FullTimestamp: true,
	})
	logrus.SetLevel(logrus.DebugLevel)

	db, err := testdatabase.NewDatabaseWrapper()
	assert.NoError(t, err)
	defer db.Close()

	url, err := db.URL()
	assert.NoError(t, err)

	service := internal.New(url)

	test(&service)
}
