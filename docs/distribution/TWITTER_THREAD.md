# Twitter / X Launch Thread for MAPai

## Tweet 1 (The Hook + Graphic)
Autonomous AI agents are great at writing code, but terrible at teamwork:
- They clobber each other's files.
- They leave hollow `// TODO: fix later` stubs.
- They bloat RAM and blow token budgets.

We benchmarked DeepSeek Harness, JCODE, & Hermes Agent — then built the solution.

Introducing **MAPai**: A sovereign Rust harness for AI agents. 🧵👇
*(Attach: docs/benchmarks/Quad_Agent_Harness_Benchmark_preview.png)*

---

## Tweet 2 (The Benchmark)
Here is our empirical benchmark comparing @deepseek_ai Harness, JCODE, @NousResearch Hermes, and MAPai:

📊 **Hollow Stub Rate (HCR)**:
- DeepSeek: 14.2%
- Hermes: 11.5%
- JCODE: 4.8%
- MAPai: 0.00% (Strict AST Quarantine)

⚡ **Memory**: MAPai runs at 2.52 MB RAM vs 80-120 MB for Python runtimes.

---

## Tweet 3 (Two-Phase Commit Swarm Safety)
How MAPai prevents multi-agent file collisions:

Agents never touch the disk directly. File edits are staged in an in-memory virtual overlay using transactional ReadSet leases.

If tests fail or another agent modified a dependency, the transaction aborts. Zero corruptions.

---

## Tweet 4 (Zero-Copy Context Forking)
When agents fork into subagents, most frameworks duplicate the entire chat transcript, burning millions of tokens.

MAPai uses zero-copy `Arc<Vec<Turn>>` memory slices in Rust — slashing token consumption by 68.4%.

---

## Tweet 5 (Blast Radius Security)
Ever had an agent run `rm -rf` or delete `.git`?

MAPai implements a deterministic 4-tier blast radius perimeter. Catastrophic paths (`~`, `/`, `C:\Windows`, `.ssh/id_rsa`, `.env`) are blocked at the binary level. The model cannot bypass it.

---

## Tweet 6 (Universal IDE & Embedded Cockpit)
- Works natively with **Cursor**, **Zed**, and **Claude Desktop** via standard ACP & MCP protocols.
- Run `mapai ui` for a zero-dependency dark mode browser cockpit running directly from the binary on port 4321. No Node.js or Docker required.

---

## Tweet 7 (Call to Action / Link)
MAPai is 100% open source under Apache-2.0 / MIT.

⭐ Star the repo & check out the technical whitepaper:
https://github.com/Nithindevaraj/MAPai

Full benchmark report & interactive HTML:
https://github.com/Nithindevaraj/MAPai/tree/main/docs/benchmarks
