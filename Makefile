.PHONY: help build run info

WASMCLOUD_HOST_CONSTRAINT ?= "app=xkcd"
WASH ?= $(shell which wash)

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

build:
	make -C interface
	make -C xkcd
	make -C xkcd-provider

run: build
	make start -C xckd
	make start -C xckd-provider

info:
	$(WASH) --version
	echo "Host constraint: $(WASMCLOUD_HOST_CONSTRAINT)"