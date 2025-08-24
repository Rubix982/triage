# Triage Platform Kubernetes Deployment Guide

## Architecture Overview

The Triage People Intelligence Platform consists of:

```
┌─ Frontend (React + Nginx) ─┐  ┌─ Backend (Rust) ──────┐
│ • Port 8080 (internal)     │  │ • Port 3001           │
│ • Port 3000 (service)      │  │ • REST API            │
│ • User Interface           │  │ • Authentication      │
│ • OAuth redirects          │  │ • Chainguard secure   │
└────────────────────────────┘  └───────────────────────┘
                │                           │
                └───────────┬───────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
┌─ PostgreSQL ──┐ ┌─ OpenSearch ──┐ ┌─ Neo4j ────┐
│ • Port 5432   │ │ • Port 9200   │ │ • Port 7474 │
│ • Auth, Users │ │ • Full-text   │ │ • Graph DB  │
│ • Analytics   │ │   Search      │ │ • Relations │
└───────────────┘ └───────────────┘ └─────────────┘
```

## Security-First Architecture

This deployment uses **Chainguard Images** for enhanced security:

- ✅ **Distroless runtime** - minimal attack surface
- ✅ **Non-root by default** - principle of least privilege  
- ✅ **Daily security updates** - always patched
- ✅ **SBOM included** - full supply chain transparency
- ✅ **Zero known CVEs** - clean security posture

## Prerequisites

- **Docker** (for building images)
- **kubectl** (configured with cluster access)
- **Kubernetes cluster** (local or cloud)
- **Ingress Controller** (nginx recommended)
- **Persistent Storage** (for database data)

## Quick Start

1. **Clone and navigate to the project**:
   ```bash
   cd /path/to/triage
   ```

2. **Update secrets** in `k8s/namespace.yaml`:
   ```yaml
   stringData:
     POSTGRES_PASSWORD: "your_secure_password"
     GOOGLE_CLIENT_ID: "your_google_oauth_id"
     GOOGLE_CLIENT_SECRET: "your_google_oauth_secret"
     # ... update all secrets
   ```

3. **Deploy the platform**:
   ```bash
   ./deploy.sh
   ```

4. **Access the application**:
   ```bash
   # Frontend
   kubectl port-forward -n triage service/triage-frontend-service 3000:3000
   
   # Backend API
   kubectl port-forward -n triage service/triage-backend-service 3001:3001
   ```

## Manual Deployment Steps

If you prefer manual deployment:

1. **Create namespace and secrets**:
   ```bash
   kubectl apply -f k8s/namespace.yaml
   ```

2. **Create persistent volumes**:
   ```bash
   kubectl apply -f k8s/persistent-volumes.yaml
   ```

3. **Deploy databases**:
   ```bash
   kubectl apply -f k8s/postgres.yaml
   kubectl apply -f k8s/opensearch.yaml
   kubectl apply -f k8s/neo4j.yaml
   ```

4. **Wait for databases**:
   ```bash
   kubectl wait --for=condition=available --timeout=300s deployment/postgres -n triage
   kubectl wait --for=condition=available --timeout=300s deployment/opensearch -n triage
   kubectl wait --for=condition=available --timeout=300s deployment/neo4j -n triage
   ```

5. **Deploy applications**:
   ```bash
   kubectl apply -f k8s/backend.yaml
   kubectl apply -f k8s/frontend.yaml
   ```

## Configuration

### OAuth Setup

1. **Google OAuth**:
   - Go to [Google Cloud Console](https://console.cloud.google.com)
   - Create OAuth 2.0 credentials
   - Set authorized redirect URI: `http://your-domain/auth/google/callback`

2. **Slack OAuth**:
   - Go to [Slack API](https://api.slack.com/apps)
   - Create new app
   - Set redirect URI: `http://your-domain/auth/slack/callback`

### Environment Variables

Key configuration in `k8s/namespace.yaml`:

```yaml
# ConfigMap (non-sensitive)
POSTGRES_HOST: "postgres-service"
OPENSEARCH_HOST: "opensearch-service" 
NEO4J_HOST: "neo4j-service"

# Secrets (sensitive)
POSTGRES_PASSWORD: "secure_password"
GOOGLE_CLIENT_ID: "oauth_client_id"
GOOGLE_CLIENT_SECRET: "oauth_client_secret"
```

## Storage Configuration

### Persistent Volumes

Each database has dedicated persistent storage:

- **PostgreSQL**: 10Gi (transactional data, auth tokens)
- **OpenSearch**: 20Gi (search indices, content)
- **Neo4j Data**: 5Gi (graph database)
- **Neo4j Logs**: 1Gi (application logs)

### Storage Classes

Update `storageClassName` in `k8s/persistent-volumes.yaml` for your cluster:

```yaml
# Examples:
storageClassName: standard        # GKE
storageClassName: gp2            # EKS
storageClassName: hostpath       # Local/minikube
```

## Networking

### Internal Services

All databases are accessible via cluster-internal DNS:
- `postgres-service:5432`
- `opensearch-service:9200`
- `neo4j-service:7474` (HTTP) / `neo4j-service:7687` (Bolt)

### External Access

#### Option 1: Port Forwarding (Development)
```bash
kubectl port-forward -n triage service/triage-frontend-service 3000:3000
```

#### Option 2: Ingress (Production)
Configure ingress in `k8s/frontend.yaml`:
```yaml
spec:
  rules:
  - host: your-domain.com
    http:
      paths:
      - path: /
        backend:
          service:
            name: triage-frontend-service
```

#### Option 3: LoadBalancer (Cloud)
The `triage-loadbalancer` service in `k8s/frontend.yaml` will get an external IP on cloud providers.

## Monitoring & Debugging

### Check Pod Status
```bash
kubectl get pods -n triage
kubectl describe pod <pod-name> -n triage
```

### View Logs
```bash
kubectl logs -f deployment/triage-backend -n triage
kubectl logs -f deployment/postgres -n triage
```

### Database Access

#### PostgreSQL
```bash
kubectl exec -it deployment/postgres -n triage -- psql -U triage_user -d triage
```

#### Neo4j Browser
```bash
kubectl port-forward -n triage service/neo4j-service 7474:7474
# Access: http://localhost:7474
```

#### OpenSearch Dashboards
```bash
kubectl port-forward -n triage service/opensearch-dashboards-service 5601:5601
# Access: http://localhost:5601
```

## Security Considerations

### Production Checklist

- [ ] Update all default passwords in secrets
- [ ] Enable TLS/SSL for all services
- [ ] Configure network policies
- [ ] Set up RBAC (Role-Based Access Control)
- [ ] Enable audit logging
- [ ] Configure backup strategies
- [ ] Set resource limits and requests
- [ ] Use non-root containers (already configured)
- [ ] Regular security updates

### Secret Management

For production, consider:
- **Kubernetes Secrets encryption at rest**
- **External secret management** (HashiCorp Vault, AWS Secrets Manager)
- **Service mesh** for mTLS (Istio, Linkerd)

## Scaling

### Horizontal Scaling
```bash
# Scale frontend
kubectl scale deployment triage-frontend --replicas=3 -n triage

# Scale backend
kubectl scale deployment triage-backend --replicas=3 -n triage
```

### Database Scaling
- **PostgreSQL**: Consider read replicas for heavy read workloads
- **OpenSearch**: Add more nodes to the cluster
- **Neo4j**: Upgrade to Enterprise for clustering (paid)

## Backup & Recovery

### PostgreSQL Backup
```bash
kubectl exec -it deployment/postgres -n triage -- pg_dump -U triage_user triage > backup.sql
```

### Neo4j Backup
```bash
kubectl exec -it deployment/neo4j -n triage -- neo4j-admin backup --to=/var/lib/neo4j/backups
```

### OpenSearch Snapshot
Configure snapshot repository in OpenSearch for automated backups.

## Troubleshooting

### Common Issues

1. **Pods stuck in Pending**:
   - Check persistent volume availability
   - Verify storage class exists
   - Check node resources

2. **Database connection failures**:
   - Verify service names in environment variables
   - Check database pod logs
   - Ensure init containers completed successfully

3. **OAuth redirect errors**:
   - Update redirect URIs in OAuth providers
   - Check ingress/service configuration

4. **Memory/CPU issues**:
   - Adjust resource requests/limits
   - Monitor with `kubectl top pods -n triage`

### Getting Help

- Check logs: `kubectl logs -f deployment/<deployment-name> -n triage`
- Debug pod: `kubectl exec -it deployment/<deployment-name> -n triage -- /bin/sh`
- Events: `kubectl get events -n triage --sort-by='.lastTimestamp'`

## Next Steps

After successful deployment:

1. **Configure OAuth applications** with proper redirect URIs
2. **Set up monitoring** (Prometheus/Grafana)
3. **Configure CI/CD** for automated deployments
4. **Set up backup automation**
5. **Plan for disaster recovery**
6. **Implement logging aggregation**