SHELL := /bin/bash

BIN ?= hymod
RUN := cargo run -q --bin $(BIN) --
BUILD_CMD := cargo build -q --bin $(BIN)
BIN_PATH := ./target/debug/$(BIN)
ARGS ?=

.PHONY: make-help run run-built noargs help new build link dev deploy server config install-local

make-help:
	@echo "Hymod CLI Make targets"
	@echo ""
	@echo "Usage:"
	@echo "  make noargs"
	@echo "  make help"
	@echo "  make <target> ARGS='...'"
	@echo ""
	@echo "Targets:"
	@echo "  run      - run arbitrary args, e.g. make run ARGS='server list'"
	@echo "  run-built - build then run compiled binary, e.g. make run-built ARGS='server list'"
	@echo "  noargs   - run hymod with no args (logo + help)"
	@echo "  help     - run hymod --help"
	@echo "  new      - run hymod new <args>"
	@echo "  build    - run hymod build <args>"
	@echo "  link     - run hymod link <args>"
	@echo "  dev      - run hymod dev <args>"
	@echo "  deploy   - run hymod deploy <args>"
	@echo "  server   - run hymod server <args>"
	@echo "  config   - run hymod config <args>"
	@echo "  install-local - build hymod and install to /usr/local/bin (uses sudo)"

run:
	$(RUN) $(ARGS)

run-built:
	$(BUILD_CMD)
	$(BIN_PATH) $(ARGS)

noargs:
	$(RUN)

help:
	$(RUN) --help

new:
	$(RUN) new $(ARGS)

build:
	$(RUN) build $(ARGS)

link:
	$(RUN) link $(ARGS)

dev:
	$(RUN) dev $(ARGS)

deploy:
	$(RUN) deploy $(ARGS)

server:
	$(RUN) server $(ARGS)

config:
	$(RUN) config $(ARGS)

install-local:
	./scripts/local_deploy.sh
