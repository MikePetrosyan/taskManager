# Task Manager

## Project Overview

A simple desktop project and task manager built with Rust and eframe/egui, persisting its state in a JSON file.

## Prerequisites

- Rust toolchain (which includes `cargo`)
- Git (if you’re cloning from a repo)

## Build & run in one step:

```sh
cargo run
```
## Keyboard Shortcuts

- Ctrl + N: Create new project
- Ctrl + T: Create new task
- F2: Rename selected project
- Delete: Delete selected project
- Escape: Exit dialog
- Enter: Submit dialog

## JSON Persistence

The app stores its data in `projects.json` under your OS’s config directory:

- **Windows**: `%APPDATA%\projects.json`
- **Linux**: `~/.config/projects.json`
- **macOS**: `~/Library/Application Support/projects.json`

If that file is missing or malformed, the app will start empty and recreate it on exit.
