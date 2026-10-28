---
description: Mandatory pre-flight spatial perception and closed-loop verification invariant for any desktop GUI or OS automation.
globs: "*"
alwaysApply: true
---

# Desktop GUI Spatial & Layout Perception Invariant

## Mandatory Pipeline-First Execution
Never execute ad-hoc, un-tracked scratch binaries (`.cs`, `.exe`, or loose scripts) in the project root to interact with desktop applications. All desktop GUI automation MUST flow strictly through the sovereign MAPai pipeline:
1. **Perception**: Run `mapai screen layout` or `mapai screen perceive` to dynamically resolve DPI, window bounds, command ribbon, and canvas viewport coordinates.
2. **Action Batching**: Run `mapai screen act` with the target window, action recipe, payload, and commit strategy (`--commit click-outside`).
3. **Closed-Loop Verification**: Pass `--verify` or run `mapai screen verify` to ensure the visual gate confirms contrast, non-blank content, and text commitment before declaring success.

## Mandatory Pre-Flight Perception
Before issuing any input events (clicks, drags, keystrokes, SendKeys) to an operating system desktop application:
1. **Resolve Display Metrics & DPI:** Query physical screen resolution, virtual desktop bounds, and monitor DPI scale factor. Never assume 96 DPI (100% scale).
2. **Isolate Workspace Geometry:** Explicitly identify the client canvas/document workspace separate from the navigation chrome, ribbon, menus, and toolbars.
3. **Verify Active Tool & Property State:** Confirm that the target tool (e.g., Text, Brush, Line) and its active attributes (Primary Color `#000000`, Font Size, Stroke Width) are visibly selected before interacting with the workspace.
4. **Never Blindly Guess Coordinates:** Actions must target bounding box centers derived from real layout inspection, not hardcoded pixel offsets.
5. **Closed-Loop Visual Verification:** Every mutating action must be followed by a screen perception check to verify state change before claiming completion.
