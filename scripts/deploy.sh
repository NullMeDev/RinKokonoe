#!/bin/bash

# Build the binary
go build -o bin/rinkokonoe cmd/main.go

# Start or restart the PM2 process
pm2 startOrRestart ecosystem.config.js --env production

echo "Deployment completed successfully!"
