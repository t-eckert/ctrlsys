package jobs

import (
	"context"
	"fmt"
	"strconv"

	"go.uber.org/zap"
	batchv1 "k8s.io/api/batch/v1"
	corev1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/api/resource"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"

	v1 "github.com/t-eckert/ctrlsys/gen/go/ctrlsys/jobscheduler/v1"
)

// TimerJobHandler implements JobHandler for Timer jobs
type TimerJobHandler struct {
	logger *zap.Logger
}

// NewTimerJobHandler creates a new timer job handler
func NewTimerJobHandler(logger *zap.Logger) *TimerJobHandler {
	return &TimerJobHandler{
		logger: logger,
	}
}

// GetJobType returns the job type this handler manages
func (h *TimerJobHandler) GetJobType() JobType {
	return JobTypeTimer
}

// ValidateConfig validates the timer job configuration
func (h *TimerJobHandler) ValidateConfig(request *v1.ScheduleJobRequest) error {
	timerConfig := request.GetTimerJob()
	if timerConfig == nil {
		return fmt.Errorf("timer job configuration is required")
	}

	if timerConfig.DurationSeconds <= 0 {
		return fmt.Errorf("timer duration must be positive, got: %d", timerConfig.DurationSeconds)
	}

	if timerConfig.DurationSeconds > 86400 { // 24 hours
		return fmt.Errorf("timer duration cannot exceed 24 hours, got: %d seconds", timerConfig.DurationSeconds)
	}

	if timerConfig.TimerName == "" {
		return fmt.Errorf("timer name is required")
	}

	if timerConfig.ControlPlaneEndpoint == "" {
		return fmt.Errorf("control plane endpoint is required")
	}

	// Validate log level if provided
	if timerConfig.LogLevel != "" {
		validLevels := []string{"trace", "debug", "info", "warn", "error"}
		valid := false
		for _, level := range validLevels {
			if timerConfig.LogLevel == level {
				valid = true
				break
			}
		}
		if !valid {
			return fmt.Errorf("invalid log level: %s", timerConfig.LogLevel)
		}
	}

	return nil
}

// GenerateJobManifest creates a Kubernetes Job manifest for a timer job
func (h *TimerJobHandler) GenerateJobManifest(ctx context.Context, request *v1.ScheduleJobRequest, defaults *JobDefaults) (*batchv1.Job, error) {
	timerConfig := request.GetTimerJob()
	if timerConfig == nil {
		return nil, fmt.Errorf("timer job configuration is required")
	}

	// Create job metadata
	metadata := &JobMetadata{
		JobID:       request.JobId,
		Name:        request.Name,
		Namespace:   request.Namespace,
		Labels:      request.Labels,
		Annotations: request.Annotations,
		CreatedBy:   request.CreatedBy,
	}

	if metadata.Namespace == "" {
		metadata.Namespace = defaults.Namespace
	}

	// Create base job spec
	job := CreateBaseJobSpec(JobTypeTimer, metadata, defaults)

	// Determine image to use
	image := timerConfig.Image
	if image == "" {
		image = h.GetDefaultImage()
		if defaults.Registry != "" && image != "" {
			image = defaults.Registry + "/" + image
		}
	}

	// Build environment variables
	env := []corev1.EnvVar{
		{
			Name:  "TIMER_DURATION_SECONDS",
			Value: strconv.FormatInt(timerConfig.DurationSeconds, 10),
		},
		{
			Name:  "TIMER_NAME",
			Value: timerConfig.TimerName,
		},
		{
			Name:  "CONTROL_PLANE_ENDPOINT",
			Value: timerConfig.ControlPlaneEndpoint,
		},
		{
			Name:  "TIMER_ID",
			Value: request.JobId,
		},
		{
			Name:  "GRPC_PORT",
			Value: "50051",
		},
	}

	// Add log level if specified
	logLevel := timerConfig.LogLevel
	if logLevel == "" {
		logLevel = "info"
	}
	env = append(env, corev1.EnvVar{
		Name:  "RUST_LOG",
		Value: logLevel,
	})

	// Add any additional environment variables from config
	for key, value := range timerConfig.Env {
		env = append(env, corev1.EnvVar{
			Name:  key,
			Value: value,
		})
	}

	// Build resource requirements
	resources := corev1.ResourceRequirements{}

	// Set requests
	if request.Resources != nil && request.Resources.Requests != nil {
		resources.Requests = corev1.ResourceList{}
		if request.Resources.Requests.Cpu != "" {
			resources.Requests[corev1.ResourceCPU] = resource.MustParse(request.Resources.Requests.Cpu)
		} else {
			resources.Requests[corev1.ResourceCPU] = resource.MustParse(defaults.CPURequest)
		}
		if request.Resources.Requests.Memory != "" {
			resources.Requests[corev1.ResourceMemory] = resource.MustParse(request.Resources.Requests.Memory)
		} else {
			resources.Requests[corev1.ResourceMemory] = resource.MustParse(defaults.MemoryRequest)
		}
	} else {
		resources.Requests = corev1.ResourceList{
			corev1.ResourceCPU:    resource.MustParse(defaults.CPURequest),
			corev1.ResourceMemory: resource.MustParse(defaults.MemoryRequest),
		}
	}

	// Set limits
	if request.Resources != nil && request.Resources.Limits != nil {
		resources.Limits = corev1.ResourceList{}
		if request.Resources.Limits.Cpu != "" {
			resources.Limits[corev1.ResourceCPU] = resource.MustParse(request.Resources.Limits.Cpu)
		} else {
			resources.Limits[corev1.ResourceCPU] = resource.MustParse(defaults.CPULimit)
		}
		if request.Resources.Limits.Memory != "" {
			resources.Limits[corev1.ResourceMemory] = resource.MustParse(request.Resources.Limits.Memory)
		} else {
			resources.Limits[corev1.ResourceMemory] = resource.MustParse(defaults.MemoryLimit)
		}
	} else {
		resources.Limits = corev1.ResourceList{
			corev1.ResourceCPU:    resource.MustParse(defaults.CPULimit),
			corev1.ResourceMemory: resource.MustParse(defaults.MemoryLimit),
		}
	}

	// Create container spec
	container := corev1.Container{
		Name:      "timer",
		Image:     image,
		Env:       env,
		Resources: resources,
		Ports: []corev1.ContainerPort{
			{
				Name:          "grpc",
				ContainerPort: 50051,
				Protocol:      corev1.ProtocolTCP,
			},
		},
		LivenessProbe: &corev1.Probe{
			ProbeHandler: corev1.ProbeHandler{
				Exec: &corev1.ExecAction{
					Command: []string{"timer-service", "health"},
				},
			},
			InitialDelaySeconds: 10,
			PeriodSeconds:       30,
			TimeoutSeconds:      5,
			FailureThreshold:    3,
		},
		ReadinessProbe: &corev1.Probe{
			ProbeHandler: corev1.ProbeHandler{
				Exec: &corev1.ExecAction{
					Command: []string{"timer-service", "health"},
				},
			},
			InitialDelaySeconds: 5,
			PeriodSeconds:       10,
			TimeoutSeconds:      5,
			FailureThreshold:    3,
		},
		ImagePullPolicy: corev1.PullIfNotPresent,
	}

	// Create pod template
	job.Spec.Template = corev1.PodTemplateSpec{
		ObjectMeta: metav1.ObjectMeta{
			Labels: GenerateCommonLabels(JobTypeTimer, metadata),
		},
		Spec: corev1.PodSpec{
			RestartPolicy: corev1.RestartPolicyNever,
			Containers:    []corev1.Container{container},
		},
	}

	h.logger.Debug("Generated timer job manifest",
		zap.String("job_id", request.JobId),
		zap.String("job_name", request.Name),
		zap.String("timer_name", timerConfig.TimerName),
		zap.Int64("duration_seconds", timerConfig.DurationSeconds),
		zap.String("image", image))

	return job, nil
}

// ExtractJobDetails extracts timer-specific details from a Kubernetes Job
func (h *TimerJobHandler) ExtractJobDetails(job *batchv1.Job) (any, error) {
	if len(job.Spec.Template.Spec.Containers) == 0 {
		return nil, fmt.Errorf("job has no containers")
	}

	container := job.Spec.Template.Spec.Containers[0]
	details := &v1.TimerJobDetails{}

	// Extract details from environment variables
	for _, env := range container.Env {
		switch env.Name {
		case "TIMER_DURATION_SECONDS":
			if duration, err := strconv.ParseInt(env.Value, 10, 64); err == nil {
				details.DurationSeconds = duration
			}
		case "TIMER_NAME":
			details.TimerName = env.Value
		case "CONTROL_PLANE_ENDPOINT":
			details.ControlPlaneEndpoint = env.Value
		}
	}

	return details, nil
}

// GetDefaultImage returns the default container image for timer jobs
func (h *TimerJobHandler) GetDefaultImage() string {
	return "timer-service:latest"
}
