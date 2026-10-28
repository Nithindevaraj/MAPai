# MAPai (SAiA) — Master Architectural Specification & User Handbook
> **Symbiotic Adaptive Intelligent Architecture (SAiA) Protocol Reference Implementation**  
> *A Sovereign, Model-Agnostic Governance, Product Management & Deterministic Verification Engine for Human-AI Co-Creation.*

---

## Table of Contents
1. [Executive Summary & Value Proposition](#1-executive-summary--value-proposition)
2. [The 5 Pathologies of Generative AI Agents](#2-the-5-pathologies-of-generative-ai-agents)
3. [The Core Architectural Pillars of SAiA](#3-the-core-architectural-pillars-of-saia)
   - [3.1 The Two-Job Formalism ($\mathcal{J}_1 \perp \mathcal{J}_2$)](#31-the-two-job-formalism-mathcalj_1-perp-mathcalj_2)
   - [3.2 Continuity-Aware Workflow Orchestration (CAWO)](#32-continuity-aware-workflow-orchestration-cawo)
   - [3.3 Independent Verification & Validation (IV&V — IEEE 1012)](#33-independent-verification--validation-ivv--ieee-1012)
   - [3.4 The 7-Stage State Machine Lifecycle](#34-the-7-stage-state-machine-lifecycle)
   - [3.5 Cognitive Preimage Protection & Intent Distillation](#35-cognitive-preimage-protection--intent-distillation)
   - [3.6 Closed-Loop Perception-Action Display Engine](#36-closed-loop-perception-action-display-engine)
   - [3.7 Anonymized Experience Distillation (Heuristic Antibodies)](#37-anonymized-experience-distillation-heuristic-antibodies)
4. [MAPai as the Autonomous Technical Product Manager (AI-PM)](#4-mapai-as-the-autonomous-technical-product-manager-ai-pm)
   - [4.1 Canonical Core: The 5 Professional Jobs-to-be-Done (JTBD) Classes](#41-canonical-core-the-5-professional-jobs-to-be-done-jtbd-classes)
   - [4.2 Applied User Personas: Downstream Market Expressions](#42-applied-user-personas-downstream-market-expressions)
5. [The Symbiotic Human-AI Bridge & Co-Evolutionary Loop](#5-the-symbiotic-human-ai-bridge--co-evolutionary-loop)
6. [The Zero-Trust Screener: Complete Security & Data Privacy Barrier](#6-the-zero-trust-screener-complete-security--data-privacy-barrier)
7. [System Topology: Global OS Daemon vs. Project Workspace](#7-system-topology-global-os-daemon-vs-project-workspace)
8. [Hardware Independence & Execution Tiers (No Local Model Compulsion)](#8-hardware-independence--execution-tiers)
9. [Competitive Taxonomy & Landscape Analysis](#9-competitive-taxonomy--landscape-analysis)
10. [Installation, Setup & First-Time Configuration](#10-installation-setup--first-time-configuration)
11. [Complete CLI & Tool Reference](#11-complete-cli--tool-reference)
12. [Troubleshooting, Self-Healing & Immune Diagnostics](#12-troubleshooting-self-healing--immune-diagnostics)
13. [Open Source Governance & Contribution Protocol](#13-open-source-governance--contribution-protocol)

---

## 1. Executive Summary & Value Proposition

Modern software development with artificial intelligence is plagued by an underlying crisis: **Generative models are stochastic prediction engines, not deterministic software engineers.** When trusted with unmediated access to codebases, they generate plausible-looking but non-functional code, hallucinate fake library dependencies, forget architectural rules after a few conversational turns, and leak sensitive credentials and proprietary ideas to external cloud servers.

**MAPai** is an open-source, model-agnostic reference implementation of the **Symbiotic Adaptive Intelligent Architecture (SAiA)** protocol. Written in native systems **Rust** for deterministic performance and memory safety, MAPai functions as a **sovereign sidecar, agile product manager, and independent verification authority**.

### Key Value Highlights:
* **Zero Hollow Stubs**: Sub-millisecond AST audits eliminate placeholder mocks (`// TODO`) and unhandled errors.
* **Causal State Continuity**: Bounded state capsules ($< 1,000$ tokens) eliminate context window bloat and cascading amnesia.
* **Independent Authority**: The AI model is strictly barred from grading its own code; deliverables are quarantined until validated by native compilers and AST verifiers.
* **Closed-Loop Visual Grounding**: Prevents blind automation by capturing actual OS desktop displays, checking WCAG 2.1 relative luminance, and isolating spatial bounding boxes.
* **Zero Data Exfiltration**: Your code, secrets, and raw thoughts never leave your machine; external models only receive sanitized, abstract mathematical specifications.

---

## 2. The 5 Pathologies of Generative AI Agents

MAPai directly eliminates the five failure modes inherent to unconstrained generative AI:

```
┌────────────────────────────────────────────────────────────────────────────┐
│                  THE 5 SYSTEMIC PATHOLOGIES OF AI CODING                   │
├──────────────────────────────┬─────────────────────────────────────────────┤
│ 1. Context Collapse          │ Attention degradation across long-horizon   │
│    & Causal Amnesia          │ sessions; forgets architectural axioms.     │
├──────────────────────────────┼─────────────────────────────────────────────┤
│ 2. Hollow Completion         │ Generates syntactically plausible code      │
│    Artifacts (Fake Code)     │ containing unhandled unwraps & TODO stubs.  │
├──────────────────────────────┼─────────────────────────────────────────────┤
│ 3. Epistemic Sycophancy      │ Blindly agrees with invalid human premises  │
│    & Confirmation Drift      │ instead of asserting physical constraints.  │
├──────────────────────────────┼─────────────────────────────────────────────┤
│ 4. Authority Inversion       │ An untrusted stochastic model operates as   │
│    & Substrate Opacity       │ an unchecked execution authority on the OS. │
├──────────────────────────────┼─────────────────────────────────────────────┤
│ 5. Cognitive Preimage        │ Unsanitized leaking of private human ideas, │
│    & Telemetric Exfiltration │ credentials, and source code to cloud APIs. │
└──────────────────────────────┴─────────────────────────────────────────────┘
```

---

## 3. The Core Architectural Pillars of SAiA

```
                     THE FORMAL SAiA STATE MACHINE
                                     
                ┌────────────────────────────────────────┐
                │        Human Teleological Root         │
                │         (Cognitive Preimage)           │
                └───────────────────┬────────────────────┘
                                    │
                                    ▼
                 [Cognitive Invariant Distillation]
                                    │
                  ┌─────────────────┴─────────────────┐
                  ▼                                   ▼
        ┌───────────────────┐               ┌───────────────────┐
        │  Job 1 ($\mathcal{J}_1$)   │               │  Job 2 ($\mathcal{J}_2$)   │
        │ Functional Intent │               │ Material Physics  │
        └─────────┬─────────┘               └─────────┬─────────┘
                  │                                   │
                  └─────────────────┬─────────────────┘
                                    │
                                    ▼
       ┌────────────────────────────────────────────────────────┐
       │   Continuity-Aware Workflow Orchestration (CAWO)       │
       │     Causal State Capsule ($\mathcal{C} \in \mathcal{T}$)               │
       └────────────────────────────┬───────────────────────────┘
                                    │
            ┌───────────────────────┴───────────────────────┐
            ▼                                               ▼
┌───────────────────────┐                       ┌───────────────────────┐
│     The Mason         │                       │      The Auditor      │
│ Stochastic Generator  │ ════ Quarantined ═══► │ Deterministic AST &   │
│  (Cloud/Local Model)  │      Deliverable      │ Visual Closed-Loop    │
└───────────────────────┘                       └───────────┬───────────┘
                                                            │
                                         Passes Empirical Verification Gate?
                                                            │
                                            ┌───────────────┴───────────────┐
                                            ▼                               ▼
                                         [YES]                             [NO]
                                            │                               │
                                            ▼                               ▼
                                  [State Transition]            [Actionable Rejection &]
                                  [ Causal Ledger  ]            [Heuristic Antibody    ]
```

### 3.1 The Two-Job Formalism ($\mathcal{J}_1 \perp \mathcal{J}_2$)
Every engineering task is strictly bifurcated into two orthogonal requirement domains:
$$\mathcal{T} = \langle \mathcal{J}_1, \mathcal{J}_2 \rangle$$

* **Job 1 ($\mathcal{J}_1$ — Intent & Teleology)**:  
  The functional objective defined by the human collaborator (e.g., *"Implement an idempotent payment processing consumer"*).
* **Job 2 ($\mathcal{J}_2$ — Material & Environmental Constraints)**:  
  Non-negotiable physical constraints imposed by the execution environment (e.g., *deterministic latency envelope $< 50\mu\text{s}$, zero heap allocations in the hot loop, strict memory bounds, API contracts*).

Decoupling $\mathcal{J}_1$ from $\mathcal{J}_2$ ensures that generative models cannot alter architectural invariants to accommodate algorithmic laziness.

### 3.2 Continuity-Aware Workflow Orchestration (CAWO)
Instead of retransmitting 50,000 tokens of conversational history, MAPai maintains an append-only, content-addressed causal state ledger:
* **State Capsule (`capsule.toml`)**: Structured working memory kept strictly under **$1,000$ tokens**, capturing active stage, invariants, and blocking frictions.
* **Causal Hashing**: Every state capsule is anchored to the cryptographic SHA-256 hash of the codebase tree, ensuring tamper-proof reproducibility.

### 3.3 Independent Verification & Validation (IV&V — IEEE 1012)
MAPai strictly enforces the IEEE 1012 standard of independent authority:
$$\text{Authority}(\text{Generation}) \cap \text{Authority}(\text{Verification}) = \emptyset$$
* **The Mason**: The generative AI model (an untrusted proposal engine).
* **The Auditor**: MAPai’s native Rust engine (`crates/saia-verify`), which audits deliverables via static AST scanners and native compilers.
* **Zero Hollow Stubs**: Enforces the **Hollow Completion Rate (HCR)**:
  $$\text{HCR} = \frac{\sum (\text{Unimplemented Stubs} + \text{TODO Artifacts} + \text{Unhandled Unwraps})}{\text{KLOC Generated}} = 0$$

### 3.4 The 7-Stage State Machine Lifecycle
MAPai guides projects through an invariant-enforced 7-stage state machine (`crates/saia-orchestrator/src/stage_machine.rs`):
1. **Stage 1: Problem Intent** $\rightarrow$ Blocks advancement until Job 1 problem intent is defined.
2. **Stage 2: Architecture Constraints** $\rightarrow$ Formulates physical constraints and non-negotiables.
3. **Stage 3: Material Selection** $\rightarrow$ Dispatches `OnlineScoutVerifier` to verify external dependencies live on public registries, catching hallucinated or yanked packages before coding begins.
4. **Stage 4: Implementation** $\rightarrow$ Quarantines deliverables until `TruthScanner` confirms zero hollow stubs.
5. **Stage 5: Empirical Reality Check** $\rightarrow$ Validates physical compilation and test suites.
6. **Stage 6: Hardening Diagnostics** $\rightarrow$ Threat intelligence & CVE audit.
7. **Stage 7: Sovereign Delivery** $\rightarrow$ Final verified release.

### 3.5 Cognitive Preimage Protection & Intent Distillation
When humans brainstorm, they express personal stories, raw business secrets, and idioms (the *Cognitive Preimage*).
* **Never Literally Echo**: MAPai strips personal reflections, private credentials, and company identities from prompts.
* **Distill Invariants**: It extracts the core engineering teleology and applies it across the project while keeping the human's private musings completely local.

### 3.6 Closed-Loop Perception-Action Display Engine
For desktop and GUI software automation:
* **Sub-Millisecond Native Capture**: Captures OS display buffers (`WinSta0\Default`) in $< 1\text{ ms}$.
* **WCAG 2.1 Contrast & Luminance**: Mathematically computes relative luminance and enforces a minimum contrast ratio of $4.5:1$.
* **Empirical Verification Gate**: The AI cannot claim an operation succeeded unless the target UI element or window physically renders with verified contrast.

### 3.7 Anonymized Experience Distillation (Heuristic Antibodies)
When MAPai and the developer resolve a complex architectural struggle or subtle race condition, MAPai crystallizes the solution into an **anonymized heuristic antibody** (`h_<hash>.json`) in `.mapai/heuristics/`. This builds an open-source commons of engineering wisdom without leaking proprietary code.

---

## 4. MAPai as the Autonomous Technical Product Manager (AI-PM)

Software projects fail most often not from syntax errors, but from **poor requirements engineering, scope creep, and undefined acceptance criteria**. MAPai acts as an embedded Technical Product Manager:

```
                            THE AI-PM IN ACTION
                                      │
 ┌────────────────────────────────────┼────────────────────────────────────┐
 │ 1. Scope Formulation               │ 2. Pre-Code Intent Readback        │
 │ Locks in Problem Statement &       │ `mapai verify-intent` mediates     │
 │ Acceptance Criteria in capsule.    │ and confirms spec before coding.   │
 ├────────────────────────────────────┼────────────────────────────────────┤
 │ 3. Sprint Backlog Tracking         │ 4. Definition of Done (DoD) Gate   │
 │ Embedded checklist in capsule.toml │ Blocks release if tests or stub    │
 │ tracks multi-turn milestones.      │ scanners fail.                     │
 └────────────────────────────────────┴────────────────────────────────────┘
```

* **Requirements Alignment**: Refuses to trigger code generation until Job 1 Intent is documented.
* **Pre-Code Intent Readback (`mapai verify-intent`)**: Intercepts the request and reads back edge cases and success criteria for human sign-off.
* **Visual Cockpit (`mapai ui`)**: Provides a dark-mode browser dashboard on port 4321 for non-terminal stakeholders to track stage progression and sprint checklists.

### 4.1 Canonical Core: The 5 Professional Jobs-to-be-Done (JTBD) Classes

The **core engine of MAPai strictly adheres to professional systems engineering classification** (aligned with IEEE 12207 SDLC, Clayton Christensen’s Jobs-to-be-Done framework, and TOGAF Enterprise Architecture). Rather than categorizing by colloquial job titles, MAPai core classifies tasks by **state transformation, verification risk, and invariant domain**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│              CANONICAL MAPai PROFESSIONAL JTBD CLASSIFICATION               │
├──────────────────────────────┬──────────────────────────────────────────────┤
│ 1. Greenfield Synthesis      │ 0-to-1 architecture, requirement scoping,    │
│    (`greenfield-synthesis`)  │ and end-to-end multi-modal implementation.   │
├──────────────────────────────┼──────────────────────────────────────────────┤
│ 2. Brownfield Modernization  │ Legacy migrations, framework upgrades,       │
│    (`brownfield-modernization│ stack porting, and technical debt reduction. │
├──────────────────────────────┼──────────────────────────────────────────────┤
│ 3. Independent Verification  │ Deterministic AST auditing (`TruthScanner`), │
│    (`independent-verification│ CVE/security scans, and WCAG contrast gates. │
├──────────────────────────────┼──────────────────────────────────────────────┤
│ 4. Heterogeneous Automation  │ Closed-loop OS desktop control, COM APIs,    │
│    (`heterogeneous-automation│ headless browser extraction, and RPA.        │
├──────────────────────────────┼──────────────────────────────────────────────┤
│ 5. Cognitive Distillation    │ Parsing dense technical specs (PDFs),        │
│    (`cognitive-distillation`)│ competitive auditing (`OmniScout`), and ADRs.│
└──────────────────────────────┴──────────────────────────────────────────────┘
```

#### The 5 Professional Classes in Depth:

1. **Greenfield Synthesis (`ProfessionalJobClass::GreenfieldSynthesis`)**
   * *Transformation Goal:* Transform unbounded natural language intent ($\mathcal{J}_1$) into verified, executable architecture from ground zero.
   * *Core Mechanism:* Locks initial problem statement and acceptance criteria into `capsule.toml`, sets language/platform invariants ($\mathcal{J}_2$), and verifies scaffold generation.

2. **Brownfield Modernization (`ProfessionalJobClass::BrownfieldModernization`)**
   * *Transformation Goal:* Safely alter or migrate existing codebases and pipelines without regressing established behavior.
   * *Core Mechanism:* Anchors before/after state to Git SHA-256 tree hashes; enforces atomic rollback checkpoints in `ContinuityJournal`.

3. **Independent Verification & Assurance (`ProfessionalJobClass::IndependentVerification`)**
   * *Transformation Goal:* Provide empirical, non-stochastic validation of deliverables independent of the generative model.
   * *Core Mechanism:* `TruthScanner` sub-millisecond AST audit (Zero Hollow Stubs), native compiler checks, and WCAG 2.1 contrast enforcement ($4.5:1$ minimum).

4. **Heterogeneous Tool Orchestration (`ProfessionalJobClass::HeterogeneousAutomation`)**
   * *Transformation Goal:* Bridge autonomous reasoning into external legacy runtimes, desktop applications, and web services.
   * *Core Mechanism:* Closed-loop desktop frame perception (`WinSta0\Default`), native COM interop (e.g. AutoCAD, Excel), and headless browser pipelines.

5. **Cognitive Distillation & Knowledge Engineering (`ProfessionalJobClass::CognitiveDistillation`)**
   * *Transformation Goal:* Ingest high-entropy external information and distill it into structured invariants without leaking proprietary human musings.
   * *Core Mechanism:* Cognitive Preimage Protection, deep PDF document parsing, live registry auditing (`OmniScout`), and Architecture Decision Records (ADRs).

---

### 4.2 Applied User Personas: Downstream Market Expressions

While the **MAPai core strictly operates on the 5 Professional JTBD Classes**, downstream users and marketing channels experience these capabilities through **8 Applied Domain Personas**:

```
CANONICAL CORE (Professional JTBD)                APPLIED DOMAIN PERSONAS
┌─────────────────────────────────┐               ┌────────────────────────────────┐
│ 1. Greenfield Synthesis         ├──────────────►│ 🛠️ The Builder (Full-Stack/CLI)│
│                                 ├──────────────►│ 📐 The Architect (CAD/BIM)     │
│                                 ├──────────────►│ 🎮 The Game Maker (Simulation) │
├─────────────────────────────────┤               ├────────────────────────────────┤
│ 2. Brownfield Modernization     ├──────────────►│ 🛡️ The Auditor (Refactoring)   │
├─────────────────────────────────┤               ├────────────────────────────────┤
│ 3. Independent Verification     ├──────────────►│ 🛡️ The Auditor (Zero-Stubs/CVE)│
│                                 ├──────────────►│ 🎨 The Designer (WCAG Contrast)│
├─────────────────────────────────┤               ├────────────────────────────────┤
│ 4. Heterogeneous Automation     ├──────────────►│ ⚙️ The Operator (Desktop RPA)  │
│                                 ├──────────────►│ 🌐 The Scout (Web Scraping)    │
├─────────────────────────────────┤               ├────────────────────────────────┤
│ 5. Cognitive Distillation       ├──────────────►│ 📚 The Scholar (Deep Research) │
│                                 ├──────────────►│ 🌐 The Scout (Registry Intel)  │
└─────────────────────────────────┘               └────────────────────────────────┘
```

1. **Software & App Engineering (Web, Mobile, Systems)**
   * *The Struggle:* Traditional AI agents hallucinate third-party APIs, fail to wire state management properly, and leave `// TODO` stubs in async callbacks.
   * *The MAPai Advantage:* The AI-PM generates rigorous acceptance criteria, while `TruthScanner` ensures zero hollow implementations before compilation. Supports Next.js, React, Flutter, Android (Jetpack Compose), iOS (Swift), and native Rust/Go backends.

2. **Spatial, CAD & Physical Architecture**
   * *The Struggle:* Generative models struggle with spatial coherence, geometric dimension constraints, and domain-specific formats (DWG, DXF, IFC).
   * *The MAPai Advantage:* Procedurally generates parametric geometries, exports clean DXF/DWG vector scripts, controls desktop drafting suites (e.g. AutoCAD via native COM automation), and mathematically validates spatial bounding limits.

3. **UI/UX Design, Visual Synthesis & Media Pipelines**
   * *The Struggle:* Visual agents produce unstyled or inaccessible interfaces with broken color palettes and zero adherence to accessibility standards.
   * *The MAPai Advantage:* Employs native desktop frame perception (`WinSta0\Default`) to guarantee WCAG 2.1 relative luminance ($4.5:1$ contrast) and generate production-ready Tailwind/CSS design systems and automated Remotion/FFmpeg video rendering pipelines.

4. **Interactive Simulation & Game Development**
   * *The Struggle:* Game development requires complex state persistence, frame-budget physics, and procedural level generation algorithms that fail in unconstrained chat models.
   * *The MAPai Advantage:* Enforces immutable state machines for gameplay logic, audits memory allocations in hot loops, and orchestrates procedural generation in engines like Godot (GDScript/Rust), Unity, Bevy, and Three.js.

5. **Data Harvesting, Web Scraping & Market Intelligence**
   * *The Struggle:* Scrapers break on modern SPAs, trigger anti-bot measures, or ingest unvalidated schemas.
   * *The MAPai Advantage:* Couples resilient headless browsing with live schema validation. The integrated `OmniScout` verification engine continuously validates external package registries (crates.io, PyPI, npm) to catch deprecations or yanked libraries before code runs.

6. **Deep Research, Document Intelligence & Benchmarking**
   * *The Struggle:* Chat assistants summarize surface-level points but drop critical technical footnotes, fail to extract complex tables, and invent fictitious citations.
   * *The MAPai Advantage:* Deeply parses complex PDF reports, OECD capability metrics, and institutional standards. Converts dense unstructured technical reports into structured Architecture Decision Records (ADRs) with mathematical rigor.

7. **Desktop Automation & Robotic Process Automation (RPA)**
   * *The Struggle:* Macro recorders fail when screen resolutions shift, windows occlude, or desktop dialogs take 200ms longer to render.
   * *The MAPai Advantage:* Closed-loop visual perception audits the user's interactive desktop, verifying that target windows are active and visible before dispatching surgical input sequences.

8. **Codebase Modernization, Refactoring & Security Auditing**
   * *The Struggle:* Refactoring large legacy codebases introduces silent regressions and breaks backward compatibility.
   * *The MAPai Advantage:* Causal checkpoints anchored to Git SHA-256 allow risk-free atomic rollbacks, while deterministic AST scanners prevent dependency vulnerabilities and remove dead code paths.

---

## 5. The Symbiotic Human-AI Bridge & Co-Evolutionary Loop

MAPai establishes a bilateral feedback loop that elevates both the AI model and the human developer:

### How MAPai Improves the AI Model:
1. **Pristine Attention**: Replaces 50k tokens of conversational noise with $< 1,000$ tokens of pure invariants, eliminating attention degradation.
2. **Empirical Grounding**: Gives the model sensory feedback through compiler errors, AST validation reports, and display checks.
3. **Hallucination Immunization**: Catches fake libraries live on package registries before they enter the build.
4. **Heuristic Antibodies**: Prevents the model from repeating the same architectural mistake twice.

### How MAPai Improves the Human Engineer:
1. **Cognitive Elevation**: Transitions the developer from a syntax typist to a **Systems Architect** who designs invariants and specifications.
2. **Fearless Refactoring**: Causal checkpoints anchored to Git SHA-256 allow one-command rollbacks, encouraging ambitious refactors.
3. **Inoculation of Best Practices**: Continuous exposure to zero-stub standards, strict typing, and contrast audits sharpens the developer's engineering discipline.

---

## 6. The Zero-Trust Screener: Complete Security & Data Privacy Barrier

The AI model is treated as an **untrusted entity**. It is never given raw access to your machine.

```
                       MAPai ZERO-TRUST SECURITY RINGS
                                  
                       ┌─────────────────────────┐
                       │  RING 3: UNTRUSTED AI   │ ◄── AI Model Lives Here
                       │  (In-Memory Sandbox)    │
                       └────────────┬────────────┘
                                    │
                       ┌────────────▼────────────┐
                       │  RING 2: MAPai AUDITOR  │ ◄── AST Verification Gate
                       │  (Zero-Stub Quarantine) │
                       └────────────┬────────────┘
                                    │
                       ┌────────────▼────────────┐
                       │  RING 1: USER WORKSPACE │ ◄── Current Project Only
                       │  (Surgical AST Diffs)   │
                       └────────────┬────────────┘
                                    │
                       ┌────────────▼────────────┐
                       │  RING 0: HOST OS & SEC  │ ◄── Strict Air-Gap
                       │  (System Root, SSH, DB) │     (AI NEVER REACHES HERE)
                       └─────────────────────────┘
```

1. **Zero Direct File Access**: The model cannot traverse outside the project root. System directories (`C:\Windows`, `/etc`, `~/.ssh`) are physically air-gapped.
2. **Zero Shell Execution**: The model can only emit command proposals; MAPai's native Rust engine screens and executes commands only with human authorization.
3. **Automatic Secret Redaction**: High-entropy strings, API keys, passwords, and `.env` files are masked with `[MAPAI_REDACTED_SECRET]` before payloads reach the model.
4. **Virtual Shadow Workspace**: AI patches are applied to an in-memory shadow buffer and verified against compilers *before* touching physical disk files.

---

## 7. System Topology: Global OS Daemon vs. Project Workspace

MAPai operates across two distinct, complementary scopes:

* **Global Operating System Scope (`~/.mapai/`)**:
  - Installed via `mapai fti` (First-Time Installer).
  - Detects hardware microarchitecture, CPU cores, and toolchains.
  - Resolves Windows path spaces using native 8.3 short-name aliasing (`%~sI`), preventing unquoted-path crashes.
  - Hosts the global Model Context Protocol (MCP) server (`mapai mcp`) for universal editor connectivity.
  - Maintains the global Heuristic Antibody repository.
* **Project Workspace Scope (`./.mapai/`)**:
  - Initialized via `mapai init`.
  - Maintains the isolated Project State Capsule (`capsule.toml`) and Causal Journal (`journal.json`).
  - Strict sandboxing ensures zero data bleed-through between different projects.

---

## 8. Hardware Independence & Execution Tiers

> [!NOTE]
> **Running a local model is completely optional.** MAPai functions at full capability on any standard laptop using cloud APIs.

| Tier | Hardware Requirements | Execution Strategy | Best For |
| :--- | :--- | :--- | :--- |
| **Tier 1: Cloud-First** *(Default)* | Any standard PC (4–8 GB RAM, CPU only) | Cloud APIs (Gemini, Claude, OpenAI, Groq) + native Rust verification | Zero GPU required; instant setup, zero fan noise |
| **Tier 2: Hybrid** | Modern laptop (8–16 GB RAM) | Local CPU embeddings for code search + Cloud models for reasoning | Fast local indexing with frontier model power |
| **Tier 3: 100% Local** *(Optional)* | 16 GB+ RAM / Dedicated GPU (6–16 GB+ VRAM) | Self-hosted local LLMs (via Ollama, vLLM, LM Studio) | Air-gapped, offline, highly sensitive enterprise code |

---

## 9. Competitive Taxonomy & Landscape Analysis

| Architectural Dimension | Aider | Claude Code | OpenHands | Devin | **MAPai (SAiA)** |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Language Foundation** | Python | TypeScript | Python / React | Proprietary Cloud | **Native Rust (Sub-millisecond latency)** |
| **Verification Gate** | Post-commit lint/test | Prompted self-check | Docker test run | Autonomous test loop | **Independent IEEE 1012 IV&V (The Auditor)** |
| **Stub Elimination (HCR)** | Indirect (via linter) | None (Prompt only) | None | None | **Deterministic AST Scanner (`TruthScanner`)** |
| **Supply Chain Check** | None | None | None | None | **Live Dependency Registry Scouting (`OnlineScout`)** |
| **Context Overhead** | Tree-sitter RepoMap | Ripgrep scans | Vector search / memory | Full container dump | **CAWO State Capsule ($< 1,000$ tokens)** |
| **Visual / Screen Gate** | None | None | Browser screenshots | Cloud browser canvas | **Native OS Display Capture & WCAG 2.1 Audit** |
| **Browsing Engine** | None | Raw curl / URLs | Playwright in Docker | Sandboxed Chromium | **Tiered Hybrid (Headless $\rightarrow$ CDP port 9222)** |
| **Privacy & Sovereignty** | Local git commits | Cloud-routed | Local container | Cloud VM (Third-party) | **Zero Telemetry + Heuristic Antibodies** |
| **Ecosystem Extensibility**| Python scripts | MCP | Agent SDK / ACP | Closed API / Slack | **Native CLI, Embedded Web UI & MCP Server** |

---

## 10. Installation, Setup & First-Time Configuration

### Automated Installation (One Command)
Compiles release binaries, adds `mapai` to user PATH, configures Windows space-safe path aliasing, and registers the MCP server:

**Windows (PowerShell):**
```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\setup.ps1
```

**Linux / macOS:**
```bash
bash ./scripts/setup.sh
```

### Manual Compilation from Source
Requires the standard Rust toolchain (`rustup.rs`):
```bash
cargo build --release -p saia-cli
```
The optimized native binary is generated at `./target/release/mapai.exe` (or `./target/release/mapai`).

---

## 11. Complete CLI & Tool Reference

```bash
# Project Initialization & Lifecycle
mapai init --name "MyProject"     # Initialize sovereign .mapai environment
mapai fti                         # First-time interactive installer & device setup
mapai status                      # View active pipeline stage & verification metrics
mapai advance                     # Advance to next pipeline stage through verification
mapai migrate                     # Migrate project capsule to latest protocol schema
mapai guide                       # View adaptive guidance for active stage

# Verification & Code Quality
mapai check                       # Audit codebase for hollow stubs (< 10ms for 50k LOC)
mapai verify-intent -o "Goal"     # Run closed-loop intent readback before coding
mapai context                     # Output compact cognitive context (< 1,000 tokens)

# Causal Continuity & Audit
mapai checkpoint -i "Feature X"   # Record immutable checkpoint anchored to Git SHA-256
mapai log                         # Display chronological causal audit journal

# Autonomous Intelligence & Browsing
mapai scout                       # Run 7-vector autonomous internet intelligence scan
mapai browse "https://docs.rs"    # Tiered browser (Headless with auto-escalation to CDP)
mapai browse --real "url"         # Force navigation via active browser (CDP:9222)

# User Interfaces & IDE Integration
mapai ui --port 4321              # Launch interactive dark-mode Web Studio
mapai mcp                         # Launch Model Context Protocol (MCP) server

# Native OS Screen & Visual Gate
mapai screen capture -w "App"     # Capture window frame buffer to BMP
mapai screen inspect -p "snap.bmp"# Compute luminance, background & WCAG contrast
mapai screen verify -w "App" \    # Enforce empirical visual verification gate
  -t "Login" -c "black" --min-contrast 4.5
```

---

## 12. Troubleshooting, Self-Healing & Immune Diagnostics

### The Circuit Breaker & Fallback Runner
If an external IDE plugin or auxiliary hook crashes (e.g., path quoting or module failures), MAPai's native engine activates its **Micro-Circuit Breaker**:
* **Immediate Quarantine**: The failing hook is flagged as `QUARANTINED` for the session.
* **Fail-Open Policy**: Auxiliary telemetry or linter crashes **never** block your core execution.
* **Direct OS Fallback**: MAPai engages its native OS subprocess runner, bypassing compromised middleware completely.

### The `VersionAdaptiveEngine`
If your project capsule becomes outdated due to a CLI upgrade, running:
```bash
mapai migrate
```
automatically migrates your schema, preserving all existing audit records and invariants without data loss.

---

## 13. Open Source Governance & Contribution Protocol

MAPai is dual-licensed under **MIT OR Apache-2.0** (Rust RFC 1686 standard):
* **MIT**: Maximizes permissive integration into open-source, academic, and legacy GPLv2 codebases.
* **Apache 2.0**: Protects contributors and enterprise users through explicit, legally binding patent grants and retaliation immunity.

### How to Contribute Without Sharing Private Code:
1. **Heuristic Antibodies (`.mapai/heuristics/`)**: Submit anonymized struggle patterns to help models globally avoid recurring bugs.
2. **Grammar & AST Scanners (`crates/saia-verify`)**: Extend static stub scanning to additional languages (C++, TypeScript, Python, Go).
3. **Model Context Protocol (MCP) Connectors**: Build community adapters for databases, cloud infrastructure, and task trackers.

---
*MAPai (SAiA) — Built for sovereignty, driven by verification, and governed by human intent.*
