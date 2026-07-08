# Install the GUI

The GUI is built with [Tauri](https://tauri.app/) (Rust backend) and React + Mantine (frontend). It wraps the CLI and is the recommended interface for clinicians and patients.

!!! note "CLI first"
    [Install the CLI](cli.md) before installing the GUI. The GUI shells out to the CLI for all data operations.

## Prerequisites

- `cargo` available on your PATH.
- `npm` (Node.js) available on your PATH. This is a **build-time and dev-build** dependency: it compiles the frontend and, in development mode (`s/gui-dev` / `npm run tauri dev`), runs the live vite dev server. A packaged release build embeds the compiled frontend into the binary and does **not** need npm at runtime.

### Linux system dependencies

The Tauri GUI on Linux renders its UI through the system's **WebKitGTK** webview, so `webkit2gtk-4.1` is a **runtime dependency, not just a build one** - the compiled GUI dynamically links it and will not start without it, on both dev and release builds. Every Linux machine that runs the GUI needs it present (a packaged install should declare it as a dependency so the user's package manager pulls it in).

The list below is for Debian and Ubuntu; adjust for your distribution.

```sh
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

!!! note "Package names vary by distribution"
    On Debian/Ubuntu the runtime library and the build headers are split (`libwebkit2gtk-4.1-0` at runtime, `libwebkit2gtk-4.1-dev` to build). On Arch the single `webkit2gtk-4.1` package provides both. See the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/) for the current package names for your distribution.

## Run from source

From the repo root:

```sh
s/gui-dev
```

This starts the GUI in development mode against the local source. It is equivalent to running `npm run tauri dev` inside `gui/`.

## What's next

- [GUI Quick Start](../gui/quick-start.md) - first-time use.
- [GUI overview](../gui/gui.md) - walkthrough of the main panels.
