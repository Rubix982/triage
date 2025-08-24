#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BACKEND_IMAGE="triage-backend:latest"
FRONTEND_IMAGE="triage-frontend:latest"

echo -e "${BLUE}🔒 Building Triage Platform with Chainguard Images${NC}"
echo "======================================================"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${YELLOW}📋 Checking prerequisites...${NC}"

if ! command_exists docker; then
    echo -e "${RED}❌ Docker is not installed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Prerequisites check passed${NC}"

# Build backend image
echo -e "${YELLOW}🏗️  Building backend with Chainguard Rust images...${NC}"
echo "Using cgr.dev/chainguard/rust:latest-dev for build stage"
echo "Using cgr.dev/chainguard/glibc-dynamic:latest for runtime"

cd server
docker build -t $BACKEND_IMAGE . || {
    echo -e "${RED}❌ Backend build failed${NC}"
    exit 1
}
cd ..

echo -e "${GREEN}✅ Backend image built: $BACKEND_IMAGE${NC}"

# Build frontend image  
echo -e "${YELLOW}🏗️  Building frontend with Chainguard Node/Nginx images...${NC}"
echo "Using cgr.dev/chainguard/node:latest-dev for build stage"
echo "Using cgr.dev/chainguard/nginx:latest for runtime"

cd client
docker build -t $FRONTEND_IMAGE . || {
    echo -e "${RED}❌ Frontend build failed${NC}"
    exit 1
}
cd ..

echo -e "${GREEN}✅ Frontend image built: $FRONTEND_IMAGE${NC}"

# Display image information
echo -e "${BLUE}📊 Image Information${NC}"
echo "===================="
echo "Backend Image:"
docker images $BACKEND_IMAGE --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}\t{{.CreatedSince}}"
echo
echo "Frontend Image:"
docker images $FRONTEND_IMAGE --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}\t{{.CreatedSince}}"
echo

# Security scan information
echo -e "${GREEN}🎉 Images built successfully!${NC}"
echo
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Deploy to Kubernetes: ./deploy.sh"
echo "2. Or run locally:"
echo "   docker run -p 3001:3001 $BACKEND_IMAGE"
echo "   docker run -p 8080:8080 $FRONTEND_IMAGE"
echo
echo -e "${BLUE}💡 Pro tip:${NC} Run 'docker scout quickview $BACKEND_IMAGE' to see security details"