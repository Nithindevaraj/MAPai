# MAPai: A Reference Implementation of the Symbiotic Adaptive Intelligent Architecture (SAiA) Protocol

## Abstract
Contemporary generative artificial intelligence systems and autonomous coding agents exhibit severe failure modes during long-horizon software engineering tasks: context drift, silent omission of non-functional constraints, unverified completion claims (hollow stub implementations), and excessive supervisory overhead. Furthermore, proprietary model-centric orchestration frameworks induce platform lock-in and opaque data extraction risks. 

This paper presents **MAPai**, an open-source, model-agnostic reference implementation of the **Symbiotic Adaptive Intelligent Architecture (SAiA)** protocol. Written in Rust for deterministic performance and memory safety, MAPai decouples human intent from execution substrate constraints through a two-job specification model, enforces continuity via append-only causal state checkpoints, and guarantees deliverable integrity through independent, deterministic verification gates (Abstract Syntax Tree stub auditing and compiler-grounded verification).

---

## 1. Introduction and Problem Statement
Autonomous language agent architectures often optimize for conversational fluency rather than empirical correctness. In complex engineering projects, this divergence manifests as five primary pathologies:

1. **Context Window Degradation & Amnesia:** Inability to retain foundational architectural constraints across multi-turn sessions without exponential token consumption.
2. **Hollow Completion Claims:** Emission of syntactically plausible code that conceals unhandled runtime exceptions, placeholder mocks (`TODO`), and stub functions.
3. **Sycophantic Agreement & Confirmation Bias:** Absence of epistemic dissent, wherein the agent ratifies flawed user assumptions or engages in mutual consensus around invalid designs.
4. **Opaque Authority & Dependency Risk:** Inversion of sovereignty where the AI tool acts as an unverified execution authority rather than a governed sidecar.
5. **Data Extraction & Privacy Vulnerabilities:** Leakage of proprietary code, credentials, and personally identifiable information (PII) to remote inference endpoints.

---

## 2. Core Architectural Pillars

### 2.1 The Two-Job Specification Model
Every engineering task is formally partitioned into two orthogonal requirement domains:
- **Job 1 (Intent & Teleology):** The functional objective defined by the human collaborator.
- **Job 2 (Material & Environmental Constraints):** Non-negotiable physical constraints (target architecture, memory budgets, zero-allocation requirements, API contracts).

### 2.2 Continuity-Aware Workflow Orchestration (CAWO)
Instead of retransmitting full conversation transcripts, MAPai maintains an append-only, content-addressed causal state ledger:
- **State Capsule (`capsule.toml`):** Structured working memory ($< 1,000$ tokens) tracking active stage, invariants, and blocking frictions.
- **Causal Checkpoints:** Immutable state snapshots anchored to the cryptographic SHA-256 hash of the codebase tree.

### 2.3 Independent Verification & Validation (IV&V)
Adhering to IEEE 1012 standards, the generation authority (The Mason) is strictly isolated from the verification authority (The Auditor):
- The AI model cannot approve its own deliverables.
- Deliverables are quarantined until validated by deterministic static AST scanners and native compilers.

### 2.4 Anonymized Experience Distillation
Rather than collecting user telemetry or private code, MAPai crystallizes algorithmic struggle patterns into discrete, content-addressed heuristic antibodies (`h_<hash>.json`), establishing an open-source commons of collective engineering wisdom.

---

## 3. Crate Architecture & System Boundaries

```
crates/
├── saia-core/          # Pure protocol definitions, state capsules, and CAWO models
├── saia-verify/        # Deterministic AST scanners, stub detectors, and compiler bridges
├── saia-orchestrator/  # Task state machine, model router, and policy enforcement
└── saia-cli/           # Sovereign native executable and Model Context Protocol (MCP) server
```
