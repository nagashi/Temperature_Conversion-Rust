#!/usr/bin/env python3
"""
File watcher script to automatically update README.md when src/main.rs changes.
Requires: pip install watchdog
"""

import sys
import subprocess
import time
from pathlib import Path
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

class RustFileHandler(FileSystemEventHandler):
    def __init__(self, script_path):
        self.script_path = script_path
        self.last_modified = 0
        
    def on_modified(self, event):
        if event.is_directory:
            return
            
        # Only process main.rs changes
        if not event.src_path.endswith('main.rs'):
            return
            
        # Debounce rapid file changes
        current_time = time.time()
        if current_time - self.last_modified < 0.5:
            return
        self.last_modified = current_time
        
        print(f"Detected change in {event.src_path}")
        print("Updating flowchart...")
        
        try:
            result = subprocess.run([self.script_path], 
                                  capture_output=True, 
                                  text=True, 
                                  check=True)
            print(result.stdout)
        except subprocess.CalledProcessError as e:
            print(f"Error updating flowchart: {e}")
            print(f"Error output: {e.stderr}")

def main():
    # Get the current directory
    current_dir = Path.cwd()
    script_path = current_dir / "update_flowchart.sh"
    
    # Check if required files exist
    if not script_path.exists():
        print(f"Error: {script_path} not found")
        sys.exit(1) # terminate program immediately and signal an error.
    
    rust_file = current_dir / "src" / "main.rs"
    if not rust_file.exists():
        print(f"Error: {rust_file} not found")
        sys.exit(1) # terminate program immediately and signal an error.
    
    # Set up file watcher
    event_handler = RustFileHandler(str(script_path))
    observer = Observer()
    observer.schedule(event_handler, str(current_dir / "src"), recursive=False)
    
    print(f"Watching {rust_file} for changes...")
    print("Press Ctrl+C to stop")
    
    observer.start()
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
        print("\nStopping file watcher...")
    
    observer.join()

if __name__ == "__main__":
    main()
