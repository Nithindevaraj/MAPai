# Make Anything Process with AI (MAPai)

> **The Sovereign Harness for Autonomous AI Agents**  
> *"Models generate code. MAPai harnesses the swarm."*  
> A Native Systems Rust Governance Engine, Swarm Containment Perimeter & Deterministic Verification Authority for Agentic Coding.

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Verification](https://img.shields.io/badge/Truth--to--Materials-Zero%20Hollow%20Stubs-green.svg)](crates/saia-verify)
[![Protocol](https://img.shields.io/badge/Protocol-SAiA%20v1.0.1-purple.svg)](ARCHITECTURE_SPECIFICATION.md)
[![Release](https://img.shields.io/badge/Release-v1.0.21-blue.svg)](CHANGELOG.md)
[![Docs](https://img.shields.io/badge/Documentation-Master%20Handbook-blue.svg)](DOCUMENTATION.md)
[![Tests](https://img.shields.io/badge/Tests-46%20Passed%20(100%25)-brightgreen.svg)](crates/)

---

## ⚡ What is MAPai?

Modern coding agents are powerful but unconstrained: they clobber files concurrently, leave hollow `// TODO` stubs, execute destructive shell commands, and grade their own work.

**MAPai is a native Rust governance harness that sits as a layers between autonomous agents and you:**

1. 🔒 **Transactional Agent Containment (Two-Phase Commit)**: Agents operate in isolated virtual overlays. Mutations are held in-memory and only flushed to disk if all acceptance tests and linters pass atomically.
2. 🛡️ **Blast-Radius & Secret Shield**: Strict 4-tier risk classification blocks catastrophic filesystem operations (`~`, `/`, `.ssh`, `.env`) and automatically scrubs credentials before execution.
3. 📐 **AI Product Manager Constraint Gates**: Enforces PMI/PMP systems engineering disciplines, binding agents to formal Work Breakdown Structures (WBS) and Acceptance Criteria before generation begins.
4. 🌿 **Zero-Copy Swarm Orchestration**: Spawns concurrent subagents across shared memory slices (`Arc<Vec<Turn>>`), eliminating cross-agent file collisions and slashing token consumption by 60–80%.
5. 🔍 **Deterministic IV&V Authority (`TruthScanner`)**: Audits deliverables in $< 10\text{ms}$ with zero-tolerance for mock stubs, hallucinated crates, or unhandled failures.
6. 🧠 **0.01ms SQLite FTS5 Cognitive Memory**: Sub-millisecond cross-session memory recall powered by embedded SQLite full-text search.
7. 🎛️ **Dedicated Native Web Cockpit (`mapai ui`)**: An embedded browser studio serving live 2PC diff inspections, subagent trees, and reality monitors directly from a single compiled binary (no Node.js/npm runtime required).
8. 🔌 **Universal ACP & MCP Protocols**: Native support for both **Agent Communication Protocol (ACP)** and **Model Context Protocol (MCP)** for Cursor, Zed, Antigravity, and Claude Desktop.

---

## 📊 Quad-Agent Empirical Benchmark: MAPai vs. SOTA

We conducted exhaustive, empirical comparative benchmarks evaluating **MAPai**, **DeepSeek Harness (`dsh`)**, **JCODE**, and **Hermes Agent** across identical complex software tasks:

| Performance & Safety Metric | DeepSeek Harness (`dsh`) | JCODE | Hermes Agent | **MAPai (SAiA)** |
| :--- | :--- | :--- | :--- | :--- |
| **Engine Runtime** | Node.js (TypeScript) | Native Rust | Python (FastAPI) | **Native Systems Rust (MSVC/LLVM)** |
| **Per-Agent Memory (RAM)** | `45.0 MB – 85.0 MB+` | `2.60 MB` | `80.0 MB – 120.0 MB+` | **`2.52 MB` (Lightest Footprint)** |
| **Hollow Completion Rate (HCR)** | 14.2% (Unverified stubs) | 4.8% | 11.5% | **0.00% (Zero Hollow Code)** |
| **Swarm File Safety** | None (Direct disk edits) | ReadSet tracking | Ephemeral Worktrees | **ReadSet Tracker + In-Memory 2PC** |
| **Context Forking Efficiency** | Full prompt retransmission | Process duplicate | Full context dump | **Zero-Copy `Arc` (68.4% Token Savings)** |
| **Command Risk Perimeter** | Basic tool prompts | 4-tier risk gate | Preflight checks | **Deterministic Blast-Radius Guard** |
| **Cross-Session Memory** | File logs | Vector embeddings | Plain text | **0.01ms SQLite FTS5 (BM25 Ranked)** |
| **Skill Synthesis Standard** | Custom format | Tool JSON | Agent skills | **Standardized `agentskills.io`** |
| **Web UI Architecture** | Node.js / Vite SPA (:4173) | None (TUI only) | FastAPI / React 19 (:9119) | **Embedded Rust Binary GUI (:4321)** |
| **Universal IDE Protocol** | Proprietary CLI | Headless IPC | ACP Adapter | **Dual MCP + ACP (`mapai acp`)** |

> *Full PDF, HTML, and high-resolution chart benchmark deliverables are available in [`docs/benchmarks/`](docs/benchmarks/).*

---

## 🎯 What Can You Build with MAPai? (8 Natural Job Archetypes)

MAPai adapts to any engineering or creative discipline, enforcing rigorous, verified execution without human babysitting:

| Archetype | Natural User Domain | What MAPai Delivers |
| :--- | :--- | :--- |
| 🛠️ **The Builder** | **Full-Stack Software & Mobile** | Web apps (Next.js/React), native mobile (Compose/Flutter), microservices, and memory-safe Rust CLI tools with zero hollow stubs. |
| 📐 **The Architect** | **Spatial & CAD Modeling** | Procedural 2D/3D floor plans, architectural blueprints, AutoCAD COM/DXF scripting, and parametric 3D models (OpenSCAD/Blender). |
| 🎨 **The Designer** | **UI/UX & Creative Media** | Design systems, accessible Tailwind components, WCAG 2.1 contrast audits, SVG diagrams, and automated video pipelines (Remotion/FFmpeg). |
| 🎮 **The Game Maker** | **Interactive Games & Simulation** | Game loops, character state machines, Bevy/Godot/Unity integration, physics checks, and procedural content generation algorithms. |
| 🌐 **The Market Scout** | **Web Scraping & Intelligence** | Resilient headless browser scraping, data extraction pipelines, and live package registry auditing (`OmniScout`) for competitor tracking. |
| 📚 **The Scholar** | **Deep Research & Analysis** | Deep parsing of technical whitepapers (PDFs), capability benchmarks, and converting dense research into Architecture Decision Records (ADRs). |
| ⚙️ **The Operator** | **Desktop & Tool RPA** | Closed-loop OS desktop automation (`WinSta0\Default`), legacy industrial GUI control, and cross-application spreadsheet sync. |
| 🛡️ **The Auditor** | **Modernization & Security** | AST code health audits, legacy migrations, CVE vulnerability scans, and atomic rollback guarantees. |

---

## 🚀 60-Second Quickstart

### 1. Installation
Install MAPai via the automated installer or compile directly with Cargo:

**Windows (PowerShell):**
```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

**Linux / macOS:**
```bash
bash ./scripts/install.sh
```

**From Source:**
```bash
cargo build --release -p saia-cli
```

### 2. Initialize Any Project
Drop MAPai into your existing codebase without touching your production logic:
```bash
mapai init --name "MyProject"
```
*Creates `.mapai/capsule.toml` tracking active project invariants, constraints, and sprint checklists.*

### 3. Audit Code Craft (Sub-Millisecond Speed)
Scan your entire codebase for hollow stubs, fake placeholders, and forbidden patterns:
```bash
mapai check
```

### 4. Launch the Dedicated Web Cockpit
Open the embedded dark-mode studio in your browser on port `4321`:
```bash
mapai ui
```

### 5. Run as Universal IDE Agent (ACP & MCP)
Connect MAPai to **Zed**, **Cursor**, **Claude Desktop**, or **Antigravity**:
```bash
# Agent Communication Protocol (Zed / Cursor)
mapai acp

# Model Context Protocol (Cursor / Antigravity / Claude Desktop)
mapai mcp
```

---

## 🏗️ Workspace Architecture

MAPai is organized as a unified, high-performance Cargo workspace:

```
crates/
├── saia-core/          # Protocol definitions, CAWO causal ledger, SQLite FTS5 memory & skill synthesizer
├── saia-verify/        # Deterministic AST scanners, blast-radius security guard & secret maskers
├── saia-orchestrator/  # Swarm 2PC coordinator, collision tracker, zero-copy context fork & overnight engine
└── saia-cli/           # Native binary (`mapai`), Web Cockpit (:4321), ACP server & MCP bridge
```

---

## 📚 In-Depth Documentation

* **[Full Technical Master Handbook (DOCUMENTATION.md)](DOCUMENTATION.md)** — Comprehensive 13-chapter guide detailing the AI-PM lifecycle, Zero-Trust security screener, global OS daemon topology, hardware sizing, and CLI command reference.
* **[Formal Scientific Specification (ARCHITECTURE_SPECIFICATION.md)](ARCHITECTURE_SPECIFICATION.md)** — Research-grade paper detailing the Two-Job Formalism ($\mathcal{J}_1 \perp \mathcal{J}_2$), Continuity-Aware Workflow Orchestration (CAWO), and IEEE 1012 IV&V proofs.
* **[Benchmark Deliverables (docs/benchmarks/)](docs/benchmarks/)** — Executive audit reports, HTML interactive comparisons, and vector benchmarks.

---

## 📄 License

Dual-licensed under **MIT** or **Apache-2.0** at your option (Rust RFC 1686 standard).
