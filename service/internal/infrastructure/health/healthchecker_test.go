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

func (m MockComponent) CheckComponentHealth() error {
	return m.result
}

func TestHealthcheckerNoComponents(t *testing.T) {
	sut := health.NewHealthchecker()

	result := sut.CheckSystemHealth()

	assert.Equal(t, health.HealthSuccess, result.Health())
	assert.Empty(t, result.Components())
}

func TestHealthcheckerSuccessfulComponent(t *testing.T) {
	sut := health.NewHealthchecker().AddComponent("passing", MockComponent{})

	result := sut.CheckSystemHealth()

	assert.Equal(t, health.HealthSuccess, result.Health())
	assert.Len(t, result.Components(), 1)

	assert.Equal(t, health.HealthSuccess, result.Components()["passing"].Health())
	assert.Equal(t, "", result.Components()["passing"].Message())
}

func TestHealthcheckerFailingComponent(t *testing.T) {
	sut := health.NewHealthchecker().AddComponent("failing", MockComponent{result: errors.New("Oops")})

	result := sut.CheckSystemHealth()

	assert.Equal(t, health.HealthFailure, result.Health())
	assert.Len(t, result.Components(), 1)

	assert.Equal(t, health.HealthFailure, result.Components()["failing"].Health())
	assert.Equal(t, "Oops", result.Components()["failing"].Message())
}

func TestHealthcheckerMixedComponent(t *testing.T) {
	sut := health.NewHealthchecker().
		AddComponent("failing", MockComponent{result: errors.New("Oops")}).
		AddComponent("passing", MockComponent{})

	result := sut.CheckSystemHealth()

	assert.Equal(t, health.HealthFailure, result.Health())
	assert.Len(t, result.Components(), 2)

	assert.Equal(t, health.HealthFailure, result.Components()["failing"].Health())
	assert.Equal(t, "Oops", result.Components()["failing"].Message())

	assert.Equal(t, health.HealthSuccess, result.Components()["passing"].Health())
	assert.Equal(t, "", result.Components()["passing"].Message())
}
