---
name: "tokio-async-guard"
description: "Monitors async task runtimes to prevent executor deadlocks and task leaks"
triggers:
  - "tokio"
  - "async hang"
  - "deadlock"
---

# Skill: tokio-async-guard

## Overview
Monitors async task runtimes to prevent executor deadlocks and task leaks

## Instructions
1. Trace all spawned tokio tasks.
2. Ensure select! branches have default timeouts.
3. Verify graceful shutdown.

