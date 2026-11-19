# app

A fast, text-first native client for <service>, built with Rust and Tauri. It's a rebuild of the web app that keeps the information and drops the bloat.

## Run

Prerequisites: Rust, plus the Tauri system dependencies for your OS (<https://tauri.app/start/prerequisites/>).

```sh
cd src-tauri
cargo run
```

## Install (Linux)

```sh
sudo ./install.sh
```

This installs the build libraries with your package manager. Then it builds the app as you, and installs it into your home: the app in `~/.local/bin`, plus an icon and an entry in your app menu. Run `sudo ./install.sh` again to update. `sudo ./install.sh --uninstall` removes it, and `--no-deps` skips the system packages.

To build installers for other platforms, install the Tauri CLI (`cargo install tauri-cli --version "^2" --locked`) and run `cargo tauri build`. Run `cargo tauri icon path/to/icon.png` first to generate icons for every platform.

## Setup

<What the user must provide: developer app registration, client ID, redirect URI, etc.>

## What works

<Features, from SPEC.md.>

## Not included

<Dropped features and why.>

## Keys

| Key | Does |
|---|---|
| Alt+← / Alt+→ | back / forward |
| / | focus the page's search box |
| Tab | move between links and actions |
