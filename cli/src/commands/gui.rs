// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

fn is_gitehr_repo() -> bool {
    PathBuf::from(".gitehr").exists()
}

// Locates a bundled or installed GUI binary for the release launch path
// described in `run()`. Not yet wired up (dev mode runs `npm run tauri dev`).
#[allow(dead_code)]
fn find_gui_binary() -> Option<PathBuf> {
    let bundled_path = PathBuf::from(".gitehr/gitehr-gui");
    if bundled_path.exists() {
        return Some(bundled_path);
    }

    #[cfg(target_os = "windows")]
    let bundled_exe = PathBuf::from(".gitehr/gitehr-gui.exe");
    #[cfg(target_os = "windows")]
    if bundled_exe.exists() {
        return Some(bundled_exe);
    }

    if let Ok(path) = which::which("gitehr-gui") {
        return Some(path);
    }

    None
}

/// Launch the GitEHR GUI application
/// For development, launches with: WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev
/// For release, should launch the compiled, OS-appropriate GUI binary
pub fn run() -> Result<()> {
    let gui_dir = "gui";

    // Tauri's `beforeDevCommand` runs vite, so the frontend deps (vite, the
    // tauri CLI, React, ...) must be present in `gui/node_modules`. Bootstrap
    // them on first launch so `gitehr gui` works without a manual `npm install`.
    if !PathBuf::from(gui_dir).join("node_modules").exists() {
        eprintln!("Installing GUI dependencies (first run, this may take a moment)...");
        if !npm(&["install"], gui_dir)?.success() {
            anyhow::bail!("`npm install` failed in {gui_dir}/ - see the output above.");
        }
    }

    // Development mode: run tauri dev. A non-zero exit here is most often a
    // missing native prerequisite for the Tauri build (the child's own error
    // has already streamed above), so point at that.
    if !npm(&["run", "tauri", "dev"], gui_dir)?.success() {
        anyhow::bail!(
            "Failed to launch the GUI in dev mode (`npm run tauri dev`). See the build \
output above for the cause. This is usually a missing system prerequisite for the \
Tauri build - on Linux the GUI needs the WebKitGTK dev libraries (e.g. \
`webkit2gtk-4.1`); see https://tauri.app/start/prerequisites/ for your platform."
        );
    }
    Ok(())
}

/// Run `npm <args>` in `dir`, returning the exit status. A missing `npm` binary
/// (Node.js not installed) is turned into a message that names npm; callers
/// decide what a non-zero exit means for the command they ran.
fn npm(args: &[&str], dir: &str) -> Result<std::process::ExitStatus> {
    Command::new("npm")
        .args(args)
        .env("WEBKIT_DISABLE_DMABUF_RENDERER", "1")
        .current_dir(dir)
        .status()
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => anyhow::anyhow!(
                "`npm` was not found on your PATH. The GitEHR GUI needs Node.js (which \
includes npm) to run in dev mode - install it from https://nodejs.org and try again."
            ),
            _ => anyhow::anyhow!("failed to run `npm {}`: {e}", args.join(" ")),
        })
}
