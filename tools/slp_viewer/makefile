.PHONY: help check build test run
.DEFAULT_GOAL := help

SLP = 166
PLAYER = 1
GAME_DATA_DIR =

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
	@echo "  Check slp_viewer's source for compilation errors."
	@echo "  This is much faster than a full build."
	@echo ""
	@echo "> make build"
	@echo "  Build slp_viewer in the release configuration."
	@echo ""
	@echo "> make test"
	@echo "  Run unit and integration tests."
	@echo ""
	@echo ""
	@echo "> make run"
	@echo "  Build (if necessary) then run slp_viewer in the release configuration."
	@echo ""
	@echo "  Available arguments (with their default values) are:"
	@echo "    SLP=$(SLP)"
	@echo "    PLAYER=$(GAME_DATA_DIR)"
	@echo "    GAME_DATA_DIR=$(GAME_DATA_DIR)"
	@echo ""
	@echo "  All arguments must be set."
	@echo ""
	@echo "  Example:"
	@echo "    make run GAME_DATA_DIR=/Volumes/aoe1/GAME PLAYER=4 SLP=37"
	@echo ""

check:
	cargo check --release

build:
	cargo build --release

test:
	cargo test --release

run:
	$(call check_defined, SLP)
	$(call check_defined, PLAYER)
	$(call check_defined, GAME_DATA_DIR)

	cargo run --release -- \
		$(SLP) \
		--drs "$(GAME_DATA_DIR)/GRAPHICS.DRS" \
		--interfac "$(GAME_DATA_DIR)/INTERFAC.DRS" \
		--player $(PLAYER)