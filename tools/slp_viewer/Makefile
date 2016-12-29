SLP = 166
PLAYER = 1
GAME_DATA_DIR =

build:
	cargo build --release

help:
	@echo ""
	@echo "make"
	@echo "  Build slp_viewer in the release configuration"
	@echo ""
	@echo "make run"
	@echo "  Run slp_viewer the release configuration."
	@echo ""
	@echo "  Available arguments (with their default values) are:"
	@echo "    SLP=$(SLP)"
	@echo "    PLAYER=$(PLAYER)"
	@echo "    GAME_DATA_DIR=$(GAME_DATA_DIR)"
	@echo ""
	@echo "  You must set GAME_DATA_DIR, example:"
	@echo "    make run GAME_DATA_DIR=/Volumes/aoe1/GAME/DATA"
	@echo ""
	@echo "make help"
	@echo "  Display this help"
	@echo ""

run:
	cargo run --release -- \
		$(SLP) \
		--drs "$(GAME_DATA_DIR)/GRAPHICS.DRS" \
		--interfac "$(GAME_DATA_DIR)/INTERFAC.DRS" \
		--player $(PLAYER)

.PHONY: build help run