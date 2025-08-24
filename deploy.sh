#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NAMESPACE="triage"
BACKEND_IMAGE="triage-backend:latest"
FRONTEND_IMAGE="triage-frontend:latest"

echo -e "${BLUE}🚀 Triage People Intelligence Platform Deployment${NC}"
echo "=================================================="

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

if ! command_exists kubectl; then
    echo -e "${RED}❌ kubectl is not installed${NC}"
    exit 1
fi

# Check if kubectl can connect to cluster
if ! kubectl cluster-info >/dev/null 2>&1; then
    echo -e "${RED}❌ Cannot connect to Kubernetes cluster${NC}"
    echo "Make sure your kubectl is configured and cluster is running"
    exit 1
fi

echo -e "${GREEN}✅ Prerequisites check passed${NC}"

# Deploy to Kubernetes
echo -e "${YELLOW}☸️  Deploying to Kubernetes...${NC}"

# Apply namespace and configurations first
echo "Creating namespace and configurations..."
kubectl apply -f k8s/namespace.yaml

# Wait a moment for namespace to be ready
sleep 2

# Apply persistent volumes
echo "Creating persistent volumes..."
kubectl apply -f k8s/persistent-volumes.yaml

# Deploy databases
echo "Deploying databases..."
kubectl apply -f k8s/postgres.yaml
kubectl apply -f k8s/opensearch.yaml
kubectl apply -f k8s/neo4j.yaml

# Wait for databases to be ready
echo "Waiting for databases to be ready..."
kubectl wait --for=condition=available --timeout=300s deployment/postgres -n $NAMESPACE
kubectl wait --for=condition=available --timeout=300s deployment/opensearch -n $NAMESPACE
kubectl wait --for=condition=available --timeout=300s deployment/neo4j -n $NAMESPACE

# Deploy applications
echo "Deploying applications..."
kubectl apply -f k8s/backend.yaml
kubectl apply -f k8s/frontend.yaml

# Wait for applications to be ready
echo "Waiting for applications to be ready..."
kubectl wait --for=condition=available --timeout=300s deployment/triage-backend -n $NAMESPACE
kubectl wait --for=condition=available --timeout=300s deployment/triage-frontend -n $NAMESPACE

echo -e "${GREEN}✅ Deployment completed successfully!${NC}"

# Display status
echo -e "${BLUE}📊 Deployment Status${NC}"
echo "===================="
kubectl get pods -n $NAMESPACE
echo

echo -e "${BLUE}🔗 Service Endpoints${NC}"
echo "==================="
kubectl get services -n $NAMESPACE
echo

# Instructions for access
echo -e "${YELLOW}🌐 Access Instructions${NC}"
echo "======================"
echo "1. Frontend: http://localhost:3000"
echo "   kubectl port-forward -n $NAMESPACE service/triage-frontend-service 3000:3000"
echo
echo "2. Backend API: http://localhost:3001"
echo "   kubectl port-forward -n $NAMESPACE service/triage-backend-service 3001:3001"
echo
echo "3. Neo4j Browser: http://localhost:7474"
echo "   kubectl port-forward -n $NAMESPACE service/neo4j-service 7474:7474"
echo
echo "4. OpenSearch Dashboards: http://localhost:5601"
echo "   kubectl port-forward -n $NAMESPACE service/opensearch-dashboards-service 5601:5601"
echo

echo -e "${GREEN}🎉 Triage Platform is now running!${NC}"
echo
echo -e "${YELLOW}⚠️  Remember to:${NC}"
echo "1. Update OAuth credentials in k8s/namespace.yaml"
echo "2. Configure your domain in k8s/frontend.yaml (ingress)"
echo "3. Set up SSL certificates for production"
echo "4. Configure backup strategies for your databases"