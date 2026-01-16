PROTO_DIR := proto
GO_AUTH_OUT := backend/auth/internal/gen
PY_STAT_OUT := backend/statistics/src/proto
PY_NOTIF_OUT := backend/notification/src/proto
PY_ENG_OUT := backend/engagement/src/proto
PY_CONTENT_OUT := backend/content/src/proto
GOBIN := $(shell go env GOPATH)/bin
PATH := $(GOBIN):$(PATH)
UV ?= uv
PYTEST_CMD := $(shell command -v $(UV) >/dev/null 2>&1 && echo "$(UV) run pytest" || echo "python -m pytest")
COLOR_RESET := \033[0m
COLOR_RED := \033[0;31m
COLOR_GREEN := \033[0;32m
COLOR_CYAN := \033[0;36m

.PHONY: proto proto-go proto-python proto-rust proto-clean test test-auth test-content test-engagement test-notification test-statistics e2e

proto: proto-go proto-python proto-rust

proto-go:
	protoc -I $(PROTO_DIR) \
		--go_out=$(GO_AUTH_OUT) --go_opt=paths=source_relative \
		--go-grpc_out=$(GO_AUTH_OUT) --go-grpc_opt=paths=source_relative \
		$(PROTO_DIR)/auth.proto

proto-python:
	cd backend/statistics && $(UV) run python -m grpc_tools.protoc -I ../../$(PROTO_DIR) \
		--python_out=src/proto --grpc_python_out=src/proto \
		../../$(PROTO_DIR)/statistics.proto
	cd backend/notification && $(UV) run python -m grpc_tools.protoc -I ../../$(PROTO_DIR) \
		--python_out=src/proto --grpc_python_out=src/proto \
		../../$(PROTO_DIR)/notification.proto
	cd backend/engagement && $(UV) run python -m grpc_tools.protoc -I ../../$(PROTO_DIR) \
		--python_out=src/proto --grpc_python_out=src/proto \
		../../$(PROTO_DIR)/engagement.proto
	cd backend/content && $(UV) run python -m grpc_tools.protoc -I ../../$(PROTO_DIR) \
		--python_out=src/proto --grpc_python_out=src/proto \
		../../$(PROTO_DIR)/content.proto

proto-rust:
	$(MAKE) -C backend/gateway build-proto

proto-clean:
	rm -f $(GO_AUTH_OUT)/*.pb.go
	rm -f $(PY_STAT_OUT)/*_pb2.py $(PY_STAT_OUT)/*_pb2_grpc.py
	rm -f $(PY_NOTIF_OUT)/*_pb2.py $(PY_NOTIF_OUT)/*_pb2_grpc.py
	rm -f $(PY_ENG_OUT)/*_pb2.py $(PY_ENG_OUT)/*_pb2_grpc.py
	rm -f $(PY_CONTENT_OUT)/*_pb2.py $(PY_CONTENT_OUT)/*_pb2_grpc.py

test: test-auth test-content test-engagement test-notification test-statistics

test-auth:
	cd backend/auth && go test ./...

test-content:
	cd backend/content && $(PYTEST_CMD)

test-engagement:
	cd backend/engagement && $(PYTEST_CMD)

test-notification:
	cd backend/notification && $(PYTEST_CMD)

test-statistics:
	cd backend/statistics && $(PYTEST_CMD)

e2e:
	@printf "$(COLOR_CYAN)Running Selenium E2E tests...$(COLOR_RESET)\n"
	@python e2e/run_e2e.py
