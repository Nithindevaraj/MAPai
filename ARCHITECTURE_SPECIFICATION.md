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

#### 2.1.1 Canonical Professional Jobs-to-be-Done (JTBD) Classification
MAPai core strictly categorizes Job 1 Intent according to five foundational systems engineering transformation archetypes (aligned with IEEE 12207 and Enterprise SDLC standards):
1. **Greenfield Synthesis (`greenfield-synthesis`):** 0-to-1 system architecture, specification formulation, and complete multi-modal implementation from baseline requirements.
2. **Brownfield Modernization (`brownfield-modernization`):** Invariant-preserving refactoring, legacy framework migration, dependency modernization, and technical debt eradication.
3. **Independent Verification & Assurance (`independent-verification`):** Deterministic AST truth auditing (`TruthScanner`), compiler-enforced invariant verification, security compliance, and WCAG contrast gates.
4. **Heterogeneous Tool Orchestration (`heterogeneous-automation`):** Closed-loop external system integration, native OS GUI perception/action (`WinSta0\Default`), and desktop API automation (COM, headless browsers).
5. **Cognitive Distillation & Knowledge Engineering (`cognitive-distillation`):** SOTA benchmark synthesis, deep unstructured document ingestion, live registry intelligence (`OmniScout`), and Architecture Decision Record (ADR) extraction under Cognitive Preimage Protection.

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

### 2.5 Intent Distillation Over Literal Echoing (Cognitive Preimage Protection)
Human collaborators naturally communicate through personal reflections, idioms, philosophies, and real-life lessons. Under the SAiA protocol:
- **Never Literally Echo:** MAPai does not blindly copy the human's personal reflections, quotes, creeds, or conversational stories into public deliverables, UI copy, or source code.
- **Distill Key Invariants:** MAPai extracts the underlying engineering teleology, architectural standards, and physical constraints (Job 1 / Job 2) embodied in the human's lesson.
- **Apply Human Intent Ubiquitously:** The distilled takeaways are applied systematically across all projects MAPai manages, keeping the human's private reflections protected within their sovereign domain.

### 2.6 Global Screen Analyser & Closed-Loop Visual IV&V
Blind execution of GUI automation commands (sending virtual keystrokes or mouse clicks) without visual feedback induces severe failure modes: invisible text due to palette/theme desynchronization, uncommitted floating input boxes, and window occlusion. Under the SAiA protocol:
- **Zero-Blind Execution:** Every GUI and desktop operation is governed by a closed-loop perception-action cycle.
- **Physical Screen Perception:** MAPai captures the user's interactive desktop (`WinSta0\Default`) at native sub-millisecond speed, computes WCAG 2.1 relative luminance and contrast ratios, and isolates content bounding boxes.
- **Deterministic Visual Gate:** Deliverables are immediately rejected with actionable diagnostic remediation if content fails to physically manifest, if text is visually illegible (contrast ratio $< 3.0:1$), or if color requirements diverge from intent.

---

## 3. Crate Architecture & System Boundaries

```
crates/
├── saia-core/          # Pure protocol definitions, state capsules, and CAWO models
├── saia-verify/        # Deterministic AST scanners, stub detectors, and compiler bridges
├── saia-orchestrator/  # Task state machine, model router, and policy enforcement
└── saia-cli/           # Sovereign native executable and Model Context Protocol (MCP) server
```
