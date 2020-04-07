package server

import "net/http"

// TestRequest will submit a test request to the server for it to process
func (server *Server) TestRequest(w http.ResponseWriter, r *http.Request) {
	server.server.ServeHTTP(w, r)
}
