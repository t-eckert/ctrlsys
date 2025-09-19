package jobs

import (
	"fmt"
	"sync"

	"go.uber.org/zap"
)

// Registry manages job handlers for different job types
type Registry struct {
	handlers map[JobType]JobHandler
	mu       sync.RWMutex
	logger   *zap.Logger
}

// NewRegistry creates a new job registry
func NewRegistry(logger *zap.Logger) *Registry {
	return &Registry{
		handlers: make(map[JobType]JobHandler),
		logger:   logger,
	}
}

// RegisterHandler registers a job handler for a specific job type
func (r *Registry) RegisterHandler(jobType JobType, handler JobHandler) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if handler == nil {
		return fmt.Errorf("handler cannot be nil")
	}

	if handler.GetJobType() != jobType {
		return fmt.Errorf("handler job type (%s) does not match registration type (%s)",
			handler.GetJobType(), jobType)
	}

	if _, exists := r.handlers[jobType]; exists {
		return fmt.Errorf("handler for job type %s already registered", jobType)
	}

	r.handlers[jobType] = handler
	r.logger.Info("Registered job handler", zap.String("job_type", string(jobType)))

	return nil
}

// GetHandler retrieves a job handler for a specific job type
func (r *Registry) GetHandler(jobType JobType) (JobHandler, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	handler, exists := r.handlers[jobType]
	if !exists {
		return nil, fmt.Errorf("no handler registered for job type: %s", jobType)
	}

	return handler, nil
}

// ListJobTypes returns all registered job types
func (r *Registry) ListJobTypes() []JobType {
	r.mu.RLock()
	defer r.mu.RUnlock()

	types := make([]JobType, 0, len(r.handlers))
	for jobType := range r.handlers {
		types = append(types, jobType)
	}

	return types
}

// IsRegistered checks if a job type is registered
func (r *Registry) IsRegistered(jobType JobType) bool {
	r.mu.RLock()
	defer r.mu.RUnlock()

	_, exists := r.handlers[jobType]
	return exists
}

// GetHandlerCount returns the number of registered handlers
func (r *Registry) GetHandlerCount() int {
	r.mu.RLock()
	defer r.mu.RUnlock()

	return len(r.handlers)
}

// UnregisterHandler removes a job handler (primarily for testing)
func (r *Registry) UnregisterHandler(jobType JobType) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if _, exists := r.handlers[jobType]; !exists {
		return fmt.Errorf("no handler registered for job type: %s", jobType)
	}

	delete(r.handlers, jobType)
	r.logger.Info("Unregistered job handler", zap.String("job_type", string(jobType)))

	return nil
}

// InitializeDefaultHandlers registers all default job handlers
func (r *Registry) InitializeDefaultHandlers() error {
	// Register Timer job handler
	timerHandler := NewTimerJobHandler(r.logger)
	if err := r.RegisterHandler(JobTypeTimer, timerHandler); err != nil {
		return fmt.Errorf("failed to register timer job handler: %w", err)
	}

	// Future job handlers can be registered here:
	// weatherHandler := NewWeatherReporterJobHandler(r.logger)
	// if err := r.RegisterHandler(JobTypeWeatherReporter, weatherHandler); err != nil {
	//     return fmt.Errorf("failed to register weather reporter job handler: %w", err)
	// }

	r.logger.Info("Initialized default job handlers", zap.Int("handler_count", r.GetHandlerCount()))

	return nil
}
