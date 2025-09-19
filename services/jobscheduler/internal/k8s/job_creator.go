package k8s

import (
	"context"
	"fmt"

	"go.uber.org/zap"
	batchv1 "k8s.io/api/batch/v1"

	v1 "github.com/t-eckert/ctrlsys/gen/go/ctrlsys/jobscheduler/v1"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/config"
	"github.com/t-eckert/ctrlsys/services/jobscheduler/internal/jobs"
)

// JobCreator handles the creation and management of Kubernetes Jobs
type JobCreator struct {
	client   *Client
	config   *config.Config
	registry *jobs.Registry
	logger   *zap.Logger
}

// NewJobCreator creates a new JobCreator
func NewJobCreator(client *Client, config *config.Config, registry *jobs.Registry, logger *zap.Logger) *JobCreator {
	return &JobCreator{
		client:   client,
		config:   config,
		registry: registry,
		logger:   logger,
	}
}

// CreateJobFromRequest creates a Kubernetes Job from a ScheduleJobRequest
func (jc *JobCreator) CreateJobFromRequest(ctx context.Context, request *v1.ScheduleJobRequest) (*v1.ScheduleJobResponse, error) {
	// Determine job type from the request
	jobType, err := jc.getJobTypeFromRequest(request)
	if err != nil {
		return nil, fmt.Errorf("failed to determine job type: %w", err)
	}

	// Get the appropriate job handler
	handler, err := jc.registry.GetHandler(jobType)
	if err != nil {
		return nil, fmt.Errorf("failed to get job handler: %w", err)
	}

	// Validate the job configuration
	if err := handler.ValidateConfig(request); err != nil {
		return nil, fmt.Errorf("job configuration validation failed: %w", err)
	}

	// Generate job defaults from config
	defaults := jc.createJobDefaults()

	// Generate the Kubernetes Job manifest
	job, err := handler.GenerateJobManifest(ctx, request, defaults)
	if err != nil {
		return nil, fmt.Errorf("failed to generate job manifest: %w", err)
	}

	// Create the job in Kubernetes
	createdJob, err := jc.client.CreateJob(ctx, job)
	if err != nil {
		return nil, fmt.Errorf("failed to create Kubernetes job: %w", err)
	}

	// Build response
	response := &v1.ScheduleJobResponse{
		JobId:      request.JobId,
		Status:     v1.JobStatus_JOB_STATUS_PENDING,
		Message:    "Job successfully scheduled",
		K8SJobName: createdJob.Name,
	}

	jc.logger.Info("Successfully created job",
		zap.String("job_id", request.JobId),
		zap.String("job_name", request.Name),
		zap.String("job_type", string(jobType)),
		zap.String("k8s_job_name", createdJob.Name),
		zap.String("namespace", createdJob.Namespace))

	return response, nil
}

// GetJobInfo retrieves information about a job
func (jc *JobCreator) GetJobInfo(ctx context.Context, jobID string, namespace string) (*v1.JobInfo, error) {
	// Find the job by label selector
	labelSelector := map[string]string{
		"ctrlsys.io/job-id": jobID,
	}

	if namespace == "" {
		namespace = jc.config.Kubernetes.DefaultNamespace
	}

	jobList, err := jc.client.ListJobs(ctx, namespace, labelSelector)
	if err != nil {
		return nil, fmt.Errorf("failed to list jobs: %w", err)
	}

	if len(jobList.Items) == 0 {
		return nil, fmt.Errorf("job with ID %s not found", jobID)
	}

	if len(jobList.Items) > 1 {
		jc.logger.Warn("Multiple jobs found with same job ID",
			zap.String("job_id", jobID),
			zap.Int("count", len(jobList.Items)))
	}

	job := &jobList.Items[0]
	return jc.convertJobToJobInfo(job)
}

// ListJobs lists jobs with optional filtering
func (jc *JobCreator) ListJobs(ctx context.Context, request *v1.ListJobsRequest) (*v1.ListJobsResponse, error) {
	namespace := request.Namespace
	if namespace == "" {
		namespace = jc.config.Kubernetes.DefaultNamespace
	}

	// Build label selector
	labelSelector := map[string]string{
		"app.kubernetes.io/managed-by": "jobscheduler",
	}

	// Add user-provided label filters
	for k, v := range request.LabelSelector {
		labelSelector[k] = v
	}

	jobList, err := jc.client.ListJobs(ctx, namespace, labelSelector)
	if err != nil {
		return nil, fmt.Errorf("failed to list jobs: %w", err)
	}

	var jobInfos []*v1.JobInfo
	for _, job := range jobList.Items {
		// Apply status filter if provided
		if len(request.StatusFilter) > 0 {
			jobStatus := jc.client.GetJobStatus(&job)
			statusMatch := false
			for _, filter := range request.StatusFilter {
				if jobStatus == filter {
					statusMatch = true
					break
				}
			}
			if !statusMatch {
				continue
			}
		}

		jobInfo, err := jc.convertJobToJobInfo(&job)
		if err != nil {
			jc.logger.Error("Failed to convert job to job info",
				zap.String("job_name", job.Name),
				zap.Error(err))
			continue
		}

		jobInfos = append(jobInfos, jobInfo)
	}

	response := &v1.ListJobsResponse{
		Jobs:       jobInfos,
		TotalCount: int32(len(jobInfos)),
	}

	return response, nil
}

// CancelJob cancels a running job
func (jc *JobCreator) CancelJob(ctx context.Context, jobID string, namespace string) error {
	jobInfo, err := jc.GetJobInfo(ctx, jobID, namespace)
	if err != nil {
		return fmt.Errorf("failed to find job: %w", err)
	}

	if jobInfo.Status == v1.JobStatus_JOB_STATUS_SUCCEEDED || jobInfo.Status == v1.JobStatus_JOB_STATUS_FAILED {
		return fmt.Errorf("cannot cancel job in status: %s", jobInfo.Status.String())
	}

	return jc.client.DeleteJob(ctx, jobInfo.Namespace, jobInfo.K8SJobName)
}

// getJobTypeFromRequest determines the job type from the request configuration
func (jc *JobCreator) getJobTypeFromRequest(request *v1.ScheduleJobRequest) (jobs.JobType, error) {
	switch request.JobConfig.(type) {
	case *v1.ScheduleJobRequest_TimerJob:
		return jobs.JobTypeTimer, nil
	default:
		return "", fmt.Errorf("unknown job configuration type")
	}
}

// createJobDefaults creates job defaults from the configuration
func (jc *JobCreator) createJobDefaults() *jobs.JobDefaults {
	ttlSeconds := jc.config.Kubernetes.JobTTLSeconds
	backoffLimit := int32(3)
	completions := int32(1)
	parallelism := int32(1)
	completionMode := batchv1.NonIndexedCompletion

	return &jobs.JobDefaults{
		Namespace:      jc.config.Kubernetes.DefaultNamespace,
		CPURequest:     jc.config.JobDefaults.DefaultCPURequest,
		MemoryRequest:  jc.config.JobDefaults.DefaultMemoryRequest,
		CPULimit:       jc.config.JobDefaults.DefaultCPULimit,
		MemoryLimit:    jc.config.JobDefaults.DefaultMemoryLimit,
		Registry:       jc.config.JobDefaults.DefaultRegistry,
		TTLSeconds:     &ttlSeconds,
		RestartPolicy:  "Never",
		BackoffLimit:   &backoffLimit,
		CompletionMode: &completionMode,
		Parallelism:    &parallelism,
		Completions:    &completions,
	}
}

// convertJobToJobInfo converts a Kubernetes Job to our JobInfo protobuf message
func (jc *JobCreator) convertJobToJobInfo(job *batchv1.Job) (*v1.JobInfo, error) {
	// Extract job ID and other metadata from labels/annotations
	jobID := job.Labels["ctrlsys.io/job-id"]
	jobName := job.Annotations["ctrlsys.io/job-name"]
	createdBy := job.Annotations["ctrlsys.io/created-by"]
	jobType := jobs.JobType(job.Labels["ctrlsys.io/job-type"])

	jobInfo := &v1.JobInfo{
		JobId:       jobID,
		Name:        jobName,
		K8SJobName:  job.Name,
		Namespace:   job.Namespace,
		Status:      jc.client.GetJobStatus(job),
		CreatedAt:   job.CreationTimestamp.Unix(),
		CreatedBy:   createdBy,
		Labels:      make(map[string]string),
		Annotations: make(map[string]string),
	}

	// Copy user labels (exclude system labels)
	for k, v := range job.Labels {
		if !isSystemLabel(k) {
			jobInfo.Labels[k] = v
		}
	}

	// Copy user annotations (exclude system annotations)
	for k, v := range job.Annotations {
		if !isSystemAnnotation(k) {
			jobInfo.Annotations[k] = v
		}
	}

	// Set start and completion times
	if job.Status.StartTime != nil {
		jobInfo.StartedAt = job.Status.StartTime.Unix()
	}

	if job.Status.CompletionTime != nil {
		jobInfo.CompletedAt = job.Status.CompletionTime.Unix()
	}

	// Extract job-specific details
	if jc.registry.IsRegistered(jobType) {
		handler, err := jc.registry.GetHandler(jobType)
		if err == nil {
			if details, err := handler.ExtractJobDetails(job); err == nil {
				switch jobType {
				case jobs.JobTypeTimer:
					if timerDetails, ok := details.(*v1.TimerJobDetails); ok {
						jobInfo.JobDetails = &v1.JobInfo_TimerDetails{
							TimerDetails: timerDetails,
						}
					}
				}
			}
		}
	}

	return jobInfo, nil
}

// isSystemLabel checks if a label is a system-managed label
func isSystemLabel(key string) bool {
	systemPrefixes := []string{
		"app.kubernetes.io/",
		"ctrlsys.io/",
	}

	for _, prefix := range systemPrefixes {
		if len(key) >= len(prefix) && key[:len(prefix)] == prefix {
			return true
		}
	}

	return false
}

// isSystemAnnotation checks if an annotation is a system-managed annotation
func isSystemAnnotation(key string) bool {
	return isSystemLabel(key) // Same logic for now
}
