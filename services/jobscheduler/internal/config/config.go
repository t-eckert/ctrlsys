package config

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"

	"github.com/spf13/viper"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

// Config holds all configuration for the jobscheduler service
type Config struct {
	// Server configuration
	Server ServerConfig `mapstructure:"server"`

	// Kubernetes configuration
	Kubernetes KubernetesConfig `mapstructure:"kubernetes"`

	// Job defaults
	JobDefaults JobDefaultsConfig `mapstructure:"job_defaults"`

	// Logging configuration
	Logging LoggingConfig `mapstructure:"logging"`
}

type ServerConfig struct {
	Port        int    `mapstructure:"port"`
	Host        string `mapstructure:"host"`
	MetricsPort int    `mapstructure:"metrics_port"`
}

type KubernetesConfig struct {
	// Namespace where jobs will be created
	DefaultNamespace string `mapstructure:"default_namespace"`

	// Path to kubeconfig file (empty for in-cluster config)
	KubeConfigPath string `mapstructure:"kubeconfig_path"`

	// Whether to use in-cluster configuration
	InCluster bool `mapstructure:"in_cluster"`

	// Job cleanup settings
	JobTTLSeconds int32 `mapstructure:"job_ttl_seconds"`
}

type JobDefaultsConfig struct {
	// Default resource requests and limits
	DefaultCPURequest    string `mapstructure:"default_cpu_request"`
	DefaultMemoryRequest string `mapstructure:"default_memory_request"`
	DefaultCPULimit      string `mapstructure:"default_cpu_limit"`
	DefaultMemoryLimit   string `mapstructure:"default_memory_limit"`

	// Default container registry
	DefaultRegistry string `mapstructure:"default_registry"`

	// Timer job specific defaults
	Timer TimerJobDefaults `mapstructure:"timer"`
}

type TimerJobDefaults struct {
	Image                  string `mapstructure:"image"`
	ControlPlaneEndpoint   string `mapstructure:"control_plane_endpoint"`
	DefaultDurationSeconds int64  `mapstructure:"default_duration_seconds"`
	LogLevel               string `mapstructure:"log_level"`
}

type LoggingConfig struct {
	Level  string `mapstructure:"level"`
	Format string `mapstructure:"format"` // json or text
}

// LoadConfig loads configuration from environment variables and config files
func LoadConfig() (*Config, error) {
	// Set defaults
	viper.SetDefault("server.port", 50054)
	viper.SetDefault("server.host", "0.0.0.0")
	viper.SetDefault("server.metrics_port", 8080)

	viper.SetDefault("kubernetes.default_namespace", "default")
	viper.SetDefault("kubernetes.in_cluster", true)
	viper.SetDefault("kubernetes.job_ttl_seconds", 86400) // 24 hours

	viper.SetDefault("job_defaults.default_cpu_request", "100m")
	viper.SetDefault("job_defaults.default_memory_request", "64Mi")
	viper.SetDefault("job_defaults.default_cpu_limit", "200m")
	viper.SetDefault("job_defaults.default_memory_limit", "128Mi")
	viper.SetDefault("job_defaults.default_registry", "")

	viper.SetDefault("job_defaults.timer.image", "timer-service:latest")
	viper.SetDefault("job_defaults.timer.control_plane_endpoint", "http://control-plane-service:50053")
	viper.SetDefault("job_defaults.timer.default_duration_seconds", 300)
	viper.SetDefault("job_defaults.timer.log_level", "info")

	viper.SetDefault("logging.level", "info")
	viper.SetDefault("logging.format", "json")

	// Environment variable mapping
	viper.SetEnvPrefix("JOBSCHEDULER")
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	viper.AutomaticEnv()

	// Bind specific environment variables
	envMappings := map[string]string{
		"GRPC_PORT":                    "server.port",
		"HOST":                         "server.host",
		"METRICS_PORT":                 "server.metrics_port",
		"K8S_NAMESPACE":                "kubernetes.default_namespace",
		"KUBECONFIG":                   "kubernetes.kubeconfig_path",
		"IN_CLUSTER":                   "kubernetes.in_cluster",
		"JOB_TTL_SECONDS":              "kubernetes.job_ttl_seconds",
		"DEFAULT_CPU_REQUEST":          "job_defaults.default_cpu_request",
		"DEFAULT_MEMORY_REQUEST":       "job_defaults.default_memory_request",
		"DEFAULT_CPU_LIMIT":            "job_defaults.default_cpu_limit",
		"DEFAULT_MEMORY_LIMIT":         "job_defaults.default_memory_limit",
		"DEFAULT_REGISTRY":             "job_defaults.default_registry",
		"TIMER_IMAGE":                  "job_defaults.timer.image",
		"TIMER_CONTROL_PLANE_ENDPOINT": "job_defaults.timer.control_plane_endpoint",
		"TIMER_DEFAULT_DURATION":       "job_defaults.timer.default_duration_seconds",
		"TIMER_LOG_LEVEL":              "job_defaults.timer.log_level",
		"LOG_LEVEL":                    "logging.level",
		"LOG_FORMAT":                   "logging.format",
	}

	for envVar, configKey := range envMappings {
		if value := os.Getenv(envVar); value != "" {
			viper.Set(configKey, value)
		}
	}

	// Handle boolean environment variables
	if inCluster := os.Getenv("IN_CLUSTER"); inCluster != "" {
		if val, err := strconv.ParseBool(inCluster); err == nil {
			viper.Set("kubernetes.in_cluster", val)
		}
	}

	// Try to read config file
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	viper.AddConfigPath(".")
	viper.AddConfigPath("/etc/jobscheduler/")
	viper.AddConfigPath("$HOME/.jobscheduler")

	// Config file is optional
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			return nil, fmt.Errorf("error reading config file: %w", err)
		}
	}

	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		return nil, fmt.Errorf("error unmarshaling config: %w", err)
	}

	return &config, nil
}

// Validate validates the configuration
func (c *Config) Validate() error {
	if c.Server.Port <= 0 || c.Server.Port > 65535 {
		return fmt.Errorf("invalid server port: %d", c.Server.Port)
	}

	if c.Kubernetes.DefaultNamespace == "" {
		return fmt.Errorf("kubernetes default namespace cannot be empty")
	}

	if c.JobDefaults.Timer.DefaultDurationSeconds <= 0 {
		return fmt.Errorf("timer default duration must be positive")
	}

	// Validate log level
	validLogLevels := []string{"debug", "info", "warn", "error", "dpanic", "panic", "fatal"}
	if !slices.Contains(validLogLevels, c.Logging.Level) {
		return fmt.Errorf("invalid log level: %s", c.Logging.Level)
	}

	return nil
}

// SetupLogging configures the logger based on the configuration and returns a zap logger
func (c *Config) SetupLogging() (*zap.Logger, error) {
	// Parse log level
	var level zapcore.Level
	switch c.Logging.Level {
	case "debug":
		level = zapcore.DebugLevel
	case "info":
		level = zapcore.InfoLevel
	case "warn":
		level = zapcore.WarnLevel
	case "error":
		level = zapcore.ErrorLevel
	case "dpanic":
		level = zapcore.DPanicLevel
	case "panic":
		level = zapcore.PanicLevel
	case "fatal":
		level = zapcore.FatalLevel
	default:
		return nil, fmt.Errorf("invalid log level: %s", c.Logging.Level)
	}

	// Configure encoder
	var encoder zapcore.Encoder
	encoderConfig := zap.NewProductionEncoderConfig()
	encoderConfig.TimeKey = "timestamp"
	encoderConfig.EncodeTime = zapcore.ISO8601TimeEncoder

	if c.Logging.Format == "json" {
		encoder = zapcore.NewJSONEncoder(encoderConfig)
	} else {
		encoderConfig.EncodeLevel = zapcore.CapitalColorLevelEncoder
		encoder = zapcore.NewConsoleEncoder(encoderConfig)
	}

	// Create core
	core := zapcore.NewCore(encoder, zapcore.AddSync(os.Stdout), level)

	// Create logger
	logger := zap.New(core, zap.AddCaller(), zap.AddStacktrace(zapcore.ErrorLevel))

	return logger, nil
}
