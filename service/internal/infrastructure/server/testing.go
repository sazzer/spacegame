package server

import (
	"net/http"
	"net/http/httptest"
)

// ServeHTTP will send a request to the server and return the response
func (server *Server) ServeHTTP(r *http.Request) *httptest.ResponseRecorder {
	w := httptest.NewRecorder()
	server.server.ServeHTTP(w, r)
	return w
}
