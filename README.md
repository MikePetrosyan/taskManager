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

The app stores its data in `app.ron` under your OS’s data directory:

- Linux: /home/UserName/.local/share/task-manager
- macOS: /Users/UserName/Library/Application Support/task-manager
- Windows: C:\Users\UserName\AppData\Roaming\task-manager\data

If that file is missing or malformed, the app will start empty and recreate it on exit.
