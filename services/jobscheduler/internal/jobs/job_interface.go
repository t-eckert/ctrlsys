package jobs

import (
	"context"

	batchv1 "k8s.io/api/batch/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"

	pb "github.com/t-eckert/ctrlsys/services/jobscheduler/proto"
)

// JobType represents the type of job
type JobType string

const (
	JobTypeTimer           JobType = "timer"
	JobTypeWeatherReporter JobType = "weather_reporter"
	JobTypeHealthCheck     JobType = "health_check"
)

// JobHandler defines the interface that all job types must implement
type JobHandler interface {
	// GetJobType returns the type of job this handler manages
	GetJobType() JobType

	// ValidateConfig validates the job-specific configuration
	ValidateConfig(request *pb.ScheduleJobRequest) error

	// GenerateJobManifest creates a Kubernetes Job manifest for this job type
	GenerateJobManifest(ctx context.Context, request *pb.ScheduleJobRequest, defaults *JobDefaults) (*batchv1.Job, error)

	// ExtractJobDetails extracts job-specific details from a Kubernetes Job
	ExtractJobDetails(job *batchv1.Job) (interface{}, error)

	// GetDefaultImage returns the default container image for this job type
	GetDefaultImage() string
}

// JobDefaults contains default values for job creation
type JobDefaults struct {
	Namespace      string
	CPURequest     string
	MemoryRequest  string
	CPULimit       string
	MemoryLimit    string
	Registry       string
	TTLSeconds     *int32
	RestartPolicy  string
	BackoffLimit   *int32
	CompletionMode *batchv1.CompletionMode
	Parallelism    *int32
	Completions    *int32
}

// JobMetadata contains common metadata for all jobs
type JobMetadata struct {
	JobID       string
	Name        string
	Namespace   string
	Labels      map[string]string
	Annotations map[string]string
	CreatedBy   string
}

// GenerateCommonLabels creates standard labels for all jobs
func GenerateCommonLabels(jobType JobType, metadata *JobMetadata) map[string]string {
	labels := map[string]string{
		"app.kubernetes.io/name":       "ctrlsys-job",
		"app.kubernetes.io/component":  string(jobType),
		"app.kubernetes.io/managed-by": "jobscheduler",
		"ctrlsys.io/job-type":          string(jobType),
		"ctrlsys.io/job-id":            metadata.JobID,
	}

	// Add user-provided labels
	for k, v := range metadata.Labels {
		// Prevent overriding system labels
		if _, exists := labels[k]; !exists {
			labels[k] = v
		}
	}

	return labels
}

// GenerateCommonAnnotations creates standard annotations for all jobs
func GenerateCommonAnnotations(metadata *JobMetadata) map[string]string {
	annotations := map[string]string{
		"ctrlsys.io/job-id":   metadata.JobID,
		"ctrlsys.io/job-name": metadata.Name,
	}

	if metadata.CreatedBy != "" {
		annotations["ctrlsys.io/created-by"] = metadata.CreatedBy
	}

	// Add user-provided annotations
	for k, v := range metadata.Annotations {
		annotations[k] = v
	}

	return annotations
}

// GenerateJobName creates a Kubernetes-compatible job name
func GenerateJobName(jobType JobType, jobID string) string {
	// Kubernetes job names must be DNS-1123 compliant
	// Replace any invalid characters and ensure length limits
	name := string(jobType) + "-" + jobID
	if len(name) > 63 {
		// Truncate to fit within Kubernetes name limits
		name = name[:63]
	}
	return name
}

// CreateBaseJobSpec creates a base Kubernetes Job spec with common fields
func CreateBaseJobSpec(jobType JobType, metadata *JobMetadata, defaults *JobDefaults) *batchv1.Job {
	jobName := GenerateJobName(jobType, metadata.JobID)

	job := &batchv1.Job{
		TypeMeta: metav1.TypeMeta{
			APIVersion: "batch/v1",
			Kind:       "Job",
		},
		ObjectMeta: metav1.ObjectMeta{
			Name:        jobName,
			Namespace:   metadata.Namespace,
			Labels:      GenerateCommonLabels(jobType, metadata),
			Annotations: GenerateCommonAnnotations(metadata),
		},
		Spec: batchv1.JobSpec{
			TTLSecondsAfterFinished: defaults.TTLSeconds,
			BackoffLimit:            defaults.BackoffLimit,
			Completions:             defaults.Completions,
			Parallelism:             defaults.Parallelism,
			CompletionMode:          defaults.CompletionMode,
		},
	}

	return job
}
