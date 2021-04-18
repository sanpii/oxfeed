YARN?=yarn
YARN_FLAGS?=
CARGO?=cargo
CARGO_FLAGS?=
WASM_PACK?=wasm-pack
WASM_PACK_FLAGS?=

ifeq ($(APP_ENVIRONMENT),prod)
	ENV=release
	YARN_FLAGS+=--production
	CARGO_FLAGS+=--release
	WASM_PACK_FLAGS+=--release
else
	ENV=debug
	WASM_PACK_FLAGS+=--dev
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

wasm:
	RUST_LOG=info $(WASM_PACK) build $(WASM_PACK_FLAGS) --target web --out-dir ./static front
	ln --relative --force --symbolic $(shell ls -rt $(shell find target/ -name index.html | grep ".") | tail -1) front/static/index.html
	rm front/static/.gitignore
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
	microserver front/static/
.PHONY: serve_front
