package service

import "net/http"

// TestRequest will submit a test request to the server for it to process
func (s *Service) TestRequest(w http.ResponseWriter, r *http.Request) {
	s.server.TestRequest(w, r)
}
