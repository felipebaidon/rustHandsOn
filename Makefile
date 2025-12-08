SHELL := /bin/bash
DIR = .

.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

##ToDo: implement clean based on passed directory
##ToDO: implement cle an all 
clean: ## Clean the project using cargo
	cargo -Z unstable-options -C $(DIR) clean

#ToDo: implement build all
build: ## Build the rust passed directory using cargo, for this the rust nightly channel is used
	echo $(DIR)
	cargo -Z unstable-options -C $(DIR) build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt
