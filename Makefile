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

build: api cli front
.PHONY: build

api: target/$(ENV)/oxfeed-api
.PHONY: api

target/$(ENV)/oxfeed-api:
	$(CARGO) build $(CARGO_FLAGS) --package oxfeed-api

cli: target/$(ENV)/oxfeed-cli
.PHONY: cli

target/$(ENV)/oxfeed-cli:
	$(CARGO) build $(CARGO_FLAGS) --bin oxfeed-cli

front: yarn wasm
.PHONY: front

wasm: front/static/oxfeed_front.js
.PHONY: wasm

front/static/oxfeed_front.js:
	$(WASM_PACK) build $(WASM_PACK_FLAGS) --target web --out-dir ./static front

yarn: front/static/lib
.PHONY: yarn

front/static/lib: front/package.json
	cd front && $(YARN) $(YARN_FLAGS) install

serve: front
	microserver front/static/
.PHONY: server
