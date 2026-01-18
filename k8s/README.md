Kubernetes Deployment

Single Rust backend with a Vue frontend. The backend uses in-memory stores plus local
disk for content storage, so this setup is intended for development/demo use.

Quick start (Minikube)

1) Build images in Minikube's Docker daemon:
   eval $(minikube docker-env)
   docker compose build

2) Apply manifests:
   kubectl apply -k k8s

3) Open the frontend:
   minikube service frontend -n third-library

Notes
- Images are referenced as local tags: 3rd-library-backend:latest and 3rd-library-frontend:latest.
- The backend service is ClusterIP, consumed by the frontend over the cluster network.
