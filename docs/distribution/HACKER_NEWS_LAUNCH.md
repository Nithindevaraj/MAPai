# Hacker News "Show HN" Launch Guide for MAPai

## 📌 Submission Details
- **Title**: Show HN: MAPai – Rust harness for AI coding agents with 2PC and zero-copy context forking
- **URL**: https://github.com/Nithindevaraj/MAPai
- **Best Posting Window**: Tuesday or Wednesday between 8:00 AM – 10:00 AM EST (optimal front-page algorithmic velocity).

---

## 📝 Top Comment (Post Immediately After Submitting)

Hi HN,

We've been building and benchmarking autonomous multi-agent coding workflows, and kept hitting the same architectural walls:
1. **File Collision / Race Conditions:** When two subagents work concurrently on the same codebase, they mutate files out from under each other.
2. **Hollow Stubs:** LLMs love writing syntactically plausible code filled with `// TODO: implement later` and mock data, then grading their own homework and claiming everything passed.
3. **Context Inflation:** Heavy agent frameworks dump raw conversation histories on every fork, consuming huge memory (50–120MB+ RAM per agent) and inflating token costs.
4. **Command Blast Radius:** Agents routinely run destructive commands (`rm -rf`, wiping git history, or touching root directories) when an instruction goes sideways.

To fix this, we built **MAPai** (Make Anything Process) in native systems Rust.

### How it works under the hood:
- **Two-Phase Commit (2PC) Virtual Staging**: Agents never mutate the disk directly during task execution. File modifications are isolated in an in-memory staging overlay with ReadSet leases. Disk writes occur atomically only after verification passes.
- **Deterministic TruthScanner**: An AST-level verification engine that audits 50,000 LOC in `< 10ms` for stub rates (HCR), quarantining hollow code.
- **Zero-Copy Context Forking**: Subagents share conversation memory slices via `Arc<Vec<Turn>>`, cutting token consumption by 60–80% compared to traditional prompt retransmission.
- **Four-Tier Blast Radius Perimeter**: Commands are classified into `Safe`, `Low`, `Confirm`, and `Catastrophic`. Catastrophic paths (`~`, `/`, `C:\Windows`, `.ssh`, `.aws`, `.env`) are blocked at the kernel level with zero model override.
- **Sub-0.01ms Cognitive Memory**: Embedded SQLite with FTS5 BM25 full-text indexing for cross-session associative search.
- **Universal ACP & MCP Support**: Native support for the Agent Communication Protocol (Zed, Cursor) and Model Context Protocol (Claude Desktop, Antigravity).
- **Embedded Web Cockpit**: A zero-dependency dark mode UI (`mapai ui`) served directly from the compiled binary on port `4321` (0 MB node_modules, total binary footprint < 8 MB RSS).

### Empirical Benchmarks:
We ran head-to-head tests against DeepSeek Harness, JCODE, and Nous Research Hermes Agent. MAPai achieved:
- 0.00% Hollow Completion Rate (vs. 14.2% DeepSeek, 11.5% Hermes)
- 2.52 MB RAM per agent (vs. 45–85 MB DeepSeek, 80–120 MB Hermes)
- 68.4% token savings via zero-copy forks

Repo: https://github.com/Nithindevaraj/MAPai
Documentation: https://github.com/Nithindevaraj/MAPai/blob/main/DOCUMENTATION.md
Full Benchmark PDF/HTML: https://github.com/Nithindevaraj/MAPai/tree/main/docs/benchmarks

Would love feedback on our 2PC staging mechanism and the zero-copy forking architecture!
