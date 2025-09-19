#!/bin/bash

# Script to generate Go code from protobuf definitions

set -e

# Check if protoc is installed
if ! command -v protoc &> /dev/null; then
    echo "Error: protoc is not installed. Please install Protocol Buffers compiler."
    exit 1
fi

# Install Go protobuf plugins if not present
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

# Create output directory
mkdir -p proto

# Generate Go code from proto files
protoc \
    --go_out=. \
    --go_opt=paths=source_relative \
    --go-grpc_out=. \
    --go-grpc_opt=paths=source_relative \
    proto/jobscheduler.proto

echo "Successfully generated Go code from protobuf definitions"