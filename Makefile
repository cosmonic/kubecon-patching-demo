.PHONY: help build run info start

HTTPSERVER_URL ?= wasmcloud.azurecr.io/httpserver:0.15.0
HTTPSERVER_ID ?= VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
HTTPSERVER_CONTRACT ?= wasmcloud:httpserver
XKCD_CONTRACT ?= cosmonic:xkcd
WASH ?= $(shell which wash)

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

build: ## Build the interface, actor, and provider for this example
	@make -sC interface
	@make -sC xkcd
	@make -sC xkcd-provider

link: ## Link the XKCD actor and XKCD provider on their contract ID
	@$(WASH) ctl link put \
		$(shell make actor_id -sC xkcd) \
		$(shell wash par inspect xkcd-provider/build/xkcd_provider.par.gz -o json | jq -r '.service') \
		$(XKCD_CONTRACT)
	@$(WASH) ctl link put \
		$(shell make actor_id -sC xkcd) \
		$(HTTPSERVER_ID) \
		$(HTTPSERVER_CONTRACT) \
		ADDRESS=0.0.0.0:8085

run: build link ## Build and run the full example
	-@wash ctl start provider $(HTTPSERVER_URL) --skip-wait
	@echo "Prerequisite services started and linked, navigate to http://localhost:4000 to start xkcd components from file"
	@echo "Once you've started 'xkcd' and the 'xkcd-provider', you can view your random comic at http://localhost:8085"

run-registry: build link start ## Build and run the full example, pushing to a local OCI compliant registry

info:
	$(WASH) --version
