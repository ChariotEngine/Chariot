SCENARIO = "MULTIP_3"
GAME_DIR = 

build:
	cargo build --release

help:
	@echo ""
	@echo "make"
	@echo "  Build OpenAOE in the release configuration"
	@echo ""
	@echo "make run"
	@echo "  Run in the release configuration."
	@echo ""
	@echo "  Available arguments (with their default values) are:"
	@echo "    SCENARIO=$(SCENARIO)"
	@echo "    GAME_DIR=$(GAME_DIR)"
	@echo ""
	@echo "  You must set GAME_DIR, example:"
	@echo "    make run GAME_DIR=/Volumes/aoe1/GAME"
	@echo ""
	@echo "make help"
	@echo "  Display this help"
	@echo ""

run:
	cargo run --release -- \
		"$(GAME_DIR)/Scenario/$(SCENARIO).scn" \
		-d "$(GAME_DIR)"

.PHONY: build help run