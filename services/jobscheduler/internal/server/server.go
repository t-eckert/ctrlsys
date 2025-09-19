package server

import (
	"context"
	"fmt"
	"net/http"

	"connectrpc.com/connect"
	"go.uber.org/zap"

	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/config"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/jobs"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/k8s"
	v1 "github.com/t-eckert/ctrlsys/gen/go/ctrlsys/jobscheduler/v1"
	"github.com/t-eckert/ctrlsys/gen/go/ctrlsys/jobscheduler/v1/v1connect"
)

// Server implements the JobScheduler ConnectRPC service
type Server struct {
	config     *config.Config
	jobCreator *k8s.JobCreator
	registry   *jobs.Registry
	logger     *zap.Logger
}

// NewServer creates a new ConnectRPC server instance
func NewServer(
	config *config.Config,
	jobCreator *k8s.JobCreator,
	registry *jobs.Registry,
	logger *zap.Logger,
) *Server {
	return &Server{
		config:     config,
		jobCreator: jobCreator,
		registry:   registry,
		logger:     logger,
	}
}

// ScheduleJob schedules a new job to run in Kubernetes
func (s *Server) ScheduleJob(ctx context.Context, req *connect.Request[v1.ScheduleJobRequest]) (*connect.Response[v1.ScheduleJobResponse], error) {
	msg := req.Msg
	s.logger.Info("Received ScheduleJob request",
		zap.String("job_id", msg.JobId),
		zap.String("job_name", msg.Name))

	// Validate request
	if err := s.validateScheduleJobRequest(msg); err != nil {
		s.logger.Error("Invalid ScheduleJob request",
			zap.String("job_id", msg.JobId),
			zap.Error(err))
		return nil, connect.NewError(connect.CodeInvalidArgument, fmt.Errorf("invalid request: %v", err))
	}

	// Generate job ID if not provided
	if msg.JobId == "" {
		msg.JobId = generateJobID()
	}

	// Create the job
	response, err := s.jobCreator.CreateJobFromRequest(ctx, msg)
	if err != nil {
		s.logger.Error("Failed to create job",
			zap.String("job_id", msg.JobId),
			zap.Error(err))
		return nil, connect.NewError(connect.CodeInternal, fmt.Errorf("failed to create job: %v", err))
	}

	s.logger.Info("Successfully scheduled job",
		zap.String("job_id", response.JobId),
		zap.String("k8s_job_name", response.K8SJobName))

	return connect.NewResponse(response), nil
}

// GetJobStatus retrieves the status of a scheduled job
func (s *Server) GetJobStatus(ctx context.Context, req *connect.Request[v1.GetJobStatusRequest]) (*connect.Response[v1.GetJobStatusResponse], error) {
	msg := req.Msg
	s.logger.Debug("Received GetJobStatus request", zap.String("job_id", msg.JobId))

	if msg.JobId == "" {
		return nil, connect.NewError(connect.CodeInvalidArgument, fmt.Errorf("job_id is required"))
	}

	jobInfo, err := s.jobCreator.GetJobInfo(ctx, msg.JobId, "")
	if err != nil {
		s.logger.Error("Failed to get job status",
			zap.String("job_id", msg.JobId),
			zap.Error(err))
		return nil, connect.NewError(connect.CodeNotFound, fmt.Errorf("job not found: %v", err))
	}

	response := &v1.GetJobStatusResponse{
		JobId:   msg.JobId,
		JobInfo: jobInfo,
		Status:  jobInfo.Status,
		Message: getStatusMessage(jobInfo.Status),
	}

	return connect.NewResponse(response), nil
}

// ListJobs lists jobs with optional filtering
func (s *Server) ListJobs(ctx context.Context, req *connect.Request[v1.ListJobsRequest]) (*connect.Response[v1.ListJobsResponse], error) {
	msg := req.Msg
	s.logger.Debug("Received ListJobs request",
		zap.String("namespace", msg.Namespace),
		zap.Any("label_selector", msg.LabelSelector),
		zap.Any("status_filter", msg.StatusFilter))

	response, err := s.jobCreator.ListJobs(ctx, msg)
	if err != nil {
		s.logger.Error("Failed to list jobs", zap.Error(err))
		return nil, connect.NewError(connect.CodeInternal, fmt.Errorf("failed to list jobs: %v", err))
	}

	s.logger.Debug("Successfully listed jobs", zap.Int("job_count", len(response.Jobs)))

	return connect.NewResponse(response), nil
}

// CancelJob cancels a scheduled job
func (s *Server) CancelJob(ctx context.Context, req *connect.Request[v1.CancelJobRequest]) (*connect.Response[v1.CancelJobResponse], error) {
	msg := req.Msg
	s.logger.Info("Received CancelJob request", zap.String("job_id", msg.JobId))

	if msg.JobId == "" {
		return nil, connect.NewError(connect.CodeInvalidArgument, fmt.Errorf("job_id is required"))
	}

	err := s.jobCreator.CancelJob(ctx, msg.JobId, "")
	if err != nil {
		s.logger.Error("Failed to cancel job",
			zap.String("job_id", msg.JobId),
			zap.Error(err))
		return nil, connect.NewError(connect.CodeInternal, fmt.Errorf("failed to cancel job: %v", err))
	}

	response := &v1.CancelJobResponse{
		Success: true,
		Message: "Job successfully cancelled",
	}

	s.logger.Info("Successfully cancelled job", zap.String("job_id", msg.JobId))

	return connect.NewResponse(response), nil
}

// Start starts the ConnectRPC HTTP server
func (s *Server) Start() error {
	address := fmt.Sprintf("%s:%d", s.config.Server.Host, s.config.Server.Port)

	mux := http.NewServeMux()

	// Create ConnectRPC service handler
	path, handler := v1connect.NewJobSchedulerServiceHandler(s)
	mux.Handle(path, handler)

	s.logger.Info("Starting ConnectRPC HTTP server", zap.String("address", address))

	server := &http.Server{
		Addr:    address,
		Handler: mux,
	}

	if err := server.ListenAndServe(); err != nil {
		return fmt.Errorf("failed to serve ConnectRPC server: %w", err)
	}

	return nil
}

// validateScheduleJobRequest validates a ScheduleJob request
func (s *Server) validateScheduleJobRequest(req *v1.ScheduleJobRequest) error {
	if req.Name == "" {
		return fmt.Errorf("job name is required")
	}

	if req.JobConfig == nil {
		return fmt.Errorf("job configuration is required")
	}

	// Validate namespace if provided
	if req.Namespace != "" {
		// Add namespace validation logic if needed
	}

	return nil
}


// getStatusMessage returns a human-readable message for a job status
func getStatusMessage(status v1.JobStatus) string {
	switch status {
	case v1.JobStatus_JOB_STATUS_PENDING:
		return "Job is pending execution"
	case v1.JobStatus_JOB_STATUS_RUNNING:
		return "Job is currently running"
	case v1.JobStatus_JOB_STATUS_SUCCEEDED:
		return "Job completed successfully"
	case v1.JobStatus_JOB_STATUS_FAILED:
		return "Job failed to complete"
	case v1.JobStatus_JOB_STATUS_CANCELLED:
		return "Job was cancelled"
	default:
		return "Unknown job status"
	}
}
