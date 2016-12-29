SCENARIO = "MULTIP_3"
GAME_DIR = 

build:
	cargo build --release

help:
	@echo "SCENARIO=$(SCENARIO)"
	@echo "GAME_DIR=$(GAME_DIR)"

run:
	cargo run --release -- \
		"$(GAME_DIR)/Scenario/$(SCENARIO).scn" \
		-d "$(GAME_DIR)"

.PHONY: build help run