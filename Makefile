INSTALL_PATH			:=$(HOME)/usr/libexec/
PHO_NAME			:=pho
PHO_DEBUG_EXEC			:=target/debug/$(PHO_NAME)
PHO_RELEASE_EXEC		:=target/release/$(PHO_NAME)
PHO_EXEC			:=$(PHO_DEBUG_EXEC)
PHO_RUN				:=$(PHO_RELEASE_EXEC)
all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(PHO_RELEASE_EXEC): $(INSTALL_PATH)
	cargo build --release

$(PHO_DEBUG_EXEC): $(INSTALL_PATH)
	cargo build

release: check fix | $(PHO_RELEASE_EXEC)
	install $(PHO_RELEASE_EXEC) $(INSTALL_PATH)

debug: check fix | $(PHO_DEBUG_EXEC)
	install $(PHO_DEBUG_EXEC) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cleanx:
	@rm -rf $(PHO_DEBUG_EXEC)
	@rm -rf $(PHO_RELEASE_EXEC)

cls:
	-@reset || tput reset

fix:
	cargo fix

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

build test: check
	cargo $@

run:
	cargo run one


.PHONY: all clean cls release debug fix fmt check build test examples
