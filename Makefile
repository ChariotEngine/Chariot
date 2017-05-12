.PHONY: help check build test run
.DEFAULT_GOAL := help

# Filestem (filename minus the extension) of the scenario (map) to run.
# This value must be only the filesteam, not a path.
SCENARIO = "MULTIP_3"

# The absolute path to the 'GAME' directory provided on the retail disk.
GAME_DIR =

check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))

__check_defined = \
    $(if $(value $1),, \
      $(error $1 is not set$(if $2, $2)))

help:
	@echo ""
	@echo "> make help"
	@echo "  Display this help"
	@echo ""
	@echo "> make check"
	@echo "  Check Chariot's source for compilation errors."
	@echo "  This is much faster than a full build."
	@echo ""
	@echo "> make build"
	@echo "  Build Chariot in the release configuration."
	@echo ""
	@echo "> make test"
	@echo "  Run unit and integration tests."
	@echo ""
	@echo "> make run"
	@echo "  Build (if necessary) then run Chariot in the release configuration."
	@echo ""
	@echo "  Available arguments (with their default values) are:"
	@echo "    SCENARIO=$(SCENARIO)"
	@echo "    GAME_DIR=$(GAME_DIR)"
	@echo ""
	@echo "  All arguments must be set."
	@echo ""
	@echo "  Example:"
	@echo "    make run GAME_DIR=/Volumes/aoe1/GAME"
	@echo ""

check:
	cargo check --release

build:
	cargo build --release

test:
	cargo test --release

run:
	$(call check_defined, GAME_DIR)
	$(call check_defined, SCENARIO)
	cargo run --release -- "$(GAME_DIR)/Scenario/$(SCENARIO).SCN" --game-data-dir "$(GAME_DIR)"
