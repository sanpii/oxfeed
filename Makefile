YARN?=yarn
YARN_FLAGS?=
CARGO?=cargo
CARGO_FLAGS?=
TRUNK?=trunk
TRUNK_FLAGS?=

ifeq ($(APP_ENVIRONMENT),prod)
	ENV=release
	YARN_FLAGS+=--production
	CARGO_FLAGS+=--release
	TRUNK_FLAGS+=--release
else
	ENV=debug
endif

.DEFAULT_GOAL := build

ifneq (,$(wildcard ./.env))
	include .env
	export
endif

build: api front
.PHONY: build

api:
	$(CARGO) build $(CARGO_FLAGS) --package oxfeed-api
.PHONY: api

front: yarn wasm
.PHONY: front

wasm: yarn
	RUST_LOG=info $(TRUNK) build $(TRUNK_FLAGS) front/index.html
.PHONY: wasm

yarn: front/static/lib
.PHONY: yarn

front/static/lib: front/package.json
	cd front && $(YARN) $(YARN_FLAGS) install

serve: serve_api serve_front
.PHONY: server

serve_api:
	$(CARGO) $(CARGO_FLAGS) run --package oxfeed-api
.PHONY: serve_api

serve_front: front
	$(TRUNK) serve $(TRUNK_FLAGS) front/index.html
.PHONY: serve_front

check:
	cargo check --package oxfeed-common
	cargo check --package oxfeed-api
	cargo check --package oxfeed-front --target wasm32-unknown-unknown
.PHONY: check

clippy:
	cargo clippy --package oxfeed-common
	cargo clippy --package oxfeed-api
	cargo clippy --package oxfeed-front --target wasm32-unknown-unknown
.PHONY: clippy
