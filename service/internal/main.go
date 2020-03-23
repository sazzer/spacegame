package internal

import "github.com/sirupsen/logrus"

// Service encapsulates the entire service
type Service struct {
}

// New builds the entire service ready to work with
func New() Service {
	return Service{}
}

// Start will start the service listening on the given port
func (service Service) Start(port uint16) {
	logrus.WithField("port", port).Info("Starting...")
}
