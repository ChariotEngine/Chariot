SLP = 166
PLAYER = 1
GAME_DATA_DIR =

build:
	cargo build --release

help:
	@echo "SLP=$(SLP)"
	@echo "PLAYER=$(PLAYER)"
	@echo "GAME_DATA_DIR=$(GAME_DATA_DIR)"

run:
	cargo run --release -- \
		$(SLP) \
		--drs "$(GAME_DATA_DIR)/GRAPHICS.DRS" \
		--interfac "$(GAME_DATA_DIR)/INTERFAC.DRS" \
		--player $(PLAYER)

.PHONY: build help run