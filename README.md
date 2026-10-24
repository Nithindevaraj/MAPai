# MAPai: Built on the SAiA Protocol

> **Symbiotic Adaptive Intelligent Architecture (SAiA)**  
> *A Sovereign, Model-Agnostic Governance & Verification Engine for Human-AI Co-Creation*

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Verification](https://img.shields.io/badge/Truth--to--Materials-Zero%20Hollow%20Stubs-green.svg)](crates/saia-verify)
[![Protocol](https://img.shields.io/badge/Protocol-SAiA%20v0.1.0-purple.svg)](ARCHITECTURE_SPECIFICATION.md)

---

## 1. Abstract & Motivation

Contemporary autonomous language agents and generative AI developer tools exhibit systemic failure modes during long-horizon engineering tasks:
- **Context Loss & Attention Degradation:** Cascading amnesia over extended sessions.
- **Hollow Completions:** Syntactically plausible code concealing unhandled exceptions, unverified mocks, and `TODO` stubs.
- **Confirmation Bias & Sycophancy:** Inability to provide epistemic dissent against flawed user premises.
- **Opaque Proprietary Telemetry:** Invasive tracking and extraction of private source code and credentials.

**MAPai** is an open-source, model-agnostic reference implementation of the **Symbiotic Adaptive Intelligent Architecture (SAiA)** protocol. Written in native Rust for deterministic performance and memory safety, MAPai functions as a **zero-invasive sidecar** that decouples human intent from substrate physics, enforces continuity via append-only causal state ledgers, and guarantees deliverable integrity through independent, deterministic verification gates.

---

## 2. Core Architectural Pillars

```
┌────────────────────────────────────────────────────────────────────────┐
│                        MAPai - SAiA CORE ENGINE                        │
├───────────────────────────────────┬────────────────────────────────────┤
│ 1. TWO-JOB SPECIFICATION MODEL    │ 2. CAWO CAUSAL LEDGER              │
│    • Job 1: Human Intent (Goal)   │    • Continuity-Aware Orchestrator │
│    • Job 2: Material Constraints  │    • Content-addressed state nodes │
├───────────────────────────────────┼────────────────────────────────────┤
│ 3. TRUTH-TO-MATERIALS VERIFIER    │ 4. LOCAL PRIVACY GUARDIAN          │
│    • Zero Hollow Stubs (`TODO`)   │    • Zero telemetry extraction     │
│    • Sub-millisecond AST audit    │    • Independent policy & sandbox  │
└───────────────────────────────────┴────────────────────────────────────┘
```

1. **The Two-Job Model:** Formally isolates functional intent (**Job 1**) from system and medium constraints (**Job 2**: memory bounds, latency budgets, hardware physics).
2. **Continuity-Aware Workflow Orchestration (CAWO):** Replaces raw conversational transcripts with high-density, structured state capsules (`capsule.toml`) to prevent context drift.
3. **Independent Verification & Validation (IV&V):** Adheres to IEEE 1012 standards. The generative model cannot grade its own homework; deliverables must pass deterministic AST scanning and compiler gates.
4. **Anonymized Experience Distillation:** Crystallizes algorithmic breakthroughs into discrete, content-addressed heuristic antibodies without leaking private code or PII.

---

## 3. Workspace Architecture

The system is organized as a unified, high-performance Cargo workspace:

* **[`crates/saia-core`](crates/saia-core):** Protocol definitions, state capsule schema, and CAWO models.
* **[`crates/saia-verify`](crates/saia-verify):** Deterministic multi-language AST scanner and Hollow Completion Rate (HCR) engine.
* **[`crates/saia-orchestrator`](crates/saia-orchestrator):** 7-stage state machine runner, cost-optimal model router, and closed-loop intent readback mediator.
* **[`crates/saia-cli`](crates/saia-cli):** Standalone native binary (`mapai`) and Model Context Protocol (MCP) server.

---

## 4. Quick Start (Zero-Dependency Binary)

### Installation
Download the pre-compiled native binary for your platform, or build from source:

```bash
cargo build --release -p saia-cli
```

### Basic Usage

#### 1. Initialize a Sovereign Project
Drop MAPai into any repository without touching your production code:
```bash
mapai init --name "MyProject"
```
*Generates `.mapai/capsule.toml` tracking active project invariants and constraints.*

#### 2. Audit Code Craft (Zero Hollow Stubs)
Scan your entire codebase at native speed (< 10ms for 50,000 LOC):
```bash
mapai check
```

#### 3. Closed-Loop Intent Verification
Verify that the AI understands human intent and non-negotiables before writing code:
```bash
mapai verify-intent --objective "Build a bit-perfect audio pipeline without allocations in the hot loop"
```

#### 4. Model Context Protocol (MCP) Server Mode
Expose MAPai directly to Claude Desktop, Cursor, or Antigravity via standard JSON-RPC over stdio:
```bash
mapai mcp
```

---

## 5. Scientific Metrics & Evaluation

MAPai formalizes code generation reliability through rigorous, quantitative metrics:

$$\text{Hollow Completion Rate (HCR)} = \frac{\text{Unhandled TODOs + Placeholder Stubs}}{\text{Total KLOC Generated}}$$

$$\text{Information Transfer Failure Rate (ITFR)} = \frac{\text{Failures due to stale / dropped constraints}}{\text{Total Task Executions}}$$

---

## 6. Philosophy & Lineage

MAPai is grounded in the foundational conviction that artificial intelligence must remain an empowering symbiont to humanity rather than an opaque corporate monopoly.

> *"Yaadhum Oore Yaavarum Kelir"*  
> *(To us all towns are our hometown, and all humans our kin)*

---

## 7. License

Licensed under either of:
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
