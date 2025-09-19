package k8s

import (
	"context"
	"fmt"
	"os"
	"path/filepath"

	"go.uber.org/zap"
	batchv1 "k8s.io/api/batch/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/labels"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/rest"
	"k8s.io/client-go/tools/clientcmd"

	pb "github.com/t-eckert/ctrlsys/services/jobscheduler/proto"
)

// Client wraps the Kubernetes client and provides job management operations
type Client struct {
	clientset kubernetes.Interface
	logger    *zap.Logger
}

// NewClient creates a new Kubernetes client
func NewClient(inCluster bool, kubeConfigPath string, logger *zap.Logger) (*Client, error) {
	var config *rest.Config
	var err error

	if inCluster {
		// Use in-cluster configuration
		config, err = rest.InClusterConfig()
		if err != nil {
			return nil, fmt.Errorf("failed to create in-cluster config: %w", err)
		}
		logger.Info("Using in-cluster Kubernetes configuration")
	} else {
		// Use kubeconfig file
		if kubeConfigPath == "" {
			if home := homeDir(); home != "" {
				kubeConfigPath = filepath.Join(home, ".kube", "config")
			}
		}

		config, err = clientcmd.BuildConfigFromFlags("", kubeConfigPath)
		if err != nil {
			return nil, fmt.Errorf("failed to create config from kubeconfig: %w", err)
		}
		logger.Info("Using kubeconfig file", zap.String("kubeconfig", kubeConfigPath))
	}

	// Create the clientset
	clientset, err := kubernetes.NewForConfig(config)
	if err != nil {
		return nil, fmt.Errorf("failed to create Kubernetes clientset: %w", err)
	}

	client := &Client{
		clientset: clientset,
		logger:    logger,
	}

	// Test the connection
	if err := client.testConnection(); err != nil {
		return nil, fmt.Errorf("failed to connect to Kubernetes cluster: %w", err)
	}

	return client, nil
}

// testConnection verifies that we can connect to the Kubernetes cluster
func (c *Client) testConnection() error {
	ctx, cancel := context.WithTimeout(context.Background(), 10)
	defer cancel()

	_, err := c.clientset.CoreV1().Namespaces().List(ctx, metav1.ListOptions{Limit: 1})
	if err != nil {
		return fmt.Errorf("failed to list namespaces: %w", err)
	}

	c.logger.Info("Successfully connected to Kubernetes cluster")
	return nil
}

// CreateJob creates a new Kubernetes Job
func (c *Client) CreateJob(ctx context.Context, job *batchv1.Job) (*batchv1.Job, error) {
	createdJob, err := c.clientset.BatchV1().Jobs(job.Namespace).Create(ctx, job, metav1.CreateOptions{})
	if err != nil {
		c.logger.Error("Failed to create Kubernetes job",
			zap.String("job_name", job.Name),
			zap.String("namespace", job.Namespace),
			zap.Error(err))
		return nil, fmt.Errorf("failed to create job: %w", err)
	}

	c.logger.Info("Successfully created Kubernetes job",
		zap.String("job_name", createdJob.Name),
		zap.String("namespace", createdJob.Namespace),
		zap.String("uid", string(createdJob.UID)))

	return createdJob, nil
}

// GetJob retrieves a Kubernetes Job by name and namespace
func (c *Client) GetJob(ctx context.Context, namespace, name string) (*batchv1.Job, error) {
	job, err := c.clientset.BatchV1().Jobs(namespace).Get(ctx, name, metav1.GetOptions{})
	if err != nil {
		return nil, fmt.Errorf("failed to get job %s/%s: %w", namespace, name, err)
	}

	return job, nil
}

// ListJobs lists Kubernetes Jobs with optional label selector
func (c *Client) ListJobs(ctx context.Context, namespace string, labelSelector map[string]string) (*batchv1.JobList, error) {
	var selector string
	if len(labelSelector) > 0 {
		selector = labels.SelectorFromSet(labelSelector).String()
	}

	jobs, err := c.clientset.BatchV1().Jobs(namespace).List(ctx, metav1.ListOptions{
		LabelSelector: selector,
	})
	if err != nil {
		return nil, fmt.Errorf("failed to list jobs: %w", err)
	}

	return jobs, nil
}

// DeleteJob deletes a Kubernetes Job
func (c *Client) DeleteJob(ctx context.Context, namespace, name string) error {
	deletePolicy := metav1.DeletePropagationForeground
	err := c.clientset.BatchV1().Jobs(namespace).Delete(ctx, name, metav1.DeleteOptions{
		PropagationPolicy: &deletePolicy,
	})
	if err != nil {
		c.logger.Error("Failed to delete Kubernetes job",
			zap.String("job_name", name),
			zap.String("namespace", namespace),
			zap.Error(err))
		return fmt.Errorf("failed to delete job: %w", err)
	}

	c.logger.Info("Successfully deleted Kubernetes job",
		zap.String("job_name", name),
		zap.String("namespace", namespace))

	return nil
}

// GetJobStatus converts Kubernetes Job status to our protobuf JobStatus
func (c *Client) GetJobStatus(job *batchv1.Job) pb.JobStatus {
	// Check job conditions for more specific status
	for _, condition := range job.Status.Conditions {
		switch condition.Type {
		case batchv1.JobComplete:
			if condition.Status == "True" {
				return pb.JobStatus_JOB_STATUS_SUCCEEDED
			}
		case batchv1.JobFailed:
			if condition.Status == "True" {
				return pb.JobStatus_JOB_STATUS_FAILED
			}
		}
	}

	// Check if job is running
	if job.Status.Active > 0 {
		return pb.JobStatus_JOB_STATUS_RUNNING
	}

	// If the job exists but hasn't started yet
	return pb.JobStatus_JOB_STATUS_PENDING
}

// homeDir returns the home directory for the current user
func homeDir() string {
	if h := os.Getenv("HOME"); h != "" {
		return h
	}
	return os.Getenv("USERPROFILE") // Windows
}
