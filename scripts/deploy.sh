#!/bin/bash
# Production Deployment Script for Quantum-Resistant Nano-Messenger
# Session 8: Production Hardening

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
VERSION="${VERSION:-2.0.0}"
ENVIRONMENT="${ENVIRONMENT:-production}"
CONFIG_FILE="${CONFIG_FILE:-$PROJECT_ROOT/config/$ENVIRONMENT.toml}"
DEPLOYMENT_MODE="${DEPLOYMENT_MODE:-docker}" # docker, kubernetes, systemd

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking deployment prerequisites..."
    
    # Check if running as appropriate user
    if [[ "$DEPLOYMENT_MODE" == "systemd" && "$EUID" -ne 0 ]]; then
        log_error "Systemd deployment requires root privileges"
        exit 1
    fi
    
    # Check required tools
    local required_tools=()
    case "$DEPLOYMENT_MODE" in
        docker)
            required_tools=("docker" "docker-compose")
            ;;
        kubernetes)
            required_tools=("kubectl" "helm")
            ;;
        systemd)
            required_tools=("systemctl" "cargo")
            ;;
    esac
    
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            log_error "Required tool '$tool' is not installed"
            exit 1
        fi
    done
    
    # Check configuration file
    if [[ ! -f "$CONFIG_FILE" ]]; then
        log_error "Configuration file not found: $CONFIG_FILE"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Validate configuration
validate_configuration() {
    log_info "Validating configuration..."
    
    # Build configuration validator
    cd "$PROJECT_ROOT"
    cargo build --release --bin config-validator || {
        log_error "Failed to build configuration validator"
        exit 1
    }
    
    # Run validation
    ./target/release/config-validator --config "$CONFIG_FILE" --environment "$ENVIRONMENT" || {
        log_error "Configuration validation failed"
        exit 1
    }
    
    log_success "Configuration validation passed"
}

# Build application
build_application() {
    log_info "Building Quantum-Resistant Nano-Messenger v$VERSION..."
    
    cd "$PROJECT_ROOT"
    
    # Build optimized release
    cargo build --release || {
        log_error "Build failed"
        exit 1
    }
    
    # Run tests
    cargo test --release || {
        log_error "Tests failed"
        exit 1
    }
    
    log_success "Build completed successfully"
}

# Docker deployment
deploy_docker() {
    log_info "Deploying with Docker..."
    
    cd "$PROJECT_ROOT"
    
    # Build Docker image
    docker build -t "nano-messenger:$VERSION" -t "nano-messenger:latest" . || {
        log_error "Docker build failed"
        exit 1
    }
    
    # Create deployment directory
    local deploy_dir="/opt/nano-messenger"
    sudo mkdir -p "$deploy_dir"
    sudo cp "$CONFIG_FILE" "$deploy_dir/config.toml"
    
    # Generate docker-compose.yml
    cat > "$deploy_dir/docker-compose.yml" << EOF
version: '3.8'

services:
  nano-messenger:
    image: nano-messenger:$VERSION
    container_name: nano-messenger-$ENVIRONMENT
    restart: unless-stopped
    ports:
      - "8080:8080"
      - "8443:8443"
      - "9090:9090"
    volumes:
      - ./config.toml:/etc/nano-messenger/config.toml:ro
      - nano-messenger-data:/var/lib/nano-messenger
      - nano-messenger-logs:/var/log/nano-messenger
    environment:
      - RUST_LOG=info
      - ENVIRONMENT=$ENVIRONMENT
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  prometheus:
    image: prom/prometheus:latest
    container_name: nano-messenger-prometheus
    restart: unless-stopped
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'

volumes:
  nano-messenger-data:
  nano-messenger-logs:
  prometheus-data:
EOF

    # Generate Prometheus configuration
    cat > "$deploy_dir/prometheus.yml" << EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'nano-messenger'
    static_configs:
      - targets: ['nano-messenger:9090']
    scrape_interval: 5s
    metrics_path: /metrics
EOF

    # Deploy
    cd "$deploy_dir"
    docker-compose up -d || {
        log_error "Docker deployment failed"
        exit 1
    }
    
    log_success "Docker deployment completed"
    log_info "Service available at: https://localhost:8443"
    log_info "Metrics available at: http://localhost:9091"
}

# Kubernetes deployment
deploy_kubernetes() {
    log_info "Deploying with Kubernetes..."
    
    local namespace="nano-messenger-$ENVIRONMENT"
    
    # Create namespace
    kubectl create namespace "$namespace" --dry-run=client -o yaml | kubectl apply -f -
    
    # Create ConfigMap from configuration file
    kubectl create configmap nano-messenger-config \
        --from-file=config.toml="$CONFIG_FILE" \
        --namespace="$namespace" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Generate Kubernetes manifests
    local k8s_dir="$PROJECT_ROOT/k8s"
    mkdir -p "$k8s_dir"
    
    # Deployment manifest
    cat > "$k8s_dir/deployment.yaml" << EOF
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nano-messenger
  namespace: $namespace
  labels:
    app: nano-messenger
    version: $VERSION
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nano-messenger
  template:
    metadata:
      labels:
        app: nano-messenger
        version: $VERSION
    spec:
      containers:
      - name: nano-messenger
        image: nano-messenger:$VERSION
        ports:
        - containerPort: 8080
          name: api
        - containerPort: 8443
          name: websocket
        - containerPort: 9090
          name: metrics
        env:
        - name: ENVIRONMENT
          value: "$ENVIRONMENT"
        - name: RUST_LOG
          value: "info"
        volumeMounts:
        - name: config
          mountPath: /etc/nano-messenger
          readOnly: true
        - name: data
          mountPath: /var/lib/nano-messenger
        - name: logs
          mountPath: /var/log/nano-messenger
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
      volumes:
      - name: config
        configMap:
          name: nano-messenger-config
      - name: data
        persistentVolumeClaim:
          claimName: nano-messenger-data
      - name: logs
        persistentVolumeClaim:
          claimName: nano-messenger-logs
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: nano-messenger-data
  namespace: $namespace
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: nano-messenger-logs
  namespace: $namespace
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 5Gi
EOF

    # Service manifest
    cat > "$k8s_dir/service.yaml" << EOF
apiVersion: v1
kind: Service
metadata:
  name: nano-messenger
  namespace: $namespace
  labels:
    app: nano-messenger
spec:
  type: LoadBalancer
  ports:
  - port: 80
    targetPort: 8080
    name: api
  - port: 443
    targetPort: 8443
    name: websocket
  - port: 9090
    targetPort: 9090
    name: metrics
  selector:
    app: nano-messenger
EOF

    # Apply manifests
    kubectl apply -f "$k8s_dir/deployment.yaml"
    kubectl apply -f "$k8s_dir/service.yaml"
    
    # Wait for deployment
    kubectl rollout status deployment/nano-messenger -n "$namespace" --timeout=300s || {
        log_error "Kubernetes deployment failed"
        exit 1
    }
    
    log_success "Kubernetes deployment completed"
    
    # Show service information
    kubectl get services -n "$namespace"
}

# Systemd deployment
deploy_systemd() {
    log_info "Deploying with systemd..."
    
    # Create service user
    if ! id "nano-messenger" &>/dev/null; then
        useradd -r -s /bin/false nano-messenger
        log_info "Created service user: nano-messenger"
    fi
    
    # Create directories
    local service_dir="/opt/nano-messenger"
    local config_dir="/etc/nano-messenger"
    local data_dir="/var/lib/nano-messenger"
    local log_dir="/var/log/nano-messenger"
    
    mkdir -p "$service_dir" "$config_dir" "$data_dir" "$log_dir"
    chown nano-messenger:nano-messenger "$data_dir" "$log_dir"
    
    # Install binary
    cp "$PROJECT_ROOT/target/release/nano-relay" "$service_dir/"
    cp "$PROJECT_ROOT/target/release/nano-client" "$service_dir/"
    chmod +x "$service_dir/nano-relay" "$service_dir/nano-client"
    
    # Install configuration
    cp "$CONFIG_FILE" "$config_dir/config.toml"
    
    # Create systemd service file
    cat > "/etc/systemd/system/nano-messenger.service" << EOF
[Unit]
Description=Quantum-Resistant Nano-Messenger
After=network.target
Wants=network.target

[Service]
Type=simple
ExecStart=$service_dir/nano-relay --config $config_dir/config.toml
Restart=always
RestartSec=10
User=nano-messenger
Group=nano-messenger

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$data_dir $log_dir
CapabilityBoundingSet=CAP_NET_BIND_SERVICE

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

# Environment
Environment=ENVIRONMENT=$ENVIRONMENT
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

    # Enable and start service
    systemctl daemon-reload
    systemctl enable nano-messenger
    systemctl start nano-messenger
    
    # Check status
    sleep 5
    if systemctl is-active --quiet nano-messenger; then
        log_success "Systemd deployment completed"
        log_info "Service status:"
        systemctl status nano-messenger --no-pager
    else
        log_error "Service failed to start"
        systemctl status nano-messenger --no-pager
        exit 1
    fi
}

# Cleanup function
cleanup() {
    log_info "Performing cleanup..."
    # Add any cleanup tasks here
}

# Main deployment function
main() {
    log_info "Starting deployment of Quantum-Resistant Nano-Messenger"
    log_info "Version: $VERSION"
    log_info "Environment: $ENVIRONMENT"
    log_info "Deployment mode: $DEPLOYMENT_MODE"
    
    # Trap cleanup function
    trap cleanup EXIT
    
    # Run deployment steps
    check_prerequisites
    validate_configuration
    build_application
    
    case "$DEPLOYMENT_MODE" in
        docker)
            deploy_docker
            ;;
        kubernetes)
            deploy_kubernetes
            ;;
        systemd)
            deploy_systemd
            ;;
        *)
            log_error "Unknown deployment mode: $DEPLOYMENT_MODE"
            log_info "Supported modes: docker, kubernetes, systemd"
            exit 1
            ;;
    esac
    
    log_success "Deployment completed successfully!"
    log_info "🚀 Quantum-Resistant Nano-Messenger v$VERSION is now running"
}

# Help function
show_help() {
    cat << EOF
Quantum-Resistant Nano-Messenger Deployment Script

Usage: $0 [OPTIONS]

Options:
    -h, --help              Show this help message
    -e, --environment ENV   Set deployment environment (production, staging, development)
    -m, --mode MODE         Set deployment mode (docker, kubernetes, systemd)
    -c, --config FILE       Specify configuration file path
    -v, --version VERSION   Set application version

Environment Variables:
    VERSION                 Application version (default: 2.0.0)
    ENVIRONMENT            Deployment environment (default: production)
    CONFIG_FILE            Configuration file path
    DEPLOYMENT_MODE        Deployment mode (default: docker)

Examples:
    $0 --environment production --mode docker
    $0 --environment staging --mode kubernetes
    $0 --environment production --mode systemd --config /path/to/config.toml

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -e|--environment)
            ENVIRONMENT="$2"
            CONFIG_FILE="$PROJECT_ROOT/config/$ENVIRONMENT.toml"
            shift 2
            ;;
        -m|--mode)
            DEPLOYMENT_MODE="$2"
            shift 2
            ;;
        -c|--config)
            CONFIG_FILE="$2"
            shift 2
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Run main function
main "$@"