# Build stage
FROM golang:1.21-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git protobuf-dev protoc

# Set working directory
WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Install protobuf generation tools
RUN go install google.golang.org/protobuf/cmd/protoc-gen-go@latest && \
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

# Copy source code
COPY . .

# Generate protobuf code
RUN chmod +x scripts/generate-proto.sh && ./scripts/generate-proto.sh

# Set build args for version information
ARG VERSION=dev
ARG GIT_COMMIT=unknown
ARG BUILD_DATE=unknown
ARG GO_VERSION=unknown

# Build the application with version information
RUN CGO_ENABLED=0 GOOS=linux go build \
    -ldflags "-X github.com/t-eckert/ctrlsys/services/jobscheduler/internal/version.Version=${VERSION} \
              -X github.com/t-eckert/ctrlsys/services/jobscheduler/internal/version.GitCommit=${GIT_COMMIT} \
              -X github.com/t-eckert/ctrlsys/services/jobscheduler/internal/version.BuildDate=${BUILD_DATE} \
              -X github.com/t-eckert/ctrlsys/services/jobscheduler/internal/version.GoVersion=${GO_VERSION} \
              -s -w" \
    -a -installsuffix cgo -o jobscheduler .

# Runtime stage
FROM alpine:3.18

# Install ca-certificates for TLS
RUN apk --no-cache add ca-certificates tzdata

# Create non-root user
RUN adduser -D -s /bin/sh jobscheduler

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/jobscheduler .

# Change ownership of the app directory
RUN chown -R jobscheduler:jobscheduler /app

# Switch to non-root user
USER jobscheduler

# Expose gRPC port
EXPOSE 50054

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD ./jobscheduler health || exit 1

# Default command
CMD ["./jobscheduler"]