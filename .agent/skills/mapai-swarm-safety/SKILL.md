---
name: "mapai-swarm-safety"
description: "Audits concurrent multi-agent file mutations and enforces Two-Phase Commit virtual staging."
triggers:
  - "concurrent edit"
  - "swarm collision"
  - "2pc commit"
---

# Skill: mapai-swarm-safety

## Overview
Audits concurrent multi-agent file mutations and enforces Two-Phase Commit virtual staging.

## Instructions
1. Acquire file lease via ReadSetCollisionTracker before inspecting source.
2. Stage all modifications in TwoPhaseCommitTransaction memory overlay.
3. Verify test passes and execute atomic commit Phase 2.

## Verification Gates
- [ ] Verify zero collision events before commit
- [ ] Ensure 0ms rollback on abort

