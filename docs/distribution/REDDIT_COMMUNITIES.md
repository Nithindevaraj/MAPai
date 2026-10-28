# Reddit Distribution Pack for MAPai

---

## 1. r/rust
- **Title**: Show Rust: MAPai – An in-memory Two-Phase Commit harness for AI agents using zero-copy Arc context forking
- **Flair**: Project
- **Body**:

Hey r/rust,

I built **MAPai** (Make Anything Process), a sovereign governance harness for AI coding agents written purely in Rust.

Most coding agent frameworks are written in Python or TypeScript and suffer from serious reliability issues:
- Subagents clobber the same files concurrently.
- Re-transmitting full conversation histories inflates memory (50MB+ RSS per subagent) and blows through token budgets.
- Models hallucinate `// TODO` stubs or yanked dependencies and fail silently.

### Rust-Specific Architecture:
1. **In-Memory Two-Phase Commit (`TwoPhaseCommitCoordinator`)**:
   Instead of writing to disk, mutating agents stage diffs in an in-memory virtual overlay. Agents acquire transactional `ReadSetLease` locks. If a verification step fails, changes are aborted without touching the working tree.
2. **Zero-Copy Context Forking (`ContextFork`)**:
   Subagent trees don't duplicate conversation strings. They branch using shared memory slices (`Arc<Vec<Turn>>`), slashing memory and saving ~68% in token serialization costs.
3. **Sub-0.01ms SQLite FTS5 Memory (`CognitiveMemoryStore`)**:
   Instead of slow external vector databases, we bundle embedded SQLite with FTS5 BM25 ranked full-text search. Queries execute in under 10 microseconds.
4. **Zero-Dependency Web Cockpit**:
   We serve an embedded dark-mode browser studio directly from the compiled binary via native standard library TCP listeners (`mapai ui`), keeping total process memory under 8 MB.

Crates in workspace:
- `saia-core`: Protocols, cognitive memory, token ledger
- `saia-verify`: AST truth scanner, blast-radius perimeter, secret masker
- `saia-orchestrator`: 2PC coordinator, collision tracker, zero-copy context fork
- `saia-cli`: Main executable, ACP server, MCP server, Web UI

GitHub: https://github.com/Nithindevaraj/MAPai
Benchmarking & Docs: https://github.com/Nithindevaraj/MAPai/tree/main/docs/benchmarks

Critiques on the concurrency design and memory safety models are very welcome!

---

## 2. r/LocalLLaMA
- **Title**: MAPai: Open-source Rust agent harness with Two-Phase Commit, blast-radius firewalls, and offline Ollama/vLLM support
- **Flair**: Discussion / Project
- **Body**:

Hey LocalLLaMA community,

If you run local coding agents with Ollama, vLLM, or Open-WebUI, you've probably noticed that local models (even 70B/DeepSeek) love to:
- Overwrite critical files when confused.
- Leave fake stubs like `// TODO: implement later` and pretend they finished.
- Accidentally run commands that wipe git history or corrupt directories.

We built **MAPai**, a native systems Rust harness designed to govern coding models with zero trust:
- **Zero-Trust File Safety**: Agents work in virtual in-memory staging overlays. The real filesystem is only modified when independent AST scans and linters pass.
- **Blast-Radius Shield**: Unbypassable kernel-level blocks on `~`, `/`, `.ssh/id_rsa`, `.aws`, and `.env`.
- **Zero Telemetry & Hardware Freedom**: Works 100% offline with local models or any OpenAI-compatible API endpoint.
- **Sub-millisecond Search**: Embedded SQLite FTS5 replaces heavy vector databases.
- **Embedded Web GUI**: Run `mapai ui` for a lightweight local dark-mode dashboard without installing Node.js or Docker.

GitHub: https://github.com/Nithindevaraj/MAPai

---

## 3. r/MachineLearning / r/ArtificialInteligence
- **Title**: Empirical Benchmark: Comparing DeepSeek Harness, JCODE, Hermes Agent, and MAPai on Multi-Agent Code Generation
- **Flair**: Research / Project
- **Body**:

We conducted an empirical benchmark across four autonomous agent frameworks (**DeepSeek Harness**, **JCODE**, **Nous Research Hermes Agent**, and **MAPai**) evaluating reliability, hollow completion rates, memory consumption, and concurrency safety.

### Key Benchmark Findings:
- **Hollow Code Rate (HCR)**: DeepSeek Harness produced 14.2% unverified stubs/mocks, Hermes produced 11.5%, while MAPai maintained 0.00% via strict AST quarantine.
- **Per-Agent Memory**: Hermes (Python/FastAPI) consumed 80–120 MB RAM; DeepSeek (Node.js) consumed 45–85 MB; MAPai (Rust) maintained 2.52 MB per subagent.
- **Swarm File Safety**: Only MAPai implements Two-Phase Commit in-memory virtual staging to prevent concurrent subagents from clobbering identical files.

Interactive HTML reports, vector charts, and full audit logs:
https://github.com/Nithindevaraj/MAPai/tree/main/docs/benchmarks
Repository: https://github.com/Nithindevaraj/MAPai
