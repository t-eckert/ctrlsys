package server

import (
	"context"
	"fmt"
	"net"
	"time"

	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/reflection"
	"google.golang.org/grpc/status"

	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/config"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/jobs"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/k8s"
	pb "github.com/t-eckert/ctrlsys/services/jobscheduler/proto"
)

// Server implements the JobScheduler gRPC service
type Server struct {
	pb.UnimplementedJobSchedulerServer

	config     *config.Config
	jobCreator *k8s.JobCreator
	registry   *jobs.Registry
	logger     *zap.Logger
}

// NewServer creates a new gRPC server instance
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
func (s *Server) ScheduleJob(ctx context.Context, req *pb.ScheduleJobRequest) (*pb.ScheduleJobResponse, error) {
	s.logger.Info("Received ScheduleJob request",
		zap.String("job_id", req.JobId),
		zap.String("job_name", req.Name))

	// Validate request
	if err := s.validateScheduleJobRequest(req); err != nil {
		s.logger.Error("Invalid ScheduleJob request",
			zap.String("job_id", req.JobId),
			zap.Error(err))
		return nil, status.Errorf(codes.InvalidArgument, "invalid request: %v", err)
	}

	// Generate job ID if not provided
	if req.JobId == "" {
		req.JobId = generateJobID()
	}

	// Create the job
	response, err := s.jobCreator.CreateJobFromRequest(ctx, req)
	if err != nil {
		s.logger.Error("Failed to create job",
			zap.String("job_id", req.JobId),
			zap.Error(err))
		return nil, status.Errorf(codes.Internal, "failed to create job: %v", err)
	}

	s.logger.Info("Successfully scheduled job",
		zap.String("job_id", response.JobId),
		zap.String("k8s_job_name", response.K8SJobName))

	return response, nil
}

// GetJobStatus retrieves the status of a scheduled job
func (s *Server) GetJobStatus(ctx context.Context, req *pb.GetJobStatusRequest) (*pb.GetJobStatusResponse, error) {
	s.logger.Debug("Received GetJobStatus request", zap.String("job_id", req.JobId))

	if req.JobId == "" {
		return nil, status.Error(codes.InvalidArgument, "job_id is required")
	}

	jobInfo, err := s.jobCreator.GetJobInfo(ctx, req.JobId, "")
	if err != nil {
		s.logger.Error("Failed to get job status",
			zap.String("job_id", req.JobId),
			zap.Error(err))
		return nil, status.Errorf(codes.NotFound, "job not found: %v", err)
	}

	response := &pb.GetJobStatusResponse{
		JobId:   req.JobId,
		JobInfo: jobInfo,
		Status:  jobInfo.Status,
		Message: getStatusMessage(jobInfo.Status),
	}

	return response, nil
}

// ListJobs lists jobs with optional filtering
func (s *Server) ListJobs(ctx context.Context, req *pb.ListJobsRequest) (*pb.ListJobsResponse, error) {
	s.logger.Debug("Received ListJobs request",
		zap.String("namespace", req.Namespace),
		zap.Any("label_selector", req.LabelSelector),
		zap.Any("status_filter", req.StatusFilter))

	response, err := s.jobCreator.ListJobs(ctx, req)
	if err != nil {
		s.logger.Error("Failed to list jobs", zap.Error(err))
		return nil, status.Errorf(codes.Internal, "failed to list jobs: %v", err)
	}

	s.logger.Debug("Successfully listed jobs", zap.Int("job_count", len(response.Jobs)))

	return response, nil
}

// CancelJob cancels a scheduled job
func (s *Server) CancelJob(ctx context.Context, req *pb.CancelJobRequest) (*pb.CancelJobResponse, error) {
	s.logger.Info("Received CancelJob request", zap.String("job_id", req.JobId))

	if req.JobId == "" {
		return nil, status.Error(codes.InvalidArgument, "job_id is required")
	}

	err := s.jobCreator.CancelJob(ctx, req.JobId, "")
	if err != nil {
		s.logger.Error("Failed to cancel job",
			zap.String("job_id", req.JobId),
			zap.Error(err))
		return nil, status.Errorf(codes.Internal, "failed to cancel job: %v", err)
	}

	response := &pb.CancelJobResponse{
		Success: true,
		Message: "Job successfully cancelled",
	}

	s.logger.Info("Successfully cancelled job", zap.String("job_id", req.JobId))

	return response, nil
}

// Start starts the gRPC server
func (s *Server) Start() error {
	address := fmt.Sprintf("%s:%d", s.config.Server.Host, s.config.Server.Port)
	listener, err := net.Listen("tcp", address)
	if err != nil {
		return fmt.Errorf("failed to listen on %s: %w", address, err)
	}

	grpcServer := grpc.NewServer(
		grpc.UnaryInterceptor(s.loggingInterceptor),
	)

	pb.RegisterJobSchedulerServer(grpcServer, s)

	// Enable reflection for debugging
	reflection.Register(grpcServer)

	s.logger.Info("Starting gRPC server", zap.String("address", address))

	if err := grpcServer.Serve(listener); err != nil {
		return fmt.Errorf("failed to serve gRPC server: %w", err)
	}

	return nil
}

// validateScheduleJobRequest validates a ScheduleJob request
func (s *Server) validateScheduleJobRequest(req *pb.ScheduleJobRequest) error {
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

// loggingInterceptor logs gRPC requests and responses
func (s *Server) loggingInterceptor(
	ctx context.Context,
	req interface{},
	info *grpc.UnaryServerInfo,
	handler grpc.UnaryHandler,
) (interface{}, error) {
	start := time.Now()

	resp, err := handler(ctx, req)

	duration := time.Since(start)

	if err != nil {
		s.logger.Error("gRPC request completed",
			zap.String("method", info.FullMethod),
			zap.Duration("duration", duration),
			zap.Error(err))
	} else {
		s.logger.Info("gRPC request completed",
			zap.String("method", info.FullMethod),
			zap.Duration("duration", duration))
	}

	return resp, err
}

// getStatusMessage returns a human-readable message for a job status
func getStatusMessage(status pb.JobStatus) string {
	switch status {
	case pb.JobStatus_JOB_STATUS_PENDING:
		return "Job is pending execution"
	case pb.JobStatus_JOB_STATUS_RUNNING:
		return "Job is currently running"
	case pb.JobStatus_JOB_STATUS_SUCCEEDED:
		return "Job completed successfully"
	case pb.JobStatus_JOB_STATUS_FAILED:
		return "Job failed to complete"
	case pb.JobStatus_JOB_STATUS_CANCELLED:
		return "Job was cancelled"
	default:
		return "Unknown job status"
	}
}
