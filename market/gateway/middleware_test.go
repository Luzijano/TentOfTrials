package gateway

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestGetClientIP(t *testing.T) {
	tests := []struct {
		name       string
		remoteAddr string
		headers    map[string]string
		want       string
	}{
		{
			name:       "host port remote addr",
			remoteAddr: "203.0.113.10:4321",
			want:       "203.0.113.10",
		},
		{
			name:       "bare ip remote addr fallback",
			remoteAddr: "203.0.113.11",
			want:       "203.0.113.11",
		},
		{
			name:       "malformed remote addr fallback",
			remoteAddr: "not-a-host-port",
			want:       "not-a-host-port",
		},
		{
			name:       "x forwarded for uses first non empty client",
			remoteAddr: "10.0.0.1:1234",
			headers: map[string]string{
				"X-Forwarded-For": " 198.51.100.7, 198.51.100.8 ",
			},
			want: "198.51.100.7",
		},
		{
			name:       "x forwarded for skips empty entries",
			remoteAddr: "10.0.0.1:1234",
			headers: map[string]string{
				"X-Forwarded-For": " , 198.51.100.9 ",
			},
			want: "198.51.100.9",
		},
		{
			name:       "x real ip is used when forwarded for absent",
			remoteAddr: "10.0.0.1:1234",
			headers: map[string]string{
				"X-Real-IP": " 198.51.100.10 ",
			},
			want: "198.51.100.10",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(http.MethodGet, "/orders", nil)
			req.RemoteAddr = tt.remoteAddr
			for key, value := range tt.headers {
				req.Header.Set(key, value)
			}

			if got := getClientIP(req); got != tt.want {
				t.Fatalf("getClientIP() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestRateLimitMiddlewareUsesResolvedClientIP(t *testing.T) {
	middleware := RateLimitMiddleware(1, 1)
	var seenClientIP string
	handler := middleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		seenClientIP = getClientIP(r)
		w.WriteHeader(http.StatusNoContent)
	}))

	first := httptest.NewRequest(http.MethodGet, "/orders", nil)
	first.RemoteAddr = "10.0.0.1:1234"
	first.Header.Set("X-Forwarded-For", "198.51.100.21")
	handler.ServeHTTP(httptest.NewRecorder(), first)
	if seenClientIP != "198.51.100.21" {
		t.Fatalf("handler saw client IP %q, want forwarded client", seenClientIP)
	}

	second := httptest.NewRequest(http.MethodGet, "/orders", nil)
	second.RemoteAddr = "10.0.0.2:5678"
	second.Header.Set("X-Forwarded-For", "198.51.100.21")
	second = second.WithContext(context.WithValue(second.Context(), ContextKeyClientIP, "ignored-context-value"))
	response := httptest.NewRecorder()
	handler.ServeHTTP(response, second)

	if response.Code != http.StatusTooManyRequests {
		t.Fatalf("second request status = %d, want %d", response.Code, http.StatusTooManyRequests)
	}
}
