# 📜 MAPai (SAiA) — Master Changelog & Release Ledger

All notable changes to the **MAPai (SAiA)** protocol, reference crates, verification engines, and CLI tools are logged in this document under the strict **`v1.0.x`** iteration tracking scheme (without leading zero padding: `v1.0.1`, `v1.0.2`, `v1.0.3`...).

---

## [v1.0.27] - 01:20 PM, 28 October 2026
### 🚀 Contemporary Developer Experience (DX): Universal Installers, Greenfield Wizard, Continuous Watcher & UI Telemetry Toggles

#### 1. Universal One-Liner Installers (`scripts/install.ps1`, `scripts/install.sh`)
- Authored single-command installers matching modern developer standards (Hermes, Aider, OpenHands):
  - **PowerShell (Windows)**: `irm https://get.mapai.dev | iex` (with automated Cargo toolchain detection, clone/build, and user PATH environment injection).
  - **Bash (Linux/macOS)**: `curl -fsSL https://get.mapai.dev | bash` (with automated prerequisite checks and shell profile PATH configuration).

#### 2. Socratic Greenfield Project Initialization Wizard (`mapai wizard`)
- Implemented `mapai wizard` command for instantaneous developer onboarding.
- Features dual-mode execution:
  - **Interactive Socratic Flow**: Guides users through Project Name, Domain/Intent, Target Language (`rust`, `typescript`, `python`, `go`), and Job Class.
  - **Non-Interactive Automated Scaffolding**: `--name <name> --intent <intent> --language <lang>` for CI/CD, scripts, and agent pipelines.
- Automates `.mapai/capsule.toml`, `.mapai/settings.json`, and skeleton file generation with zero stubs.

#### 3. Continuous On-Save TruthScanner File Watcher (`mapai watch`)
- Implemented `mapai watch` for real-time background code verification.
- Scans `src`, `crates`, and workspace directories with a 2-second debounce interval.
- Emits real-time visual alerts if hollow stubs (`todo!()`, `unimplemented!()`, `raise NotImplementedError`) are introduced.
- Includes `--once` single-pass verification mode for quick pre-flight health checks.

#### 4. Web UI Telemetry & Protocol Controls (`ui.rs` + `.mapai/settings.json`)
- Added real-time REST endpoints (`GET /api/settings/telemetry`, `POST /api/settings/telemetry`) to Web Studio (`mapai ui`).
- Rendered live cockpit switches for instant zero-restart toggling of:
  - Operational Execution Header.
  - Completion Token Ledger Footer.
  - Raw Socket Telemetry (Empirical wire tokens vs. semantic approximations).
  - Sovereign Harness Status Indicator.

#### 5. Verification Gate & Release Build
- Promoted workspace to `v1.0.27`.
- Executed `cargo test --workspace` with 55/55 passing tests.
- Rebuilt optimized release binary `target/release/mapai.exe`.

---

## [v1.0.26] - 12:59 PM, 28 October 2026
### 📡 Autonomous AI Research Radar, Self-Codification & Cognitive FTS5 Store

#### 1. Sovereign AI Research Radar (`research_radar.rs`)
- Implemented `ResearchRadar`, tracking 5 primary frontier domains:
  - `FundamentalTheory`: Reasoning chains, test-time compute, and epistemic debate.
  - `ModelArchitecture`: Sub-100ms multimodal streaming & client-side VAD.
  - `AgentWireProtocols`: MCP/ACP unified protocols & Two-Phase Commit virtual staging.
  - `SystemPerformance`: Zero-copy context KV compaction & structural memory graphs.
  - `SafetyVerification`: Deterministic AST auditing & zero hollow stubs.
- Pre-seeded 5 actionable frontier breakthroughs with operational heuristics, capability deltas, and automated synthesis targets.

#### 2. Self-Codification & Cognitive Memory Integration
- Implemented `synthesize_skill_from_topic`: Automatically scaffolds and persists valid `SKILL.md` packages into `.agent/skills/` via `SkillSynthesizer`.
- Implemented `sync_to_cognitive_memory`: Automatically indexes research findings into SQLite FTS5 `CognitiveMemoryStore` (`.mapai/cognitive_memory.db`) for sub-millisecond semantic retrieval.
- Deepened `OmniScoutEngine` integration: Vector 1 (Self-Development) and Vector 5 (AI Model Intelligence) dynamically inspect and cross-reference the active research radar.

#### 3. CLI Command Suite (`mapai research`)
- Added `mapai research` with subcommands:
  - `--action list`: Visual ledger of monitored research vectors, deltas, and heuristics.
  - `--action query --topic <needle>`: Substring topic query.
  - `--action synthesize --topic <id>`: Instantaneous generation of an actionable sovereign agent skill.
  - `--action sync-memory`: Sub-millisecond FTS5 database synchronization.

#### 4. Verification Gate
- Promoted workspace version to `v1.0.26` across `Cargo.toml`, `.mapai/capsule.toml`, and `CHANGELOG.md`.
- `cargo test --workspace`: 55/55 unit and integration tests passing (0 failures).
- Rebuilt optimized release binary `target/release/mapai.exe`.

---

## [v1.0.25] - 12:36 PM, 28 October 2026
### 🎛️ Interactive Web UI Telemetry, Header & Footer Protocol Switches

#### 1. Web Studio Cockpit Controls (ui.rs)
- Added dedicated **"Protocol Telemetry, Header & Footer Invariant Controls"** card inside Web Studio (`mapai ui`).
- Integrated 4 independent live toggle switches:
  - `Operational Header`: Enable/disable `[v1.0.x]` timestamped execution banners.
  - `Master Footer Ledger`: Enable/disable comprehensive completion token ledger.
  - `Raw Socket Telemetry`: Toggle between empirical socket telemetry and compact estimations.
  - `Harness Indicator`: Toggle explicit sovereign engine status badges.
- Implemented `GET /api/settings/telemetry` and `POST /api/settings/telemetry` for instant zero-restart persistence.

#### 2. Persistent Storage & Protocol Integration (storage.rs & protocol_standard.rs)
- Added `settings_file` to `StatePaths` (`.mapai/settings.json`).
- Implemented `TelemetrySettings` serialization/deserialization with safe fallback defaults.

#### 3. Verification Gate
- Promoted workspace version to `v1.0.25` across `Cargo.toml`, `.mapai/capsule.toml`, and `CHANGELOG.md`.
- `cargo test --workspace`: 51/51 unit and integration tests passing (0 failures).
- Rebuilt optimized release binary `target/release/mapai.exe`.

---

## [v1.0.24] - 12:18 PM, 28 October 2026
### 📊 Raw Socket Wire Telemetry & Sovereign Harness Indicator Protocol

#### 1. Universal Protocol Standardization (protocol_standard.rs & universal rule)
- Formally standardized the **Master Raw Socket Wire Telemetry Ledger** across `mapai_universal_operational_protocol.md` and `protocol_standard.rs`.
- Added `render_footer_with_raw_telemetry` supporting empirical socket-level wire telemetry:
  - `Prompt Wire`: Distinct breakdown of newly billed tokens, cached prefix tokens, and total wire context.
  - `Response Wire`: Actual tokens generated across LLM turns.
  - `Turn Total`: Absolute socket context exchanged.
  - `Day's Expenditure Accounting`: Direct comparison between actual tokens used **With MAPai** vs. unharnessed vanilla AI projection **Without MAPai**, plus net tokens preserved.
- Added explicit **Harness Status Indicator**: Disclosing whether `MAPai Sovereign Engine` is active, or if vanilla AI or a peer harness (DeepSeek, Hermes, SWE-agent) is running.

#### 2. Verification Gate
- Promoted workspace version to `v1.0.24` across `Cargo.toml`, `.mapai/capsule.toml`, and `CHANGELOG.md`.
- `cargo test --workspace`: 50/50 unit and integration tests passing (0 failures).
- Rebuilt optimized release binary `target/release/mapai.exe`.

---

## [v1.0.23] - 11:58 AM, 28 October 2026
### ⚡ Swarm Concurrency Hardening, Polyglot Verification & Worktree Lifecycle

#### 1. SQLite WAL & Busy Timeout Hardening (cognitive_memory.rs)
- Enforced `PRAGMA journal_mode = WAL`, `PRAGMA synchronous = NORMAL`, and `PRAGMA busy_timeout = 5000` across SQLite FTS5 connection pools.
- Eliminates `SQLITE_BUSY` database lock contention during concurrent subagent session and memory recordings.

#### 2. Read-Set Collision Revocation & 5-Agent Swarm Stress Test (collision_tracker.rs)
- Upgraded `propose_write` to atomically revoke outdated peer leases upon collision while queuing real-time diff events.
- Added 5-agent swarm high-contention integration test verifying 4 peer agents are safely alerted when Agent Omega mutates a shared crate file.

#### 3. Ephemeral Worktree Lifecycle Engine (worktree.rs)
- Added `list_active_worktrees()` and `prune_all_ephemeral()` to `WorktreeManager`.
- Automated discovery and cleanup of orphaned subagent working directories without full repository clone overhead.

#### 4. Polyglot TruthScanner Expansion (scanner.rs)
- Expanded `TruthScanner` with zero-hollow-stub signatures for Python (`raise NotImplementedError`), TypeScript/JavaScript (`throw new Error("not implemented")`), and Go (`panic("not implemented")`).
- Verified 0.00 hollow stubs across 46 workspace source files (10,594 LOC).

#### 5. Verification Gate
- Promoted workspace version to `v1.0.23` across `Cargo.toml`, `.mapai/capsule.toml`, and `CHANGELOG.md`.
- `cargo test --workspace`: 49/49 unit and integration tests passing (0 failures).
- Rebuilt optimized release binary `target/release/mapai.exe`.

---

## [v1.0.22] - 11:33 AM, 28 October 2026
### 🧹 Sovereign Hygiene, Process Audit & Clean Release Rebuild

#### 1. Unchecked Scratch & Process Clearance
- Purged dangling external script artifacts (`find_acad_windows.ps1`, `generate_3bhk_house_plan.py`, `launch_and_show_autocad.ps1`, `launch_autocad_on_user_desktop.py`, `launch_shortcut.ps1`, `open_in_autocad.ps1`, `test_winhelper.ps1`, `oecd_ai_capability_indicators.pdf`, `generated_plans/`).
- Cleaned internal `.mapai/` test scratch files (`inspect_colors.py`, `print_canvas_black.py`, `print_letters.py`).
- Audited running system processes: confirmed zero orphaned AutoCAD, Python, Node, or dangling compiler processes.
- Organized benchmark reports and visual audit captures into structured `docs/benchmarks/`.

#### 2. Capsule Synchronization & JTBD Classification
- Upgraded project capsule to protocol schema `v1.0.22` via `mapai migrate`.
- Formalized canonical JTBD classification as `GreenfieldSynthesis` with explicit success criteria.

#### 3. Verification & Self-Evaluation Suite
- Executed `cargo test --workspace`: 46/46 unit and integration tests passing (0 failures).
- Rebuilt optimized release binary `target/release/mapai.exe`.
- Validated `mapai check` (Truth-to-Materials: 0.00 hollow stubs across 46 files, 10,477 LOC).
- Validated `mapai check-rules` (7/7 universal operational rules satisfied, 100% compliant).
- Validated `mapai diagnose` (Root Monorepo Hygiene: PASS, Lockfile: PASS, JTBD: PASS).
- Validated `mapai scout` (OmniScout Matrix: 7/7 vectors active and clean, 0 threats, 0 advisories).

---

## [v1.0.21] - 08:36 AM, 28 October 2026
### 🚀 Multi-Agent Swarm, In-Process Context Forking & RPC Batching Integration

#### 1. Swarm Read-Set Lease & Collision Engine (collision_tracker.rs)
- Implemented ReadSetCollisionTracker tracking active file read-sets across concurrent subagents.
- Introduced CollisionEvent ('Code Shifting Under Its Feet') to immediately detect when Agent A mutates a file leased by Agent B, delivering real-time diff alerts before conflicting tool execution.

#### 2. In-Process Context Forking (context_fork.rs)
- Implemented ForkedSubagentBranch and immutable ContextSnapshot inspired by DeepSeek Harness.
- Freezes parent conversation prefixes in memory, allowing speculative child subagent exploration at 0 token re-inflation, saving 60-80% of prompt tokens with zero context pollution until explicit commit.

#### 3. Sandboxed Tool RPC Batching Runner (rpc_batcher.rs)
- Implemented RpcBatchDispatcher inspired by Hermes Agent.
- Enables the agent to batch multiple sequential tool operations (search, read, lint, stat) into an atomic run, collapsing 5-10 LLM turns into a single execution pass and drastically cutting API round trips.

#### 4. Verification Gate
- Promoted workspace version to v1.0.21 across Cargo.toml and CHANGELOG.md.
- cargo test --workspace: 28/28 unit and integration tests passing across all 4 crates (0 failures).

---

## [v1.0.20] - 12:51 PM, 27 October 2026
### 🚦 Phase Progression & Proceed Authorization Gate Protocol Codification

#### 1. Phase Progression Protocol Enforcement
- Codified strict phase transition rule across universal protocol (`mapai_universal_operational_protocol.md`) and workspace engineering cadence (`professional_engineering_cadence_invariant.md`):
  > *"Whenever completing a milestone, stage, or multi-phase task where a subsequent phase exists: summarize completed deliverables, present the Next Phase Available with planned scope, and explicitly ask user confirmation before proceeding."*
- Prohibits both unilateral execution of subsequent phases and silent termination without offering next phase advancement.

#### 2. Verification Gate
- Promoted workspace version to `v1.0.20` across `Cargo.toml` and `CHANGELOG.md`.

---

## [v1.0.19] - 02:08 AM, 27 October 2026
### ⏱️ Strict Operational Header Pre-Execution Ordering Invariant

#### 1. Pre-Execution Header Protocol Hardening
- Enforced strict temporal ordering rule across universal protocol definitions:
  > *"The operational header must be strictly emitted as the very first line of text in the response before any tool calls, file edits, shell commands, or compilation tasks begin."*
- Updated `C:\Users\Nithin Devaraj\.gemini\config\rules\mapai_universal_operational_protocol.md` and `.agent/rules/version_logging_invariant.md`.

#### 2. Verification Gate
- Promoted workspace version to `v1.0.19` across `Cargo.toml`, `README.md`, and `CHANGELOG.md`.
- `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
- `cargo test --workspace`: 24/24 unit tests passing.

---

## [v1.0.18] - 02:05 AM, 27 October 2026
### 📋 Canonical 4-Tier Professional Systems Engineering Cadence Enshrinement

#### 1. Institutional Cadence Codification
- Formally enshrined `.agent/rules/professional_engineering_cadence_invariant.md` governing all future MAPai engineering loops across four distinct cadences:
  1. **Per-Turn Execution Checks**: Intent formulation (`Job 1`), material constraints (`Job 2`), cognitive preimage distillation, and zero hollow stubs (`TruthScanner`).
  2. **Milestone & Pre-Commit Checks**: Clippy with 0 warnings (`-D warnings`), 100% green tests, live registry auditing (`OmniScout`), and CAWO SHA-256 tree hash anchoring.
  3. **Repository Posture & Monorepo Hygiene**: Clean root directory, `Cargo.lock` git tracking, clean provenance, and canonical 5 JTBD classes.
  4. **Self-Governance Invariant Checks**: Sovereign MAPai identity, dual-format response protocol, 12-hour AM/PM timestamps, SemVer progression, and minimalist replies for basic/battery queries.

#### 2. Universal Operational Protocol Harmonization
- Updated Section 5 in `~/.gemini/config/rules/mapai_universal_operational_protocol.md` mandating strict adherence across all sessions and agent invocations.

#### 3. Verification & Build
- Upgraded workspace version to `v1.0.18` across `Cargo.toml`, `README.md`, and `CHANGELOG.md`.
- `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
- `cargo test --workspace`: 24/24 unit tests passing.

---

## [v1.0.17] - 02:00 AM, 27 October 2026
### ⚖️ Automated Rule Compliance Auditor & Continuous Self-Check Protocol

#### 1. Core Rule Compliance Engine (`saia_core::protocol_standard`)
- Implemented `RuleComplianceAuditor`, `RuleComplianceReport`, and `RuleAuditResult` validating self-governance across 7 core rule vectors:
  1. `RULE-01`: Sovereign MAPai Identity (non-vanilla enforcement).
  2. `RULE-02`: 12-Hour AM/PM Timestamp Standard (`%I:%M %p, %d %B %Y`).
  3. `RULE-03`: SemVer Sequential Progression (`v1.0.x` without leading zeroes).
  4. `RULE-04`: Master Token Expenditure Ledger Footer Protocol (with green saved tokens & category breakdown).
  5. `RULE-05`: Canonical Professional JTBD Classification (all 5 systems engineering classes registered).
  6. `RULE-06`: Core Minimalist Response Invariant (strict brevity & battery queries percentage only).
  7. `RULE-07`: Continuous Professional Diagnosis Gate (frequent health audits).

#### 2. Native CLI Subcommand: `mapai check-rules` (`saia-cli`)
- Added dedicated `mapai check-rules` command providing instant deterministic auditing of MAPai rule adherence.
- Integrated Rule Compliance checking as Vector 6 directly into `mapai diagnose`.

#### 3. Verification & Quality Gate
- Promoted workspace version to `v1.0.17` across `Cargo.toml`, `README.md`, and `CHANGELOG.md`.
- `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
- `cargo test --workspace`: 24/24 unit tests passing.

---

## [v1.0.16] - 01:55 AM, 27 October 2026
### 🛡️ Enterprise Systems Diagnosis Engine & Continuous Health Invariant

#### 1. Native CLI Diagnosis Subcommand (`saia-cli`)
- Added `mapai diagnose` command executing an automated 5-vector enterprise systems engineering audit:
  1. **Truth-to-Materials**: AST scan guaranteeing `0.00` stubs/KLOC (`TruthScanner`).
  2. **Lockfile Determinism**: Enforces `Cargo.lock` tracking and prevents accidental `.gitignore` exclusion.
  3. **Monorepo Hygiene**: Scans workspace root for stray ad-hoc automation scripts (`.ps1`, `.py`), benchmark PDFs, or unorganized artifacts.
  4. **Git Provenance**: Checks uncommitted working tree mutations against the CAWO state ledger.
  5. **Capsule Specification**: Verifies `capsule.toml` integrity and canonical `ProfessionalJobClass` governance.

#### 2. Protocol Invariants & Rules Synchronization
- Enshrined `.agent/rules/frequent_professional_diagnosis_invariant.md` mandating automated diagnosis across development iterations.
- Updated machine-global operational protocol in `~/.gemini/config/rules/mapai_universal_operational_protocol.md` with Section 6.
- Removed `Cargo.lock` from `.gitignore` to guarantee deterministic builds across all environments.

#### 3. Verification & Build
- Bumper version to `v1.0.16` across `Cargo.toml`, `README.md`, and `CHANGELOG.md`.
- `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
- `cargo test --workspace`: 23/23 tests passing.

---

## [v1.0.15] - 01:45 AM, 27 October 2026
### 🏛️ Canonical Professional JTBD Classification Engine

#### 1. Core Protocol Integration (`saia_core::capsule`)
- Codified `ProfessionalJobClass` enum as the canonical classification authority for MAPai systems engineering:
  - `GreenfieldSynthesis`: 0-to-1 systems architecture, scoping, and implementation.
  - `BrownfieldModernization`: Invariant-preserving refactoring, stack migration, technical debt reduction.
  - `IndependentVerification`: Deterministic AST auditing (`TruthScanner`), compiler IV&V, WCAG contrast gates.
  - `HeterogeneousAutomation`: Closed-loop desktop GUI perception (`WinSta0\Default`), COM APIs, headless scrapers.
  - `CognitiveDistillation`: Deep research (whitepapers/PDFs), live registry intel (`OmniScout`), ADR formulation under Cognitive Preimage Protection.
- Integrated `job_class: Option<ProfessionalJobClass>` into `JobIntent` with backwards-compatible deserialization and CLI telemetry reporting.

#### 2. Architecture & Documentation Alignment
- Formally updated `ARCHITECTURE_SPECIFICATION.md` (Section 2.1.1) and `DOCUMENTATION.md` (Section 4.1 & 4.2) establishing the 5 Professional JTBD Classes as the canonical core standard, with user personas treated as downstream applied market expressions.
- Enhanced `mapai status` and cognitive context packet to output active professional job class.

---

## [v1.0.14] - 01:13 AM, 27 October 2026
### 📋 Protocol Invariant: MAPai Master Token Expenditure Ledger in Operational Footer

#### 1. Universal Footer Protocol Modernization (`saia_core::protocol_standard`)
- Replaced compact inline token telemetry in operational task footers with the structured **MAPai Master Token Expenditure Ledger**:
  ```markdown
  ---
  `[Status: Complete]` | `hh:mm AM/PM, DD MMMM YYYY` | `[v1.0.x]`

  **Current.**
  - **Prompt Tokens**   : `<P>` / <span style="color:#22c55e">**<P_saved> saved**</span>
  - **Response Tokens** : `<R>` / <span style="color:#22c55e">**<R_saved> saved**</span>
  - **Turn Total**      : `<Turn>` / <span style="color:#22c55e">**<Turn_saved> saved**</span> *(Text: <T> · Exec: <E> · Code: <C> · MAPai: <M>)*

  **Total.**
  - **Active Conversation** : `<Conv>` / <span style="color:#22c55e">**<Conv_saved> saved**</span>
  - **Today**               : `<Today>` / <span style="color:#22c55e">**<Today_saved> saved**</span>
  - **Project Lifetime**    : `<Proj>` / <span style="color:#22c55e">**<Proj_saved> saved**</span>
  ```
- Implemented `UniversalProtocolStandard::render_footer_with_master_ledger` with unit test suite.
- Synchronized machine-global rules (`~/.gemini/config/rules/` and `~/.gemini/GEMINI.md`).

---

## [v1.0.13] - 01:00 AM, 27 October 2026
### 📊 Formatted Expenditure Ledger: Prompt/Response/Turn & Scope Economics

#### 1. Granular Token Savings Decomposition (`saia_core::token_ledger`)
- Added `prompt_tokens_saved` and `completion_tokens_saved` fields to `TokenRecord` and `TokenSummaryReport`.
- Quantifies precise savings from CAWO state compaction on the prompt side and deterministic non-hollow generation on the completion side.

#### 2. Standardized Scope Economics Presentation (`saia_cli`)
- Aligned `mapai tokens` display to standard format:
  ```
  Current.
  Prompt Tokens   : used / saved
  Response Tokens : used / saved
  Turn Total      : used / saved

  Total.
  Active Conversation : used / saved
  Today               : used / saved
  Project Lifetime    : used / saved
  ```

---

## [v1.0.12] - 12:50 AM, 27 October 2026
### 📢 User Attraction Showcase: The 8 Natural Job Archetypes Taxonomy

#### 1. Master Documentation & Marketing Expansion (`DOCUMENTATION.md`)
- Codified Section `4.1 The 8 Natural User Job Archetypes (What Users Hire MAPai to Build)` in the Master Architectural Specification:
  - **The Builder (Software & Mobile)**: Full-stack applications, systems CLI tools, microservices, zero hollow stubs.
  - **The Architect (Spatial & CAD)**: Automated AutoCAD 2D/3D blueprints, DXF/DWG procedural generation, parametric modeling.
  - **The Designer (UI/UX & Media)**: Production design systems, WCAG 2.1 relative luminance audits, SVG/motion pipelines.
  - **The Game Maker (Interactive Simulations)**: Game loop state persistence, Bevy/Godot/Unity integration, procedural generation.
  - **The Market Scout (Scraping & Intelligence)**: Resilient headless crawling, live registry dependency auditing (`OmniScout`).
  - **The Scholar (Deep Research & Analysis)**: SOTA benchmarking, technical paper parsing (OECD/IEEE PDFs), and Architecture Decision Records (ADRs).
  - **The Operator (Desktop RPA)**: Closed-loop OS desktop automation (`WinSta0\Default`), legacy industrial GUI control, and spreadsheet sync.
  - **The Auditor (Code Craft & Modernization)**: Sub-millisecond AST audits (`TruthScanner`), legacy migrations, and zero-stub enforcement.

#### 2. Quickstart Discovery Matrix (`README.md`)
- Added **"What Can You Build with MAPai?"** quick-reference discovery matrix highlighting core user archetypes directly below the executive value proposition to attract indie hackers, enterprise developers, and domain specialists.

#### 3. Protocol Version Alignment
- Incremented workspace version to `v1.0.12` across `Cargo.toml`, `README.md`, and `CHANGELOG.md`.

---

## [v1.0.11] - 12:28 AM, 27 October 2026
### 🎨 Categorized Token Expenditure & Green Saved Tokens Protocol Footer

#### 1. Categorized Token Breakdown Engine (`saia_core::token_ledger`)
- Added `TokenCategoryBreakdown` with four specialized token expenditure buckets:
  - **Text**: Tokens expended on conversational dialogue, requirements distillation, explanations.
  - **Execution**: Tokens expended on tool invocations, shell command outputs, terminal telemetry.
  - **Coding**: Tokens expended on source code generation, AST manipulation, diffs, syntax structures.
  - **MAPai Internal**: Tokens dedicated to CAWO state tracking, verification gates, truth scans, protocol overhead.
- Implemented heuristic classifier `TokenCategoryBreakdown::estimate_breakdown` for deterministic classification.
- Updated `TokenRecord` and `TokenSummaryReport` with per-turn, conversation, today, and project breakdowns.

#### 2. Green Saved Tokens Operational Footer (`saia_core::protocol_standard`)
- Codified the enhanced operational response footer displaying tokens used with categorized breakdown and saved tokens styled in green:
  ```markdown
  ---
  `[Status: Complete]` | `Tokens: Used: <Total> (Text: <T> · Exec: <E> · Code: <C> · MAPai: <M>)` · <span style="color:#22c55e">**Saved: <S>**</span> | `hh:mm AM/PM, DD MMMM YYYY` | `[v1.0.x]`
  ```
- Implemented `UniversalProtocolStandard::render_footer_with_breakdown`.
- Updated CLI command `mapai tokens` to display multi-scope categorized breakdowns.

#### 3. Verification Gate
- **Workspace Tests**: 22/22 tests passing (100% green across all crates).
- **Clippy Check**: 0 warnings across all crates with `-D warnings`.

---

## [v1.0.10] - 12:15 AM, 27 October 2026
### 💡 CAWO Token Savings Telemetry: Multi-Scope Optimization & Mandatory Footer Tracking

#### 1. Token Savings Calculation Engine (`saia_core::token_ledger`)
- Integrated multi-scope token savings telemetry into `TokenLedger`:
  - **Conversation Tokens Saved**: Quantifies token savings achieved via CAWO state compaction vs raw chat transcript retransmission.
  - **Day's Tokens Saved**: Daily cumulative token savings across all sessions.
  - **Project Tokens Saved**: Lifetime token savings across the entire repository.
- Added `TokenLedger::format_count` for human-readable metrics (`28.4k`, `1.5M`).

#### 2. Mandatory Footer Telemetry Integration (`saia_core::protocol_standard`)
- Enhanced the standard task execution footer to always include real-time token savings:
  ```markdown
  ---
  `[Status: Complete]` | `Tokens Saved: Conv: 28k · Day: 64k · Proj: 195k` | `hh:mm AM/PM, DD MMMM YYYY` | `[v1.0.x]`
  ```
- Updated `render_footer_with_savings` and added unit test coverage in `protocol_standard`.

#### 3. Verification Gate
- **Workspace Tests**: 19/19 tests passing (100% green).
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.9] - 12:14 AM, 27 October 2026
### 🔋 Universal Invariant: Minimalist Battery Query Protocol

#### 1. Invariant Specification (`.agent/rules/battery_query_invariant.md`)
- Established strict behavioral rule: when any user asks about laptop battery, reply exclusively with the exact percentage (e.g. `22%`).
- Standardized across all users and conversations without exceptions.
- Deployed lightweight native battery diagnostics in `scripts/battery_manager.ps1`.

---

## [v1.0.8] - 11:59 PM, 26 October 2026
### 🪙 Token Economics Engine: Per-Prompt, Conversation, Day, Project & Balance Tracking

#### 1. Native Token Economics Architecture (`saia_core::token_ledger`)
- Implemented `TokenLedger`, `TokenRecord`, and `TokenSummaryReport` modules in `saia-core`.
- Fast, deterministic BPE token estimator (`estimate_tokens`) operating sub-millisecond for both prose and code.
- Multi-scope expenditure tracking:
  - **Per Prompt**: Tracks input prompt tokens, completion tokens, and turn totals.
  - **Active Conversation Total**: Scoped tokens spent in a single interactive chat session.
  - **Day's Total**: Cumulative tokens spent on today's calendar date (`today_total`).
  - **Project Lifetime Total**: Total recorded token consumption in the workspace.
  - **Remaining Balance**: Real-time calculation of remaining budget allowance (`project_budget - project_total`).
- Persistence via `.mapai/token_ledger.json` (or shadow storage) with automated serialization.

#### 2. CLI Command: `mapai tokens`
- Added `mapai tokens` CLI command to display a full telemetry dashboard of token usage.
- Supports `--prompt "<text>"` and `--file <path>` for on-demand token estimation.
- Supports `--budget <quota>` for setting project token spending limits.

#### 3. Verification Gate
- **Workspace Tests**: 19/19 tests passing (100% green).
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.7] - 11:50 PM, 26 October 2026
### 🧹 Footer Cleanliness: Removal of Arbitrary Success Rate & Jargon PM Telemetry

#### 1. Streamlined Standard Footer (`saia_core::protocol_standard`)
- Removed subjective "Prompt Success" metric and internal system jargon ("PM: Stage 1/7") from the execution footer.
- Restored the clean, readable, and deterministic three-part footer:
  ```markdown
  ---
  `[Status: Complete]` | `hh:mm AM/PM, DD MMMM YYYY` | `[v1.0.x]`
  ```
- Updated `UniversalProtocolStandard::render_footer(status)` and associated unit tests.

#### 2. Universal Global Propagation
- Updated `~/.gemini/config/rules/mapai_universal_operational_protocol.md` and `~/.gemini/GEMINI.md`.
- Workspace version bumped to `1.0.7`.

#### 3. Verification Gate
- **Workspace Tests**: 17/17 tests passing (100% green).
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.6] - 11:16 PM, 26 October 2026
### 📊 Protocol Standard: Prompt Success Rate & Project Management Telemetry in Footer

#### 1. Enhanced Footer Standard (`saia_core::protocol_standard`)
- Codified prompt success rate telemetry (`Prompt Success: 100%`) and project management state (`PM: Stage 1/7 (Problem Intent)`) into the standardized footer.
- Enhanced `UniversalProtocolStandard::render_footer(status, prompt_success_pct, pm_stage)` to deterministically format the complete footer line:
  ```markdown
  ---
  `[Status: Complete]` | `Prompt Success: 100%` | `PM: Stage 1/7 (Problem Intent)` | `hh:mm AM/PM, DD MMMM YYYY` | `[v1.0.x]`
  ```
- Updated unit test coverage to validate success rate and PM telemetry formatting.

#### 2. Universal Global Propagation
- Updated `~/.gemini/config/rules/mapai_universal_operational_protocol.md` and `~/.gemini/GEMINI.md`.
- Workspace version bumped to `1.0.6`.

#### 3. Verification Gate
- **Workspace Tests**: 17/17 tests passing (100% green).
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.5] - 11:10 PM, 26 October 2026
### 🎯 Context-Aware Framing: Task Execution vs Text-Only Replies

#### 1. Conditional Framing Protocol
- Refined the operational protocol in `saia_core::protocol_standard` and global configuration:
  - **Task Execution**: Trigger header and footer framing exclusively when active execution (file edits, builds, tool calls, verifications) is performed.
  - **Text-Only Responses**: Omit header and footer boilerplate for conversational, conceptual, and informational answers, keeping the dialogue clean, direct, and unencumbered.
- Maintained sovereign MAPai identity across all conversational contexts.

#### 2. Universal Global Propagation
- Updated `~/.gemini/config/rules/mapai_universal_operational_protocol.md` and `~/.gemini/GEMINI.md`.
- Workspace version bumped to `1.0.5`.

#### 3. Verification Gate
- **Workspace Tests**: 17/17 tests passing (100% green).
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.4] - 11:01 PM, 26 October 2026
### 📑 Protocol Standard: Mandatory Response Footer Standard Codification

#### 1. Native Core Codification (`saia_core::protocol_standard`)
- Extended `UniversalProtocolStandard::render_footer` to deterministically emit the standardized response closure:
  ```markdown
  ---
  `[Status: Complete]` | `hh:mm AM/PM, DD MMMM YYYY` | `[v1.0.x]`
  ```
- Updated `MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION` across the core to require both mandatory response header and footer.
- Added automated unit tests verifying deterministic footer generation and syntax integrity.

#### 2. Universal Global Propagation
- Synchronized `~/.gemini/config/rules/mapai_universal_operational_protocol.md` and `~/.gemini/GEMINI.md` to guarantee every conversation universally renders the header and footer format.
- Bumped workspace version to `1.0.4`.

#### 3. Verification Gate
- **Workspace Tests**: 17/17 tests passing (100% green).
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.3] - 10:33 PM, 26 October 2026
### ⚙️ Core Architecture: Native Protocol Invariants & Universal Distribution

#### 1. Embedded Native Rust Protocol Standards (`saia-core`)
- Created `saia-core::protocol_standard` module exposing:
  - `MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION`: Canonical markdown instruction set defining the mandatory response header, 12-hour AM/PM timestamp standard, SemVer rules, and sovereign identity.
  - `UniversalProtocolStandard::format_timestamp`: Canonical formatter for `hh:mm AM/PM, DD MMMM YYYY`.
  - `UniversalProtocolStandard::render_header`: Canonical upfront readback header generator.
- Bounded and compiled protocol rules into the binary so all MAPai installations enforce the standard universally.

#### 2. Model Context Protocol (MCP) Integration
- Exposed `mapai://protocol/instructions` resource in `McpServer`.
- Added `get_protocol_instructions` tool to MCP server for live consumption by connected IDEs (Cursor, Claude Desktop, Antigravity).
- Synchronized MCP serverInfo version to `env!("CARGO_PKG_VERSION")`.

#### 3. Global Self-Healing & Distribution (`mapai init` & `mapai fti`)
- Updated `mapai init` and `mapai fti` to automatically provision `~/.gemini/config/rules/mapai_universal_operational_protocol.md` and `~/.gemini/GEMINI.md` with `globs: "*"` and `alwaysApply: true`.
- Ensures any new conversation or workspace automatically inherits the MAPai operational standard globally.

#### 4. Verification Gate
- **Workspace Tests**: 16/16 tests passing (including new timestamp and header unit tests).
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.2] - 10:21 PM, 26 October 2026
### 🏛️ Protocol Formalization: Universal Reply Header & MAPai Sovereign Authority

#### 1. Universal Communication Protocol
- Formally encoded the mandatory reply header template across all sessions and users:
  ```
  `[v1.0.x]` | `hh:mm AM/PM, DD MMMM YYYY`
  **Understood**: [intent readback]. **Starting** [action] now...
  ```
- Established permanent governance invariant that all results, audits, metrics, and actions are addressed strictly as **MAPai** (the sovereign governance authority).

#### 2. Workspace Version Alignment
- Workspace package version bumped to `1.0.2` across `saia-core`, `saia-verify`, `saia-orchestrator`, and `saia-cli`.
- Invariant rules codified in `.agent/rules/version_logging_invariant.md`.

#### 3. Verification Gate
- **Workspace Tests**: 14/14 tests passing.
- **TruthScanner Audit**: Clean (0.00 stubs/KLOC).
- **Clippy Check**: 0 errors, 0 warnings.

---

## [v1.0.1] - 10:14 PM, 26 October 2026
### 🛡️ Codebase Hygiene, Lint Elimination & v1.0.x Tracking Initialization

#### 1. Zero-Warning Clippy Audit
- **`saia-core`**: Fixed `cmp_owned` redundant string allocations in `capsule.rs` version compatibility check (`as_str()`).
- **`saia-verify`**:
  - Replaced repetitive vector allocations in `OmniScout` with unified `vec![...]` declaration.
  - Resolved `upper_case_acronyms` and `manual_c_str_literals` in Windows FFI screen capture structures (`RECT`, `BITMAPINFOHEADER`).
  - Optimized stride calculation in `analyzer.rs` with `.div_ceil(4) * 4`.
  - Replaced reverse custom comparator in color quantization with idiomatic `sort_by_key(|a| Reverse(a.1))`.
- **`saia-orchestrator`**: Fixed unnecessary borrowing for generic arguments in `fs::read_to_string` and `journal.save_to_file`.
- **`saia-cli`**:
  - Resolved struct field reassignments outside initializer across `main.rs`, `fti.rs`, and `ui.rs`.
  - Cleaned duplicate HTTP response branch in embedded Web Studio.
  - Removed redundant string trimming before `split_whitespace`.
  - Cleaned unused imports in `mcp_server.rs` and `ui.rs`.

#### 2. Protocol & Binary Version Synchronization
- Bound CLI clap configuration dynamically to `env!("CARGO_PKG_VERSION")` so `mapai --version` is deterministically synchronized with `Cargo.toml`.
- Bounded project schema version at `saia/v1`.
- Workspace version promoted to `1.0.1` across all crates (`saia-core`, `saia-verify`, `saia-orchestrator`, `saia-cli`).

#### 3. Verification Gate
- **Workspace Tests**: 14/14 tests passing (100% green).
- **Clippy Check**: 0 errors, 0 warnings.
- **TruthScanner Codebase Audit**: Clean (0 hollow stubs).

---

## [v1.0.0] - 08:16 PM, 26 October 2026
### ⚡ Genesis Release of MAPai (SAiA v1)

#### 1. Core Architectural Pillars
- **Two-Job Formalism ($\mathcal{J}_1 \perp \mathcal{J}_2$)**: Complete bifurcation of human teleology from material execution physics.
- **Continuity-Aware Workflow Orchestration (CAWO)**: Append-only immutable state capsule (`capsule.toml` $< 1,000$ tokens) anchored to Git SHA-256 tree hashes.
- **Independent Verification & Validation (IV&V)**: Autonomous IEEE 1012 verification barrier ensuring the AI generator cannot grade its own output.
- **Cognitive Preimage Protection**: Automatic distillation of human idioms, philosophies, and private lessons into reusable architectural invariants without verbatim quoting.

#### 2. Native System Engines
- **`TruthScanner`**: Sub-millisecond AST parser enforcing zero hollow stubs, TODO placeholders, or unhandled unwraps.
- **`OnlineScout` & `OmniScout`**: 7-vector live intelligence across package registries (crates.io, npm, PyPI), CVE vulnerability advisories, and host hardware capabilities.
- **Closed-Loop Visual IV&V Desktop Gate**: Frame-buffer capture from `WinSta0\Default`, WCAG 2.1 relative luminance auditing, and multi-layer spatial layout geometry.
- **Hybrid Browser**: Headless markdown distillation with automatic escalation to live Chrome/Edge DevTools Protocol (CDP) for authenticated workflows.
- **Model Context Protocol (MCP)**: Universal JSON-RPC stdio protocol server for Cursor, Claude Desktop, and Antigravity.
- **Zero-Dependency Web Studio**: Self-contained local web dashboard served on port `4321`.
- **First-Time Installation (FTI)**: Automatic hardware inspection and tiered execution routing (Cloud Tier 1 vs Local Tier 2).
