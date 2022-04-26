.PHONY: help build run info start

export WASMCLOUD_HOST_CONSTRAINT ?= app=xkcd
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

start: ## Build, push, and start the actor and provider in this example
	@make push -sC xkcd
	@make push -sC xkcd-provider
	-make start -sC xkcd
	-make start -sC xkcd-provider
	-wash ctl start provider $(HTTPSERVER_URL) -c $(WASMCLOUD_HOST_CONSTRAINT)

link: ## Link the XKCD actor and XKCD provider on their contract ID
	$(WASH) ctl link put \
		$(shell make actor_id -sC xkcd) \
		$(shell wash par inspect xkcd-provider/build/xkcd_provider.par.gz -o json | jq -r '.service') \
		$(XKCD_CONTRACT)
	$(WASH) ctl link put \
		$(shell make actor_id -sC xkcd) \
		$(HTTPSERVER_ID) \
		$(HTTPSERVER_CONTRACT) \
		ADDRESS=0.0.0.0:8085

run: build link start ## Build and run the full example

info:
	$(WASH) --version
	echo "Host constraint: $(WASMCLOUD_HOST_CONSTRAINT)"