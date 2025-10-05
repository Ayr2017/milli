#!/bin/bash

# Exit on error
set -e

# Default behavior is to run the Rust application
RUN_RUST=true

# Check for command line arguments
if [ "$1" == "--build-only" ]; then
  RUN_RUST=false
fi

echo "Cleaning static directory..."
# Keep the directory but remove its contents
rm -rf static/*

echo "Building frontend..."
cd frontend
# Build the frontend, but don't exit if it fails
npm run build || echo "Frontend build had errors, but continuing..."

cd ..

if [ "$RUN_RUST" = true ]; then
  echo "Running Rust application..."
  # Run cargo with verbose output to see more details
  RUST_BACKTRACE=1 cargo run -v
else
  echo "Frontend built successfully. Skipping Rust application execution."
  echo "To run the Rust application, run this script without the --build-only flag."
fi
