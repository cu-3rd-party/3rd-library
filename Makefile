COLOR_RESET := \033[0m
COLOR_CYAN := \033[0;36m

.PHONY: backend-check backend-test e2e

backend-check:
	cargo check -p backend

backend-test:
	cargo test -p backend

e2e:
	@printf "$(COLOR_CYAN)Running Selenium E2E tests...$(COLOR_RESET)\n"
	@python e2e/run_e2e.py
