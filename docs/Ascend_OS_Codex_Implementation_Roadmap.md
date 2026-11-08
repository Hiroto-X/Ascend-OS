# Ascend OS — Codex Implementation Roadmap



**Aligned specification:** Ascend OS Product Specification v5.7 (frozen)  

**Repository baseline:** Brand-new repository created from the official Tauri 2 + React + TypeScript + Vite scaffold  

**Target:** Windows desktop, single-user, offline-first  

**Development strategy:** Incremental, testable vertical slices  

**UI/UX:** Google Stitch work is a separate design input, integrated only after functional foundations exist.



---



## 1. Repository reset and starting point



This roadmap supersedes earlier implementation plans that assumed an existing Next.js application, a legacy-code audit, data migration, or an existing user-data store.



The repository is intentionally a **clean rebuild**. The generated Tauri 2 + React + TypeScript + Vite scaffold is the only implementation baseline. The scaffold has already been verified to launch: Vite loads, the React UI renders, and the Rust/Tauri application compiles and opens on Windows.



There is no legacy application code to preserve, audit, refactor, remove, import, or migrate. There is no pre-existing Ascend database, JSON store, user-data directory, configuration store, or backup to discover. Do not create a legacy migration plan or a backup of hypothetical data.



Normal safety requirements still apply once durable Ascend data exists: destructive schema changes, restore, import, export, and release upgrades must use the backup and recovery rules in Product Specification sections 18.6.6 and 18.6.13.



## 2. Mission and implementation strategy



Build Ascend OS as a reliable native desktop application through small, integrated increments.



Do not build the entire backend first and postpone the frontend. Establish backend foundations early, start the frontend shell in the same foundation phase, and then implement each workspace as a complete vertical slice.



A vertical slice includes:



- Versioned schema migration when durable data changes.

- Domain rules and Rust-side validation.

- Rust repository and application service.

- Typed Tauri command and, where useful, event contracts.

- A functional React interface using the real backend.

- Loading, empty, success, and error states.

- Tests for persistence, domain behavior, and critical user workflows.



Before approved Google Stitch designs are available, use clean, restrained functional scaffolding. Later, apply approved designs to the same working frontend. Do not build a throwaway prototype frontend.



## 3. Global technical rules



1\. Tauri 2 is the Windows desktop shell.

2\. The frontend is React + TypeScript + Vite. Do not introduce Next.js, SSR, a web server, or server-runtime assumptions.

3\. Rust owns domain logic, persistence, local system services, provider calls, and sensitive operations.

4\. SQLite is the authoritative local store; SQLx manages database access and ordered migrations.

5\. SQLite FTS5 is the initial full-text search engine.

6\. React never accesses SQLite or the filesystem directly.

7\. AI never accesses SQLite directly; AI tools call application services through the Tool Registry.

8\. TanStack Query manages backend data synchronization; Zustand is for ephemeral UI state only.

9\. Use stable IDs, normalized records, explicit relationship tables, and transactions.

10\. Do not introduce microservices, an external database server, a vector database, or another agent framework without an approved, demonstrated need.

11\. Keep the application buildable after every phase and preferably after every coherent slice.

12\. Run checks and report only checks actually run.

13\. Google Stitch is a design workflow, never a production runtime dependency.

14\. Do not implement unconfirmed ideas or resolve deferred decisions silently; raise them as decisions needed.



## 4. Phase 0 — Inspect and preserve the clean scaffold



**Purpose:** Understand the generated baseline before changing it; this is not a legacy audit.



Tasks:



- Inspect the repository root, frontend, `src-tauri`, manifests, Tauri configuration, and generated entry points.

- Record the scaffold's current commands, package manager, lockfile state, toolchain versions, and development/build status.

- Confirm the working tree and branch state before edits.

- Identify generated boilerplate that will be replaced gradually as Ascend's shell and bridge are introduced.

- Create a short baseline note in the implementation change or PR: clean scaffold, no legacy Ascend data, no import/migration work.

- Do not add application dependencies, production features, a database, or a migration merely to complete this phase.



Deliverables:



- A concise scaffold inventory and verified command baseline.

- A clear list of the generated files expected to change in Phase 1.



Exit criteria:



- The scaffold's ownership and startup path are understood.

- The app still launches after inspection.

- No legacy-system analysis, data backup, migration map, or reuse/refactor/replace matrix has been created.



## 5. Phase 1 — Foundation: application shell, typed bridge, and Settings slice



**Purpose:** Establish the real application foundation and prove it with the smallest durable end-to-end Settings slice while keeping the application runnable.



### Backend tasks



- Establish Rust module boundaries and a composition root in the existing Tauri crate.

- Define typed command, DTO, validation, and structured-error conventions.

- Add application data-directory resolution using Tauri's application directory APIs.

- Add SQLx, SQLite connection management, ordered migrations, foreign-key enforcement, WAL mode and a bounded busy timeout, with separate development, test, and production data directories.

- Choose and document the stable identifier convention used by durable records.

- Add only the initial Settings migration and schema needed by this vertical slice.

- Configure structured, redacted `tracing` diagnostics appropriate for the clean application.

- Add Settings domain, repository and application-service boundaries; persist settings only through Rust-owned SQLite access.

- Add narrow, typed Settings commands that validate input, return typed responses and map failures to structured errors.

- Define the central typed event convention; add an event only when a concrete use requires it.



### Frontend tasks



- Replace the generated screen with the Ascend application entry point.

- Establish the feature-based frontend structure: `app`, `shell`, `features`, `components`, `lib`, `state`, and `types`.

- Set up React Router, a basic application shell, placeholder destinations, settings navigation, and accessible navigation semantics.

- Add the typed bridge client as the sole frontend location that calls Tauri.

- Add TanStack Query and Zustand only after choosing and documenting the dependency versions; use them according to their separate ownership roles.

- Implement a small, real Settings UI path with first-run, loading, success, and error states. Do not leave mock data or browser storage once the commands exist.



### Integration and tests



- Exercise one React Settings screen that reads, creates and updates a supported setting through the typed bridge, then displays the validated Rust response.

- Add focused migration, persistence, validation, command and critical frontend-path tests using isolated temporary data directories.

- Verify application startup, shutdown, and safe error presentation.

- Verify that the supported setting persists after an application restart against the actual application-data database.



Exit criteria:



- The desktop app launches from the generated scaffold with the Ascend shell.

- React communicates with Rust through the typed bridge.

- SQLite initializes in the correct application-data directory, ordered migrations run successfully, and a supported setting can be created, read and updated through the real backend.

- The supported setting persists across restart.

- Typed backend events are integrated where the Settings slice needs them, and structured errors and redacted structured logging are in place.

- Formatting, linting, type checking, relevant tests, and a production build have been run and reported.



## 6. Phase 2 — Shared durable-data extensions



**Purpose:** Extend the proven durable-data foundation only when the next vertical slice requires a shared persistence capability.



Backend tasks:



- Extend transaction, repository, application-service, structured-error, pagination, filtering, and sorting patterns only as required by the next approved slice.

- Keep durable preferences and layout in the Settings service; do not create browser-storage persistence.



Frontend tasks:



- Apply the Phase 1 TanStack Query, mutation and UI-state patterns to the next approved vertical slice.



Exit criteria:



- New shared durable behavior is added only with the vertical slice that requires it and satisfies the checklist in `AGENTS.md`.



## 7. Phase 3 — Shared record, relationship, search, and file foundations



**Purpose:** Build reusable capabilities immediately before the first modules that need them.



Tasks:



- Implement shared record identity and the generalized many-to-many relationship model.

- Enforce endpoint validation, record/relationship types, duplicate prevention, transactional deletion cleanup, and integrity checks.

- Add FTS5 indexing infrastructure and lifecycle updates for searchable records.

- Establish shared backend query conventions and functional list/detail/search/filter/relationship UI primitives.

- Establish managed-file path validation and note-file synchronization foundations only when Notes work begins; do not pre-implement speculative file support.



Exit criteria:



- Relationship and deletion behavior is tested across at least two real record types.

- Search index updates are tested for create, edit, archive, move where applicable, and delete.

- Frontend components use backend services rather than a parallel in-memory model.



## 8. Phase 4 — Core workspaces as vertical slices



**Purpose:** Deliver usable features end-to-end, one coherent slice at a time.



Recommended order:



### 4A. Projects and relationships



- Project schema, lifecycle, metadata, custom-type foundation, relationship operations, functional list/detail UI, and persistence/lifecycle tests.



### 4B. Goals and milestones



- Goal and milestone records, approved progress methods and history, linked-record contribution rules, functional UI, and progress tests.



### 4C. Tasks



- Lists, tasks, subtasks, approved status/priority/date/tag fields, recurrence, functional list/board/calendar-ready data and detail UI, plus recurrence/completion tests.



### 4D. Habits and routines



- Habit schedules and occurrences; routine activities and sessions; skipped/missed/completed states; streak and consistency calculations; functional UI; schedule edge-case tests.



### 4E. Notes and knowledge



- Markdown files with stable IDs, SQLite metadata, links/backlinks, attachments, FTS5 indexing, functional editor/navigation, and synchronization/conflict tests.



### 4F. Calendar



- Internal scheduled-record projections, day/week/month queries, selected-day overview, functional calendar UI, and planned/completed/actual-work tests.



### 4G. Timers



- Pomodoro, Stopwatch, Countdown, and Deep Work state machines; persistent sessions/history; functional mini-timer; restart and duplicate-session tests.



### 4H. Notifications



- Scheduling, category, priority, quiet hours, deduplication/history, functional notification center/settings, and delivery/consolidation tests.



For every slice, finish backend/frontend integration before moving on. Build a shared service ahead of a slice only when that slice immediately needs it; do not accumulate backend-only modules.



## 9. Phase 5 — Windows background services and Activity Tracker



**Purpose:** Implement efficient, private Rust-managed services only after the core record model is stable.



Tasks:



- Define platform-service interfaces for foreground-application observation, idle detection, and any supported browser context.

- Implement Windows observation through suitable native APIs, never recurring PowerShell-process polling.

- Apply the product privacy rules: no keyboard capture, screenshots, typed content, browser scraping, or unapproved data collection.

- Implement deterministic classification, app rules, user correction, exclusions, and unassigned activity.

- Keep timers separate from observed activity; implement authorized attribution and session handling.

- Provide tracking state and timeline UI, resource-use tests, and long-duration stability tests.



Exit criteria:



- Idle, classification, attribution, exclusions, and restart behavior are testable.

- The UI remains responsive and activity records feed analytics correctly.



## 10. Phase 6 — Analytics and gamification



**Purpose:** Build trustworthy insights and progression from verified records.



Analytics:



- Define formulas, source records, time-zone handling, and exclusions.

- Aggregate/filter in SQL and backend services rather than loading all history into browser memory.

- Implement date-range reports, comparisons, functional charts/tables, and double-count tests.



Gamification:



- Implement an auditable XP ledger with unique source-event IDs.

- Add levels, achievements, and optional quests; preserve the ability to disable gamification.

- Prevent duplicate awards for linked/reopened work; add functional UI and integrity tests.



## 11. Phase 7 — Connections and integrations



**Purpose:** Add external capabilities after local behavior is stable.



Tasks:



- Define connector contracts, capabilities, provider DTOs, connection status, sync cursors, and conflict states.

- Implement Local Resources and canonical-path validation first, with execution-time Read/Write permission enforcement.

- Store credentials only in Windows secure credential storage through the keyring abstraction.

- Add bounded retry/backoff and clear recovery states.

- Implement GitHub after the connector framework is stable; add web research with attribution; add external calendar only after internal Calendar is stable.



Exit criteria:



- Permission checks occur at execution time, disconnect/revoke works, conflicts are visible and recoverable, and local operation remains usable offline.



## 12. Phase 8 — Ascend AI and Tool Registry



**Purpose:** Add the AI system after the application services it operates are stable.



Tasks:



- Implement the provider abstraction, beginning with the product-specified local Ollama baseline; retain the architecture needed for the later OpenAI-compatible provider described by the specification.

- Implement conversation and message persistence, Assistant/Agent/Researcher/Coach modes, response-style preferences, and long-term-memory rules.

- Retrieve workspace context through structured services, FTS5, relationships, and authorized memory—not direct database access.

- Implement the centralized Tool Registry with schemas, risk classes, structured results, execution-time permission checks, trusted approvals, cancellation, recovery, idempotency, and auditable runs.

- Add prompt-injection, tool-selection, schema-validation, authorization, approval, and interrupted-workflow tests.

- Connect the dedicated AI workspace and global panel to real application services.



Exit criteria:



- AI cannot access SQLite directly or bypass application services.

- Consequential actions require trusted user approval and provider failures do not break non-AI features.



## 13. Phase 9 — Google Stitch design integration



**Purpose:** Apply approved design work to the functional application.



Prerequisite: the user has completed and approved the relevant Google Stitch screens and flows.



Tasks:



- Extract approved patterns/tokens and build production primitives.

- Refine the existing shell and apply the designs workspace by workspace.

- Preserve backend contracts and domain behavior; keep logic out of presentation components.

- Verify keyboard navigation, accessibility, pane behavior, reduced motion, and loading/empty/error states.



## 14. Phase 10 — Backup, restore, hardening, and release



**Purpose:** Add recovery and release operations once durable user data and supported schema versions exist.



Tasks:



- Implement safe database-and-managed-file backup with manifest, integrity validation, recovery-copy restore, and configurable schedules.

- Test migrations from supported prior Ascend schema versions. These are future versions of this clean application—not imports from a former repository.

- Implement legacy import only if a supported external legacy source is later defined; it must be an explicit preview/mapping/error-report workflow and not an assumed project task.

- Test crash/restart, interrupted workflows, connector conflicts, filesystem validation, credential storage, tool abuse, offline behavior, large workspaces, startup/search/tracker performance, and long-running stability.

- Package Windows releases with Tauri's supported pipeline; validate installation, upgrades, update behavior, diagnostics, and known limitations.



Exit criteria:



- Backup/restore, upgrades, recovery, security, and reliability paths are tested against real Ascend data created by this application.

- A Windows package and upgrade path are validated.



## 15. Codex working protocol



For each task, Codex must:



1\. Read Product Specification v5.7, this roadmap, and `AGENTS.md`.

2\. Inspect only the relevant current scaffold/application files before editing.

3\. State the roadmap phase, intended vertical slice, assumptions, and likely files to change.

4\. Implement one coherent end-to-end increment.

5\. Add tests with the implementation.

6\. Run formatting, linting, type checking, relevant tests, and relevant build commands.

7\. Report changed files, behavior, commands actually run, results, decisions needed, and limitations.

8\. Update implementation documentation only when verified.



Codex must not:



- Treat this repository as a conversion of a former Next.js application.

- Perform a legacy audit, data inventory, migration map, import, backup, or code-reuse classification when no legacy artifact exists.

- Build a disconnected mock frontend after real services are available.

- Reset or delete generated scaffold files wholesale without first understanding their role and replacing them with the planned structure.

- Access SQLite from React or AI, add unapproved architectural dependencies, claim unrun checks passed, or change frozen product behavior silently.



## 16. First Codex task



Start with **Phase 0, then Phase 1 only**.



1\. Inspect the freshly generated Tauri 2 + React + TypeScript + Vite scaffold and verify its development and production command baseline.

2\. Replace the starter screen with the smallest accessible Ascend shell and a feature-based structure.

3\. Implement the Settings-first vertical slice: application data-directory resolution, SQLx/SQLite connection management, migrations, typed Settings commands, the real Settings screen, persistence/restart verification, and focused automated tests.

4\. Do not add persistence beyond the smallest Settings slice. Do not add backups, imports, legacy migration code, or data models for future workspaces.

5\. Run and report the relevant checks.



The first persisted vertical slice is **Settings in Phase 1**. Complete all Phase 1 exit criteria before starting Phase 2.



## 17. Definition of done



Ascend OS is ready for a production candidate when:



- Core workspaces use durable local persistence and correct cross-module relationships.

- Local files, timers, and activity tracking behave safely and privately.

- Connections enforce explicit permissions and recover from sync failures.

- AI uses the central Tool Registry and cannot bypass application services.

- Memory and workspace records remain distinct.

- Analytics and gamification are auditable.

- Backup, restore, and supported schema upgrades are tested.

- Approved Stitch designs are integrated consistently.

- Windows packaging, startup, updates, and offline operation are validated.

- Critical workflows have automated tests and documented limitations.



