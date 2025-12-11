#!/bin/bash
set -e

: "${DOCKER_USERNAME:?need DOCKER_USERNAME}"
: "${DOCKER_PASSWORD:?need DOCKER_PASSWORD}"
: "${DOCKER_REGISTRY_HOST:?e.g. git.example.com}"

cd "$(dirname "$0")"

# Make username lowercase. This is required for tags.
DOCKER_USERNAME=$(echo "$DOCKER_USERNAME" | tr '[:upper:]' '[:lower:]')

# Login to container registry
echo "$DOCKER_PASSWORD" | docker login "$DOCKER_REGISTRY_HOST" -u "$DOCKER_USERNAME" --password-stdin

# Extract crate version from top-level Cargo.toml
SOFTWARE_VERSION=$(grep '^version' ../../Cargo.toml | head -n1 | cut -d '"' -f2)
SOFTWARE_IMAGE_NAME=sam-dns-tools-dyndns
IMAGE_NAME="$DOCKER_REGISTRY_HOST/$DOCKER_USERNAME/$SOFTWARE_IMAGE_NAME"

docker buildx build --platform linux/amd64,linux/arm64 --push -t "$IMAGE_NAME:$SOFTWARE_VERSION" .
docker buildx build --platform linux/amd64,linux/arm64 --push -t "$IMAGE_NAME:latest" .
