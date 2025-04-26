# Task Manager

## Project Overview

A simple desktop project and task manager built with Rust and eframe/egui, persisting its state in a JSON file.

## Prerequisites

- Rust toolchain (which includes `cargo`)
- Git (if you’re cloning from a repo)

## Build & Run

**Clone & enter directory**

```bash
git clone https://github.com/yourname/task-manager.git
cd task-manager
```
# Build & run in one step:
cargo run

# Or build a production binary:
cargo build --release
./target/release/task_manager    # or .exe on Windows

## JSON Persistence

The app stores its data in `projects.json` under your OS’s config directory:

- **Windows**: `%APPDATA%\projects.json`
- **Linux**: `~/.config/projects.json`
- **macOS**: `~/Library/Application Support/projects.json`

If that file is missing or malformed, the app will start empty and recreate it on exit.
