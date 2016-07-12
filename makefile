SCENARIO = "MULTIP_3"
GAME_DIR = 

openaoe:
	cargo run --release -- \
		"$(GAME_DIR)/Scenario/$(SCENARIO).scn" \
		-d "$(GAME_DIR)"

help:
	@echo "SCENARIO=$(SCENARIO)"
	@echo "GAME_DIR=$(GAME_DIR)"

build:
	cargo build --release
