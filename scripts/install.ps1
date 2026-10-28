<#
.SYNOPSIS
    MAPai (SAiA) Universal One-Liner Installer for Windows PowerShell.
.DESCRIPTION
    Installs MAPai globally to $HOME\.cargo\bin, initializes MCP sidecars,
    configures zero-pollution shadow storage, and registers the sovereign trust charter.
.EXAMPLE
    irm https://mapai.dev/install.ps1 | iex
#>

$ErrorActionPreference = "Stop"

Write-Host "╔═════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║              MAPai Sovereign Suite - One-Liner Windows Installer       ║" -ForegroundColor Cyan
Write-Host "║        Symbiotic Adaptive Intelligent Architecture (SAiA) Protocol      ║" -ForegroundColor Cyan
Write-Host "╚═════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# 1. Verify Cargo / Rust Environment
$hasCargo = Get-Command "cargo" -ErrorAction SilentlyContinue
if (-not $hasCargo) {
    Write-Host "[!] Rust toolchain ('cargo') not detected." -ForegroundColor Yellow
    Write-Host "    MAPai requires Rust for native zero-allocation execution." -ForegroundColor Gray
    Write-Host "    Install via https://rustup.rs or run: winget install Rustlang.Rustup" -ForegroundColor Cyan
    exit 1
}

$cargoBinDir = Join-Path $HOME ".cargo\bin"
if (-not (Test-Path $cargoBinDir)) {
    New-Item -ItemType Directory -Force -Path $cargoBinDir | Out-Null
}

$targetExe = Join-Path $cargoBinDir "mapai.exe"

# 2. Build or Deploy Native Binary
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir

if (Test-Path (Join-Path $repoRoot "Cargo.toml")) {
    Write-Host "[*] Compiling MAPai release binary from local workspace..." -ForegroundColor Cyan
    Push-Location $repoRoot
    try {
        & cargo build --release -p saia-cli
        $builtExe = Join-Path $repoRoot "target\release\mapai.exe"
        if (Test-Path $builtExe) {
            Copy-Item -Force $builtExe $targetExe
            Write-Host "[✓] Deployed binary to $targetExe" -ForegroundColor Green
        } else {
            throw "Built binary not found at $builtExe"
        }
    } finally {
        Pop-Location
    }
} else {
    Write-Host "[*] Installing MAPai via cargo..." -ForegroundColor Cyan
    & cargo install --path "$repoRoot\crates\saia-cli" --force
}

# 3. Verify PATH availability
$pathDirs = $env:PATH -split ";"
if ($pathDirs -notcontains $cargoBinDir) {
    Write-Host "[i] Adding $cargoBinDir to current session PATH..." -ForegroundColor Yellow
    $env:PATH = "$cargoBinDir;$env:PATH"
}

# 4. Execute Sovereign First-Time Installation
Write-Host "`n[*] Running First-Time Installation (FTI)..." -ForegroundColor Cyan
& $targetExe fti --non-interactive --name "sovereign-workspace"

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "   MAPai Successfully Installed Globally on Your System!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "Quick Commands:" -ForegroundColor White
Write-Host "  • Launch Web Studio GUI : " -NoNewline; Write-Host "mapai ui" -ForegroundColor Cyan
Write-Host "  • Always-On Watch Mode  : " -NoNewline; Write-Host "mapai watch" -ForegroundColor Cyan
Write-Host "  • Socratic Setup Wizard : " -NoNewline; Write-Host "mapai wizard" -ForegroundColor Cyan
Write-Host "  • Audit AST Code Truth  : " -NoNewline; Write-Host "mapai check" -ForegroundColor Cyan
Write-Host "  • Full Systems Diagnosis: " -NoNewline; Write-Host "mapai diagnose" -ForegroundColor Cyan
Write-Host ""
