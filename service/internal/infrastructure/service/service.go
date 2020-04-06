package service

import "github.com/sirupsen/logrus"

// Service is the actual overall service to run
type Service struct {
}

// NewService builds a new instance of the Service to work with
func NewService() Service {
	logrus.Info("Building Service")

	return Service{}
}

// Start the service running
func (s *Service) Start() {
	logrus.Info("Starting Service")
}
