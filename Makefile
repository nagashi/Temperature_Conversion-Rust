# Makefile for temperature_conversion_rust project

# Default target
all: README.md

# Update markdown file when Rust source changes
README.md: src/main.rs
	@echo "Updating flowchart in README.md..."
	@./update_flowchart.sh

# Build the Rust program using Cargo
build: src/main.rs
	@echo "Building with Cargo..."
	@cargo build

# Clean compiled files
clean:
	@echo "Cleaning compiled files..."
	@cargo clean

# Run the program using Cargo
run: build
	@echo "Running temperature_conversion_rust..."
	@cargo run

# Watch for changes (requires fswatch on macOS)
watch:
	@echo "Watching for changes to src/main.rs..."
	@fswatch -o src/main.rs | xargs -n1 -I{} make README.md

# Help
help:
	@echo "Available targets:"
	@echo "  all       - Update markdown file (default)"
	@echo "  README.md - Update flowchart in markdown file"
	@echo "  build     - Build Haskell program with Cargo"
	@echo "  run       - Build and run the program"
	@echo "  clean     - Remove compiled files"
	@echo "  watch     - Watch for changes and auto-update"
	@echo "  help      - Show this help message"

.PHONY: all build clean run watch help
