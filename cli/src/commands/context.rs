// SPDX-FileCopyrightText: 2026 Marcus Baw and Baw Medical Ltd
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Store/repo working-context resolution (see spec/adr/0005).
//!
//! GitEHR finds its working context the way git finds `.git/`: by walking up
//! from the current directory. Repo-level commands resolve a subject repo
//! (`.gitehr/`); store-level commands resolve the Store root (`gitehr-mpi.json`).
//! When run at a Store root with exactly one subject, repo-level commands
//! auto-target it, so a lone self-hoster never has to `cd` into the subject.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

const REPO_MARKER: &str = ".gitehr";
const STORE_MARKER: &str = "gitehr-mpi.json";

/// Nearest ancestor of the cwd (inclusive) that contains `marker`.
fn find_up(marker: &str) -> Result<Option<PathBuf>> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join(marker).exists() {
            return Ok(Some(dir));
        }
        if !dir.pop() {
            return Ok(None);
        }
    }
}

/// Resolve the Store root for a store-level command. An explicit `--store`
/// overrides directory detection and the configured default.
pub fn resolve_store_root(store_override: Option<&Path>) -> Result<PathBuf> {
    if let Some(path) = store_override {
        return validated_store(path);
    }
    match find_up(STORE_MARKER)? {
        Some(root) => Ok(root),
        None => configured_store_root()?.ok_or_else(|| {
            anyhow::anyhow!(
                "Not inside a GitEHR Store (no {STORE_MARKER} found). Run `gitehr store init` to create one, or set one with `gitehr config set-store <path>`."
            )
        }),
    }
}

/// Resolve the subject repo for a repo-level command.
///
/// With neither selector, this is directory-driven: the nearest `.gitehr/`
/// ancestor, or - at a Store with exactly one subject - that subject. An
/// explicit `--subject` and/or `--store` instead selects from the Store's MPI,
/// letting you target one subject in a multi-subject Store from anywhere.
pub fn resolve_repo_root(subject: Option<&str>, store_override: Option<&Path>) -> Result<PathBuf> {
    // Without explicit selectors, honour the surrounding directory first.
    if subject.is_none()
        && store_override.is_none()
        && let Some(repo) = find_up(REPO_MARKER)?
    {
        return Ok(repo);
    }

    // Otherwise (or when not inside a repo) we need a Store to select from.
    let store = match store_override {
        Some(path) => validated_store(path)?,
        None => match find_up(STORE_MARKER)? {
            Some(store) => store,
            None => configured_store_root()?.ok_or_else(|| {
                anyhow::anyhow!(
                    "Not a GitEHR repository or Store. Run `gitehr store init` to create one, or set one with `gitehr config set-store <path>`."
                )
            })?,
        },
    };

    let subjects = subjects(&store)?;

    if let Some(selector) = subject {
        return subjects
            .iter()
            .find(|s| s.matches(selector))
            .map(|s| s.path.clone())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No subject '{selector}' in the Store at {} (matched against subject name and MPI id). List subjects with `gitehr store list`.",
                    store.display()
                )
            });
    }

    match subjects.as_slice() {
        [only] => {
            eprintln!(
                "Note: no subject supplied and the Store is a singleton; subject '{}' inferred.",
                only.repo_path
            );
            Ok(only.path.clone())
        }
        [] => bail!("This Store has no subjects yet. Add one with `gitehr store add [name]`."),
        many => bail!(
            "No subject selected. The Store at {} has {} subjects; choose one with `--subject <name|id>` (e.g. `--subject {}`), or cd into the subject's directory.",
            store.display(),
            many.len(),
            many[0].repo_path
        ),
    }
}

fn configured_store_root() -> Result<Option<PathBuf>> {
    let Some(store) = crate::config::configured_store_path()? else {
        return Ok(None);
    };

    if store.join(STORE_MARKER).exists() {
        Ok(Some(store))
    } else {
        bail!(
            "Configured GitEHR Store path {} does not contain {STORE_MARKER}. Update it with `gitehr config set-store <path>` or override it with {}.",
            store.display(),
            crate::config::STORE_PATH_ENV
        )
    }
}

/// A subject discovered in the Store's MPI.
struct Subject {
    /// On-disk directory / friendly name (`repo_path` in the MPI).
    repo_path: String,
    /// Canonical MPI id (`patient_id`).
    patient_id: String,
    /// Absolute path to the subject repo.
    path: PathBuf,
}

impl Subject {
    /// A `--subject` selector matches either the friendly name or the MPI id,
    /// mirroring how `gitehr store remove` accepts either.
    fn matches(&self, selector: &str) -> bool {
        // An empty selector must not match an id-less subject (patient_id == "").
        !selector.is_empty() && (self.repo_path == selector || self.patient_id == selector)
    }
}

/// Subjects in the Store's MPI (parsed loosely so this module does not depend on
/// the store command's structs).
fn subjects(store: &Path) -> Result<Vec<Subject>> {
    let mpi: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(store.join(STORE_MARKER))?)?;
    let mut out = Vec::new();
    if let Some(arr) = mpi.get("patients").and_then(|v| v.as_array()) {
        for p in arr {
            let Some(repo_path) = p.get("repo_path").and_then(|v| v.as_str()) else {
                continue;
            };
            let patient_id = p
                .get("patient_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            out.push(Subject {
                repo_path: repo_path.to_string(),
                patient_id: patient_id.to_string(),
                path: store.join(repo_path),
            });
        }
    }
    Ok(out)
}

/// Validate and absolutise an explicit `--store` path: it must be an existing
/// Store root (contain the MPI marker).
fn validated_store(path: &Path) -> Result<PathBuf> {
    let store = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    if store.join(STORE_MARKER).exists() {
        Ok(store)
    } else {
        bail!(
            "--store {} is not a GitEHR Store root ({STORE_MARKER} not found).",
            store.display()
        )
    }
}
