# GitEHR Roadmap (Spec-Aligned)

This roadmap tracks implementation status against the current `spec/` documents.

## Core CLI and Repository Lifecycle

- [x] **Store-first bootstrap ([ADR-0005](adr/0005-store-first-model.md)):** make `gitehr store init` create the Store, the MPI, and the first subject repo in one step (reusing the repo-scaffolding from the old `gitehr store init`), and **remove the top-level `gitehr init`**. Subject repos use the UUIDv7 + Crockford directory naming from the spec.
- [ ] Add robust `gitehr journal verify --verbose` (or equivalent) failure diagnostics per spec TODO.

## Import and Document Capture

Bringing existing records into a repository (see [`spec/commands/import.md`](commands/import.md), [`docs/cli/import.md`](../docs/cli/import.md), `cli/src/commands/import.rs`).

- [ ] **Built-in OCR for imported documents (eventually).** Make it as trivially easy as possible for less-technical patients using the GUI to bring their own medical records together. When importing a scan or photo via `--mode documents`, run OCR so the journal entry carries searchable, machine-readable text alongside the original file, not just a link - a patient should be able to drop in a photo of a letter and get a real, searchable record entry with near-zero friction. Keep it **built-in and offline** (no shipping medical images to a cloud OCR service), and treat the OCR text as a derived convenience layer, never a replacement for the original document.
- [ ] Add further import modes as the need arises (e.g. an imaging-scanned mode), per the "other modes later" note in `spec/commands/import.md`.
- [ ] Once a config file exists, let `--mode documents` filter against a configured file-format whitelist (per the TODO in `spec/commands/import.md`).

## Command Coverage vs Spec

- [x] Restore `gitehr calc` clinical calculators once the pacharanero/calc crates are published to crates.io. The command was temporarily detached to keep GitEHR's release pipeline free of git-only dependencies.
- [ ] Add `gitehr export` - generate standardised export bundles (FHIR / EHRxF / openEHR) from a repository for cross-border sharing and portability (see `spec/fhir-openehr.md` and the EHDS/EHRxF notes in `spec/long-term-ideas.md`).
- [ ] **Store-first, remaining ([ADR-0005](adr/0005-store-first-model.md)):** Store/repo context detection (walk up for `.gitehr/` and `gitehr-mpi.json`) plus single-subject auto-targeting; then **remove the top-level `gitehr store init`** and move the test suite onto `gitehr store init`. The MPI identifier operations (`search`, `link`, `unlink`, `merge`, `path`) and the `GITEHR_MPI_PATH` override fold in as `gitehr store` subcommands later.
- [ ] **Self-hoster on-ramp docs (families and pets):** make the single-user, multi-subject story first-class on the site and in GUI onboarding - individuals and families keeping their own records, and **pet owners** keeping their animals' records - alongside the clinic story. Per ADR-0005 these are primary audiences, not afterthoughts.
- [ ] Align `gitehr gui` launcher with command spec (prefer bundled `.gitehr/gitehr-gui`, then PATH `gitehr-gui`; current implementation still launches dev command).

## Repository Template and Data Layout

- [ ] Add `/fhir/` layout (`definitions`, `resources`, `indexes`) to template and lifecycle docs.
- [ ] Add `/openehr/` layout and storage conventions from spec.

## FHIR v5 Workstream

- [ ] Add/confirm spec-linked lifecycle docs for FHIR storage and journaling.
- [ ] Build tooling to download pinned FHIR v5 definitions into `/fhir/definitions`.
- [ ] Implement Rust FHIR modules (`src/fhir/`) for definitions loading and resource validation.
- [ ] Add CLI commands for FHIR import and validation.
- [ ] Add journal structured references for FHIR resource provenance.
- [ ] Add tests and documentation for FHIR workflows.

## openEHR Workstream

- [ ] Design and implement native openEHR RM storage model.
- [ ] Implement required openEHR REST endpoints and content negotiation.
- [ ] Add archetype/template validation integration.
- [ ] Implement versioning/audit/contribution semantics for openEHR entities.
- [ ] Add AQL query support and conformance manifest/OPTIONS support.
- [ ] Add conformance testing and implementation documentation.

## GUI and UX

- [ ] Keep GUI launch behavior aligned with CLI command spec for bundled-binary-first execution.
- [ ] Add/restore GUI E2E coverage and keep it green in CI.

## Clinical Calculators Workstream

The calculators live in their own repository, **[pacharanero/calc](https://github.com/pacharanero/calc)** (`~/code/pacharanero/calc`), built and tested there. GitEHR will consume them again once `calc-cli` and `calc-core` are published to crates.io. The integration is temporarily dormant to keep GitEHR's release pipeline free of git-only calculator dependencies. The architecture, roadmap, and input-definition design specs moved with them to that repo's `spec/`.

- [ ] Restore the `gitehr calc` subcommand and MCP `calc_<name>` tools once `calc-cli`/`calc-core` are published to crates.io. They were temporarily detached from GitEHR so releases do not depend on pre-crates.io git dependencies.
- [ ] Switch the `calc-cli`/`calc-core` dependencies to crates.io once pacharanero/calc has a distribution pipeline.
- [ ] Record calculator results in the journal (immutable entry: calculator, version, inputs, result, citation) - GitEHR-side integration.
- [ ] Add state file storage for latest results (`state/calculations/<name>-latest.json`) - GitEHR-side.
- [ ] Add a GUI calculator panel + Tauri `calculate_clinical` command calling `calc_core` natively.

## Model Context Protocol (MCP) Server

- [ ] Implement MCP JSON-RPC 2.0 protocol with stdio/HTTP/SSE transports.
- [ ] Add MCP resource handlers (journal, state, imaging, documents, status).
- [ ] Add MCP tool handlers (add_journal_entry, update_state, verify_journal, search; restore calculate_clinical after calc crates are published).
- [ ] Add MCP prompt templates (SOAP note, discharge summary, referral, medication review).
- [ ] Implement token-based authentication with `.gitehr/mcp-tokens.json`.
- [ ] Add MCP audit logging to journal entries.
- [ ] Implement encryption awareness (respect `.gitehr/ENCRYPTED` marker).
- [ ] Add MCP configuration system (`.gitehr/mcp.json`).
- [ ] Restore MCP calculator tools: each `calc-core` calculator should be exposed as a `calc_<name>` MCP tool whose `inputSchema` is the calculator's own JSON Schema; `tools/call` should run the shared engine and return the `CalculationResponse`.
- [ ] Add GUI MCP client panel for LLM chat interface.
- [ ] Document MCP integration guide and API reference.
- [ ] Add MCP client libraries (Python/TypeScript) for testing.

## Security and Integrity (to review)

- [ ] **Hardware-backed contributor signing credentials.** Design support for contributors to hold signing credentials off-device on a hardware authenticator such as a YubiKey, PIV/smartcard, TPM-backed key, Secure Enclave, or equivalent. The intended workflow is that a contributor presents/unlocks the hardware credential when signing a GitEHR journal entry or commit, so the private signing material does not live as an ordinary file on the workstation. This needs to integrate with `.gitehr/contributors.json`, contributor activation, repository signing policy, recovery/revocation, and offline use.
- [ ] **Evaluate [gittuf](https://gittuf.dev/) for GitEHR.** gittuf applies The Update Framework (TUF) concepts to a Git repository, adding security that Git itself lacks: policy-controlled, signed access to branches/tags/refs, key management and rotation, and protection against attacks on references (unauthorised ref updates, rollback/freeze, tag tampering). This is directly relevant to GitEHR's integrity, provenance, and tamper-evidence goals (who may update which refs in a patient repository, and proving a ref's history has not been rewritten). It is still in beta - the action is to keep an eye on it and review whether and how it can fit into GitEHR's security model once it matures.

## Documentation and Operations

- [ ] Restructure top-level nav to seven tabs: Home, Design, Install, CLI, GUI, TUI, Safety. Move existing content into the new sections; create stubs for sections that do not yet have content (TUI, Safety).
- [ ] Keep command docs consistently aligned with runtime behavior.
- [ ] Expand user-facing docs (Install, CLI reference, GUI walkthroughs, TUI overview once it exists, Safety / Turva, troubleshooting).
- [ ] Document packaging strategy for CLI+GUI distribution and upgrade/migration compatibility.
- [ ] Add calculator usage guide with clinical examples and validation references.
- [ ] Add MCP integration guide for LLM application developers.
- [ ] Document long-term strategic considerations (EHDS, EHRxF, quantum crypto, federated learning).

## Site Content (gitehr.org)

Source: `gitehr-site-improvement-handoff.md` at the repo root. Goal is to strengthen the "files on disk vs databases" argument that underpins GitEHR's design, by framing it as the consensus the rest of software has already reached rather than as a healthcare-specific opinion. Style: ASCII hyphen-minus only (no emdash), MkDocs-compatible admonitions (work in Zensical's classic variant), relative internal links.

### Medium priority

- [ ] Add a "Common objections" or FAQ page covering: cross-patient queries for research and population health (org-level derived databases built from canonical files, mirroring Iceberg-over-Parquet); concurrent edits (Git branch-and-merge with clinical conflict resolution); ACID and consistency (per-file atomicity plus cryptographic chain-of-custody); GDPR right to erasure (the hardest one - needs careful framing given Git's immutable history).
- [ ] Cross-reference the wider movement with explicit links: Ink and Switch local-first paper (Kleppmann et al. 2019), Steph Ango's "File over app" essay, Pat Helland's "Immutability Changes Everything" (2015), Apache Iceberg, SQLite-as-archival-format. Add to a references section or inline citations.
- [ ] Expand the N-squared integration problem into its own paragraph plus a diagram: N organisations with their own databases produces N(N-1)/2 integration pairs; N organisations agreeing on a file format produces N implementations and zero pairs.
- [ ] Add a section (in `design/files-not-databases.md` or its own page) on the agentic coding angle: clinical LLM applications can read, diff, and answer questions over a Git history of markdown files in ways that map poorly to databases. Files give you `grep`, `diff`, `git log`, and a full audit trail in context; the structured-query advantage databases historically offered shrinks when an LLM can answer "what changed in this patient's medication list last month" without writing SQL.

### Lower priority / diagrams

- [ ] Commission or generate three diagrams: (a) N(N-1)/2 integration pairs vs N implementations of a shared format, (b) "patient as folder, organisations as clones" distributed clone topology, (c) optional lakehouse-style stack diagram with canonical files at the bottom, derived org-level databases in the middle, applications at the top.
- [ ] Verify Zensical strict build (or enable equivalent) and ensure no broken internal links after content reshuffles.
