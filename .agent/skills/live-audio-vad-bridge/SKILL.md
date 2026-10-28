---
name: "live-audio-vad-bridge"
description: "Sub-100ms Multimodal Streaming with Client-Side Voice Activity Detection"
triggers:
  - "bidirectional-audio-vad-streaming"
  - "live-audio-vad-bridge"
  - "research-bidirectional-audio-vad-streaming"
---

# Skill: live-audio-vad-bridge

## Overview
Sub-100ms Multimodal Streaming with Client-Side Voice Activity Detection

## Instructions
### Background Context & Provenance
Derived autonomously from Google Gemini 3.8 Live API Technical Spec

### Capability Delta
True human-like conversational pair programming with real-time screen sensing.

### Actionable Operational Pattern
Buffer 16kHz audio chunks with local energy/RMS detection. Send interrupt tokens immediately when user voice onset crosses -24dB threshold.

### Execution Protocol
1. Ingest pattern heuristics into active subagent context.
2. Sandbox execution in temporary scratch environment.
3. Validate against deterministic verification gates before disk commit.

## Verification Gates
- [ ] Confirm pattern complies with AST truth-to-materials (0 hollow stubs)
- [ ] Verify memory and latency constraints meet sub-10ms requirement
- [ ] Anchor execution result with CAWO causal checkpoint

