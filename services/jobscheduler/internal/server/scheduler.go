package server

import (
	"crypto/rand"
	"fmt"
	"strings"
	"time"

	"go.uber.org/zap"
)

// generateJobID generates a unique job ID
func generateJobID() string {
	// Generate a random 8-byte string and encode it as hex
	bytes := make([]byte, 8)
	if _, err := rand.Read(bytes); err != nil {
		// Fallback to timestamp-based ID if random generation fails
		return fmt.Sprintf("job-%d", time.Now().UnixNano())
	}

	return fmt.Sprintf("job-%x", bytes)
}

// sanitizeJobName sanitizes a job name to be Kubernetes-compatible
func sanitizeJobName(name string) string {
	// Convert to lowercase and replace invalid characters
	name = strings.ToLower(name)
	name = strings.ReplaceAll(name, "_", "-")
	name = strings.ReplaceAll(name, " ", "-")

	// Remove any characters that aren't alphanumeric or hyphens
	var result strings.Builder
	for _, r := range name {
		if (r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') || r == '-' {
			result.WriteRune(r)
		}
	}

	cleaned := result.String()

	// Ensure it doesn't start or end with a hyphen
	cleaned = strings.Trim(cleaned, "-")

	// Ensure it's not empty
	if cleaned == "" {
		cleaned = "unnamed-job"
	}

	// Ensure it's not too long (Kubernetes limit is 63 characters)
	if len(cleaned) > 63 {
		cleaned = cleaned[:63]
		cleaned = strings.TrimSuffix(cleaned, "-")
	}

	return cleaned
}

// HealthChecker provides health check functionality
type HealthChecker struct {
	logger *zap.Logger
}

// NewHealthChecker creates a new health checker
func NewHealthChecker(logger *zap.Logger) *HealthChecker {
	return &HealthChecker{
		logger: logger,
	}
}

// Check performs a basic health check
func (hc *HealthChecker) Check() error {
	// Add any health check logic here
	// For now, just return nil to indicate healthy
	return nil
}

// ReadinessCheck checks if the service is ready to accept requests
func (hc *HealthChecker) ReadinessCheck() error {
	// Add readiness check logic here
	// This could include checking database connections, external services, etc.
	return nil
}
