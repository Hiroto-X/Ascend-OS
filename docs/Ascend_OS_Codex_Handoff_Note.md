# Ascend OS — Clean Rebuild Handoff



## What changed



- The Product Specification v5.7 was inspected but deliberately not copied, edited, or rewritten. It remains the frozen product source of truth.

- The implementation roadmap now starts from a clean, verified Tauri 2 + React + TypeScript + Vite scaffold and places the first real Settings persistence slice in Phase 1.

- All instructions that assumed an existing Next.js application, completed legacy audit, reusable legacy code, user-data discovery, migration mapping, import, or initial backup have been removed.

- The roadmap now separates future Ascend schema upgrades and backup/recovery work from nonexistent legacy-data work.

- `AGENTS.md` now identifies the clean baseline, cites Product Specification v5.7, and prohibits treating the repository as a conversion project.



## How Codex begins



1\. Read Product Specification v5.7, the revised roadmap, and revised `AGENTS.md`.

2\. Complete Phase 0: inspect the generated scaffold and verify the current development/build command baseline. This is a scaffold inventory, not a legacy audit.

3\. Complete only Phase 1: replace the starter screen with the smallest accessible Ascend shell; establish the typed bridge; then deliver the smallest real Settings vertical slice through Rust-owned SQLite/SQLx persistence, migrations, validation, loading and error states.

4\. Run and report the relevant formatting, type, test, and build checks.



Do not add persistence beyond the smallest Settings slice, backups, import code, legacy migration code, or feature data models for future workspaces. The first persisted end-to-end slice is Settings in Phase 1.



