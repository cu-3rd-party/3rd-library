Kubernetes Deployment

Mikroservisnoe prilozhenie razvernutoye v Minikube Kubernetes klastere. Vklyuchaet frontend na Vue.js, backend na FastAPI, bazy dannykh MongoDB/PostgreSQL, Kafka dlya asinkhronnoy kommunikatsii i MinIO dlya khraneniya faylov.

Quick start (Minikube)

1) Build images in Minikube's Docker daemon:
   eval $(minikube docker-env)
   docker compose build

2) Apply manifests:
   kubectl apply -k k8s

3) Open services:
   minikube service frontend -n third-library
   minikube service gateway -n third-library
   minikube service minio -n third-library

Notes
- Images are referenced as local tags: 3rd-library-<service>:latest.
- Services use ClusterIP internally, NodePort for frontend/gateway/minio.
