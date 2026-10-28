#!/usr/bin/env bash
# MAPai (SAiA) Universal One-Liner Installer for Linux and macOS.
# Example: curl -fsSL https://mapai.dev/install.sh | sh

set -euo pipefail

echo -e "\033[36m╔═════════════════════════════════════════════════════════════════════════╗\033[0m"
echo -e "\033[36m║               MAPai Sovereign Suite - POSIX Installer                   ║\033[0m"
echo -e "\033[36m║        Symbiotic Adaptive Intelligent Architecture (SAiA) Protocol      ║\033[0m"
echo -e "\033[36m╚═════════════════════════════════════════════════════════════════════════╝\033[0m"
echo ""

if ! command -v cargo >/dev/null 2>&1; then
    echo -e "\033[33m[!] Rust toolchain ('cargo') not detected.\033[0m"
    echo "    MAPai requires Rust for native zero-allocation execution."
    echo "    Install via https://rustup.rs: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

CARGO_BIN="$HOME/.cargo/bin"
mkdir -p "$CARGO_BIN"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

if [ -f "$REPO_ROOT/Cargo.toml" ]; then
    echo -e "\033[36m[*] Building MAPai release binary...\033[0m"
    (cd "$REPO_ROOT" && cargo build --release -p saia-cli)
    cp -f "$REPO_ROOT/target/release/mapai" "$CARGO_BIN/mapai"
    chmod +x "$CARGO_BIN/mapai"
else
    echo -e "\033[36m[*] Installing MAPai via cargo...\033[0m"
    cargo install --path "$REPO_ROOT/crates/saia-cli" --force
fi

export PATH="$CARGO_BIN:$PATH"

echo -e "\n\033[36m[*] Running First-Time Installation (FTI)...\033[0m"
"$CARGO_BIN/mapai" fti --non-interactive --name "sovereign-workspace"

echo ""
echo -e "\033[32m═══════════════════════════════════════════════════════════════════════════\033[0m"
echo -e "\033[32m   MAPai Successfully Installed Globally on Your System!\033[0m"
echo -e "\033[32m═══════════════════════════════════════════════════════════════════════════\033[0m"
echo -e "Quick Commands:"
echo -e "  • Launch Web Studio GUI : \033[36mmapai ui\033[0m"
echo -e "  • Always-On Watch Mode  : \033[36mmapai watch\033[0m"
echo -e "  • Socratic Setup Wizard : \033[36mmapai wizard\033[0m"
echo -e "  • Audit AST Code Truth  : \033[36mmapai check\033[0m"
echo -e "  • Full Systems Diagnosis: \033[36mmapai diagnose\033[0m\n"
