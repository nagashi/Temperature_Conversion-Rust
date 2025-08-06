# Automation Flowchart Update System

This setup allows for the automation update of the Mermaid flowchart in `README.md` whenever the modification of the algorithm in `src/main.rs` takes place.

## How It Works

1. **Embedded Diagram**: The Mermaid flowchart is embedded directly in `app/Main.hs` as comments between `MERMAID_DIAGRAM_START` and `MERMAID_DIAGRAM_END` markers.

2. **Extraction Script**: `update_flowchart.sh` extracts the diagram from the Haskell code and updates the markdown file.

3. **Automation Options**: Several tools are provided to automatically run the update when the code changes.

## Usage Options

### Option 1: Manual Update
```bash
./update_flowchart.sh
```

### Option 2: Using Make
```bash
# Update flowchart when app/Main.hs changes
make README.md

# Or use the default target
make

# Watch for changes automatically (requires fswatch on macOS)
make watch
```

### Option 3: Python File Watcher (Cross-platform)
```bash
# Install required dependency
pip install watchdog

# Start watching
./watch_and_update.py
```

## File Structure

```
├── src/main.rs         # Rust source with embedded Mermaid diagram
├── README.md           # Markdown file with flowchart
├── update_flowchart.sh # Script to extract and update diagram
├── Makefile            # Build automation
├── watch_and_update.py # Python file watcher
└── README_FLOWCHART.md # This file
```

## Maintaining the Diagram

When the algorithm in `src/main.rs` is modified, also update the embedded Mermaid diagram in the comments. The automation will handle updating `README.md`.

### Example workflow:
1. Edit `src/main.rs` - modify both code and embedded diagram to match new algorithm
2. Save the file
3. If using a watcher, `README.md` updates automatically
4. If not, run `./update_flowchart.sh` or `make`

## Benefits of the Semi-Automatic Approach:

- **✅ Single Source of Truth**: The diagram lives with the code
- **✅ Automatic Updates**: No manual copying between files
- **✅ Version Control**: Diagram changes are tracked with code changes
- **✅ Consistency**: Diagram stays in sync with code modifications 
***
```
The system, as currently constructed, gives users multiple ways to implement automation based on their preferences and environment.
```
***

## Limitations

- You must manually update the embedded diagram when changing the algorithm
- The diagram extraction relies on specific comment markers
- File watchers require additional dependencies

## Advanced Usage

You can extend this system by:
- Adding the update script to your git pre-commit hooks
- Integrating with your IDE's build system
- Adding validation to ensure the diagram syntax is correct
- Creating templates for different types of algorithms

## Why It's Not Fully Automatic:

The embedded diagram doesn't automatically generate from your code because:
-  Code analysis is complex: Automatically parsing Rust logic flow would require sophisticated Abstract Syntax Tree (AST) analysis
-  Design decisions: You decide what level of detail to show in the flowchart
-  Flexibility: You control how to represent complex logic (loops, conditionals, etc.) in the visual format