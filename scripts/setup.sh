#!/bin/bash

# Create necessary directories
mkdir -p bin data logs

# Build the binary
go build -o bin/rinkokonoe cmd/main.go

# Ensure the script is executable
chmod +x bin/rinkokonoe

echo "Setup completed successfully!"
