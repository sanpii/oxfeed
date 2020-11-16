.DEFAULT_GOAL := build

build: api cli front
.PHONY: build

api:
	cargo build --package oxfeed-api
.PHONY: api

cli:
	cargo build --bin update
.PHONY: cli

front: yarn
	cd front && wasm-pack build --target web --out-name wasm --out-dir ./static
.PHONY: front

yarn: front/static/lib
.PHONY: yarn

front/static/lib: front/package.json
	cd front && yarn install

serve: front
	microserver front/static/
.PHONY: server
