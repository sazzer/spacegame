package health_test

import (
	"errors"
	"testing"

	"github.com/sazzer/spacegame/service/internal/infrastructure/health"
	"github.com/stretchr/testify/assert"
)

type MockComponent struct {
	result error
}

func (m MockComponent) CheckHealth() error {
	return m.result
}

func TestNoChecks(t *testing.T) {
	healthchecker := health.NewHealthChecker(map[string]health.Component{})

	result := healthchecker.CheckHealth()

	assert.Equal(t, health.HealthPass, result.Status)
	assert.Equal(t, 0, len(result.Components))
}

func TestPassingCheck(t *testing.T) {
	healthchecker := health.NewHealthChecker(map[string]health.Component{
		"passing": MockComponent{},
	})

	result := healthchecker.CheckHealth()

	assert.Equal(t, health.HealthPass, result.Status)
	assert.Equal(t, 1, len(result.Components))
	assert.Equal(t, health.HealthPass, result.Components["passing"].Status)
	assert.Equal(t, "", result.Components["passing"].Message)
}

func TestFailingCheck(t *testing.T) {
	healthchecker := health.NewHealthChecker(map[string]health.Component{
		"failing": MockComponent{result: errors.New("Oops")},
	})

	result := healthchecker.CheckHealth()

	assert.Equal(t, health.HealthFail, result.Status)
	assert.Equal(t, 1, len(result.Components))
	assert.Equal(t, health.HealthFail, result.Components["failing"].Status)
	assert.Equal(t, "Oops", result.Components["failing"].Message)
}

func TestMixedChecks(t *testing.T) {
	healthchecker := health.NewHealthChecker(map[string]health.Component{
		"passing": MockComponent{},
		"failing": MockComponent{result: errors.New("Oops")},
	})

	result := healthchecker.CheckHealth()

	assert.Equal(t, health.HealthFail, result.Status)
	assert.Equal(t, 2, len(result.Components))

	assert.Equal(t, health.HealthFail, result.Components["failing"].Status)
	assert.Equal(t, "Oops", result.Components["failing"].Message)

	assert.Equal(t, health.HealthPass, result.Components["passing"].Status)
	assert.Equal(t, "", result.Components["passing"].Message)
}
