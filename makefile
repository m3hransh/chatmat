# Variables
FRONTEND_DIR = chatmat-front
BACKEND_DIR = chatmat-back

# Frontend commands
PNPM_RUN_BUILD = pnpm run build
PNPM_INSTALL = pnpm install
PNPM_DEV = pnpm run dev
PNPM_TEST = pnpm playwright test 

# Targets
.PHONY: all build-front build-back clean dev-front dev-back test-front test-back help

all: pre-front pre-back ## Prepares both frontend and backend

pre-front: $(FRONTEND_DIR)/node_modules ## Prepares the frontend by installing dependencies

$(FRONTEND_DIR)/node_modules: $(FRONTEND_DIR)/package.json $(FRONTEND_DIR)/pnpm-lock.yaml
	cd $(FRONTEND_DIR) && $(PNPM_INSTALL)

clean: ## Cleans up generated files and stops Docker containers
	rm -rf $(FRONTEND_DIR)/node_modules $(FRONTEND_DIR)/dist  \
	  && docker compose down -v

proto: ## update the proto file in front
	protoc -I=proto helloworld.proto  --grpc-web_out=import_style=commonjs,mode=grpcwebtext:chatmat-front/src/proto --js_out=import_style=commonjs:chatmat-front/src/proto

dev-back: ## Runs the backend in development mode
	cd $(BACKEND_DIR) && cargo run --bin chatmat-server

dev-envoy: ## Run envoy 
	envoy -c envoy/envoy-dev.yaml

dev-front: ## Runs the frontend in development mode
	cd $(FRONTEND_DIR) && $(PNPM_DEV)

test-front: ## Runs the frontend tests
	cd $(FRONTEND_DIR) && $(PNPM_TEST)

help: ## Displays this help message
	@echo "Usage: make [TARGET]"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*?## "}; /^[a-zA-Z_-]+:.*?## / { printf "  %-20s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: help

