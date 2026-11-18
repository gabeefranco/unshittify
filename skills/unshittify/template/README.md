# app

A fast, text-first native client for <service>, built with Rust and Tauri. It's a rebuild of the web app that keeps the information and drops the bloat.

## Run

Prerequisites: Rust, plus the Tauri system dependencies for your OS (<https://tauri.app/start/prerequisites/>).

```sh
cd src-tauri
cargo run
```

To build installers, install the Tauri CLI (`cargo install tauri-cli --version "^2" --locked`) and run `cargo tauri build`. Run `cargo tauri icon path/to/icon.png` first to generate icons for every platform.

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
