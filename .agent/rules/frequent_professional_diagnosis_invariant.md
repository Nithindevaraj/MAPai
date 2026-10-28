---
description: Mandatory and frequent professional systems engineering diagnosis across Clippy, monorepo hygiene, lockfile determinism, Git provenance, and truth-to-materials.
globs: "*"
alwaysApply: true
---

# 🛡️ Frequent Professional Systems Engineering Diagnostic Invariant

To ensure MAPai maintains sovereign, enterprise-grade engineering integrity, the following diagnostic protocol must be executed frequently and automatically across development cycles:

## 1. Automated Health Invariants
1. **Zero Compiler & Clippy Lints**:
   - `cargo clippy --workspace --all-targets -- -D warnings` must strictly succeed with zero warnings.
2. **Deterministic Lockfile Tracking**:
   - `Cargo.lock` must remain tracked in version control (never ignored in `.gitignore`) for all application and CLI crates.
3. **Monorepo Root Hygiene**:
   - No loose scripts (`.ps1`, `.py`), benchmark PDFs, or ad-hoc generated outputs in the workspace root alongside `Cargo.toml`.
   - Ad-hoc scripts must be organized in `examples/` or `scripts/`, and documents in `docs/research/`.
4. **Git Provenance & Causal Ledgers**:
   - Keep the workspace tree synchronized with the CAWO append-only ledger and remote origin.
5. **Truth-to-Materials Gate**:
   - Codebases must maintain `0.00` stubs/KLOC (`TruthScanner`), strictly eliminating hollow completions and unhandled unwraps.

## 2. Command Trigger
Whenever undertaking code alterations, refactorings, or project state transitions, execute:
```bash
mapai diagnose
```
to verify that all five foundational systems engineering invariants remain satisfied.
