package version

import (
	"fmt"
	"runtime/debug"
	"strings"
)

// Version information that can be set at build time
var (
	// Version is the semantic version of the service
	Version = "dev"
	// GitCommit is the git commit hash
	GitCommit = "unknown"
	// BuildDate is the date when the binary was built
	BuildDate = "unknown"
	// GoVersion is the Go version used to build the binary
	GoVersion = "unknown"
)

// BuildInfo contains version and build information
type BuildInfo struct {
	Version   string `json:"version"`
	GitCommit string `json:"git_commit"`
	BuildDate string `json:"build_date"`
	GoVersion string `json:"go_version"`
}

// GetBuildInfo returns the complete build information
func GetBuildInfo() BuildInfo {
	buildInfo := BuildInfo{
		Version:   Version,
		GitCommit: GitCommit,
		BuildDate: BuildDate,
		GoVersion: GoVersion,
	}

	// Try to get additional information from runtime/debug
	if info, ok := debug.ReadBuildInfo(); ok {
		// Get Go version from build info
		if buildInfo.GoVersion == "unknown" {
			buildInfo.GoVersion = info.GoVersion
		}

		// Try to extract git information from build settings
		for _, setting := range info.Settings {
			switch setting.Key {
			case "vcs.revision":
				if buildInfo.GitCommit == "unknown" {
					buildInfo.GitCommit = setting.Value
					if len(buildInfo.GitCommit) > 7 {
						buildInfo.GitCommit = buildInfo.GitCommit[:7] // Short commit hash
					}
				}
			case "vcs.time":
				if buildInfo.BuildDate == "unknown" {
					buildInfo.BuildDate = setting.Value
				}
			case "vcs.modified":
				if setting.Value == "true" && !strings.HasSuffix(buildInfo.GitCommit, "-dirty") {
					buildInfo.GitCommit += "-dirty"
				}
			}
		}

		// If version is still "dev", try to use module version
		if buildInfo.Version == "dev" && info.Main.Version != "(devel)" && info.Main.Version != "" {
			buildInfo.Version = info.Main.Version
		}
	}

	return buildInfo
}

// GetVersion returns just the version string
func GetVersion() string {
	return GetBuildInfo().Version
}

// GetFullVersionString returns a formatted version string with all information
func GetFullVersionString() string {
	info := GetBuildInfo()

	versionStr := fmt.Sprintf("JobScheduler Service %s", info.Version)

	if info.GitCommit != "unknown" {
		versionStr += fmt.Sprintf(" (commit: %s)", info.GitCommit)
	}

	if info.BuildDate != "unknown" {
		versionStr += fmt.Sprintf(" (built: %s)", info.BuildDate)
	}

	versionStr += fmt.Sprintf(" (go: %s)", info.GoVersion)

	return versionStr
}

// GetShortVersionString returns a simple version string
func GetShortVersionString() string {
	return fmt.Sprintf("JobScheduler Service v%s", GetVersion())
}