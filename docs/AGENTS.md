# AGENTS.md — Ascend OS



Engineering instructions for AI coding agents (Codex and similar) working in this repository.

Read this file fully before starting any task. It tells you **how to build**; the Product Specification and Implementation Roadmap tell you **what to build and when**.



If a task, this file, the Product Specification or the Roadmap appear to conflict, **stop and report the conflict**. Do not resolve it silently.



---



## 0. Clean rebuild baseline



This is a **brand-new Ascend OS repository** created from the official Tauri 2 + React + TypeScript + Vite scaffold. The generated application has been verified to launch: Vite serves the frontend, React renders, and the Rust/Tauri desktop application compiles and opens on Windows.



There is no prior Ascend implementation, Next.js codebase, legacy database, JSON store, configuration store, user-data folder, audit report, migration plan or data backup in scope. Do **not** search for, classify, preserve, import, migrate or back up hypothetical legacy artifacts. Generated scaffold code is a starting point, not an old product implementation.



Replace the starter UI deliberately and incrementally as the active roadmap slice requires. Once this new application has durable user data, the migration, backup, recovery and import rules in this document and the Product Specification apply normally.



---



## 1. Project overview and technology stack



Ascend OS is a desktop-first, offline-first, single-user personal operating environment for goals, habits, tasks, projects, notes, timers, activity tracking, analytics and AI assistance. The initial platform is Windows.



The architecture is a **modular monolith**: one Tauri desktop application, one local SQLite database, and well-separated internal modules. The stack below is final. Do not substitute or add to it.



| Layer | Technology |

|---|---|

| Desktop runtime | Tauri 2 (Windows first) |

| Frontend | React + TypeScript + Vite |

| Routing | React Router |

| Server state | TanStack Query |

| Ephemeral UI state | Zustand |

| Backend | Rust (modular monolith, single crate initially) |

| Desktop bridge | Tauri commands + typed events |

| Database | SQLite, owned by Rust, accessed with SQLx and versioned migrations |

| Search | SQLite FTS5 (initially) |

| AI | Ollama initially, behind a provider abstraction; orchestration in Rust |

| Secrets | OS credential store via a Rust keyring abstraction |

| Logging | Rust `tracing`, structured, redacted file output |



Next.js is intentionally not used. There is no server-side rendering, no web server and no separate backend service.



---



## 2. Source-of-truth hierarchy



| Priority | Document | Authority over |

|---|---|---|

| 1 | **Ascend OS Product Specification v5.7** (frozen) | Product requirements, finalized product decisions, and the technical architecture baseline (Sections 18.6 and 18.7) |

| 2 | **Ascend OS Codex Implementation Roadmap** | Phases, sequencing and milestones: what to build now and in what order |

| 3 | **This file** | Engineering conventions: how to build it |

| 4 | Existing code and tests | Current state only; they never override documents 1–3 |



Rules:

- The Product Specification scope is frozen. Do not add, remove or reinterpret requirements.

- Work only on the phase and slice assigned to you. Do not build ahead of the Roadmap.

- The currently authorized Phase 1 slice includes the durable Settings vertical slice: application-data directory resolution, Rust-owned SQLite/SQLx persistence and migrations, typed bridge commands, a functional React Settings screen, restart-persistence verification, and focused tests. Do not begin Phase 2 until every Phase 1 exit criterion is met.

- Do not invent product requirements, defaults, copy or behavior. If something is unspecified, ask, or record it as a decision needed (see section 10). Do not guess.

- Spec Section 15 contains **unconfirmed brainstorm ideas**. Do not implement them.

- Spec Section 16 lists open decisions and Section 18.6.15 lists deferred architecture choices. Do not decide them implicitly through code. Raise them.

- Capabilities that depend on components that do not exist yet (image understanding, voice, PDF/DOCX parsing, browser-extension tab tracking) must not be faked or approximated.

- Do not edit the Product Specification or the Roadmap unless the task explicitly asks you to.

- Cite the relevant spec section in code comments or PR descriptions when implementing a non-obvious rule.



---



## 3. Architecture and dependency boundaries



### 3.1 Layers (Spec 18.6.2)



1\. **Presentation:** React pages, components, forms, navigation, visual state.

2\. **Desktop bridge:** typed Tauri commands and events; boundary validation; exposes only supported capabilities.

3\. **Application:** use cases and orchestration (create task, link project, complete habit, execute AI tool, restore backup).

4\. **Domain:** entities, value types, invariants, progress and relationship rules, domain events.

5\. **Infrastructure:** SQLite repositories, file system, provider clients, OS integration, logging, connectors.



Dependencies point **inward**: presentation → bridge → application → domain; infrastructure implements interfaces used by the application layer.



- The domain layer must not depend on React, Tauri, SQLx or any infrastructure detail.

- Repositories and external clients must not contain UI policy or business rules.

- The frontend never imports a database driver and never touches SQLite or the filesystem.



### 3.2 Backend modules (Spec 18.6.3)



`goals`, `projects`, `tasks`, `habits` (including routines), `calendar`, `notes`, `timers`, `activity`, `analytics`, `notifications`, `connections`, `ai`, `tool\_registry`, `files`, `settings`, `gamification`.



- Start with a **single Rust crate** and enforce module boundaries. Split into crates only for a concrete build, ownership or testing benefit, and say so in the PR.

- Cross-module operations go through **application services and explicit transactions**. Never read or write another module's tables from arbitrary code.

- A module may query another through a documented service or repository interface.

- Windows-specific behavior (startup launch, tray, global shortcuts, foreground-window observation) lives behind **platform service interfaces**.



### 3.3 Hard architecture rules



- **SQLite is the durable source of truth.** All durable operations go through Rust services.

- Do not introduce microservices, extra servers, background daemons outside the app, a second AI agent framework, a vector database or an external database (Spec 18.6.3, 18.6.6, 18.6.8).

- Do not duplicate domain logic in the frontend, in tools, or in repositories. One implementation, in the domain/application layers.



### 3.4 Vertical-slice checklist (applies to every phase)



Each feature or module is delivered as a complete slice. A slice is not done until all of these exist:



- [ ] Migration(s) for any schema change.

- [ ] Domain types and invariants.

- [ ] Application service(s) and repository code, with explicit transaction boundaries.

- [ ] Tauri command(s) with typed DTOs, input validation and structured errors, registered with the narrowest capability needed.

- [ ] TypeScript contract types kept in sync with the DTOs.

- [ ] Frontend feature code in `features/<name>/` with TanStack Query hooks, loading/empty/error states and accessible UI.

- [ ] FTS5 index updates, if the record is searchable.

- [ ] Relationship handling and deletion cleanup, if the record can be linked (see section 6).

- [ ] Tool Registry entries only if the spec requires AI access to the feature, with a risk class (section 8).

- [ ] Tests at each layer touched (section 9).

- [ ] No unrelated refactors mixed in.



---



## 4. Frontend engineering conventions



### 4.1 Structure (Spec 18.6.4)



```text

app/        startup, routing, providers, composition

shell/      sidebar, tabs, command palette, global search, contextual panel, notifications

features/   home, goals, projects, tasks, habits (incl. routines), calendar, notes,

            timers, activity, analytics, gamification, connections, ai, settings

components/ reusable visual primitives and shared interaction patterns only

lib/        typed desktop bridge client, formatting, utilities

state/      UI-only global state

types/      frontend-facing DTO/contract types

```



- Organize by feature. `components/` must not contain domain logic.

- `home` has no domain of its own: it composes widgets that read other features' data through the bridge.

- `gamification` and `habits` mirror their backend modules.



### 4.2 Rules



- TypeScript **strict mode**. No `any`, no unchecked type assertions on bridge responses, no `@ts-ignore` without a justified comment.

- **Only `lib/` talks to Tauri.** Features call typed bridge functions, never raw `invoke` or event APIs scattered across components.

- **TanStack Query owns backend data.** One query key convention per feature. After a mutation, return the updated record or invalidate the affected keys.

- **Zustand is for ephemeral UI state only** (open panels, selected tab, transient layout). It must never hold copies of goals, tasks, projects or any server record.

- Do not mirror query data into `useState`, Zustand or Context.

- **Durable user preferences and layout** are persisted through backend `settings` services. Do not use `localStorage`/`sessionStorage` for durable data.

- Subscribe to Tauri events in one central place. Events trigger query invalidation or small state updates. They are notifications, not a source of truth.

- Use React Router for workspace navigation. Keep route definitions in `app/`.

- Frontend validation is for user experience only. Rust always re-validates.

- Timestamps arrive in UTC. Convert to the configured time zone only for display and calendar calculations.

- Every screen handles loading, empty, error and offline-capable states. Surface backend error codes through safe, user-readable messages.

- Accessibility is a release criterion: keyboard operability, visible focus, labeled controls, sensible semantics.

- Never render untrusted content (notes, AI output, external content) as raw HTML. Do not use `dangerouslySetInnerHTML` with untrusted input.

- Google Stitch output is a **design input only**. Translate it into maintainable components and shared primitives. Never ship generated prototypes as application architecture.

- Do not add a UI component library or design-token system without an approved decision (section 13).

- Do not show a capability as available when its dependency is not configured (for example, voice or image features).



---



## 5. Rust backend engineering conventions



- Inside each module, keep **domain, application and infrastructure code in separate submodules**.

- **Tauri command handlers are thin:** deserialize, validate, call one application service, map to a DTO. No business logic and no SQL in handlers.

- Commands are **named use cases**. Never expose arbitrary SQL, generic filesystem access or shell/process execution to the frontend.

- DTOs have explicit fields and stable identifiers. Validate all command input in Rust, even if the frontend already did.

- Errors are **typed and structured** with a stable error code, a safe user-facing message and diagnostic context. Do not return raw error strings to the UI.

- No `unwrap()`, `expect()` or `panic!` in production paths. Use `Result` and propagate with context. Tests may unwrap.

- Application services own domain mutations and validation. They define transaction boundaries explicitly.

- Long-lived work (activity sampling, timers, scheduled notifications, sync, AI execution, indexing, backup scheduling) runs in **Rust-managed services** with explicit lifecycle and shutdown. Background work must be bounded, cancellable where appropriate, and must never block the UI thread or an async executor thread.

- Publish state changes through the shared event mechanism using typed events. Keep payloads small and versionable.

- Keep the domain layer platform-independent where practical.

- Logging uses `tracing` with structured fields and redaction. Never log secrets, tokens, raw credentials or sensitive content (note bodies, window titles, activity content) unless explicitly justified.

- Secrets are read and written only through the keyring abstraction (see section 7).

- Provider unavailability (for example Ollama not running) must never break non-AI features.

- Activity tracking rules (Spec 11.5, 11.7, 11.8):

  - Use the Windows `GetLastInputInfo` API for system idle detection. Do not install a global keyboard hook.

  - Never capture keystrokes, screenshots, or page, document or typed content.

  - Honor user exclusions and privacy settings.

  -   - The system uses a built-in application recognition registry to identify and categorize known software.

  - Classification and rule matching are **deterministic Rust code** and must work without any AI or cloud service.

  - For unclassified apps, the system performs background research, provides confidence-based suggestions after session boundaries, supports optional privacy-controlled cloud research, and requires explicit user approval for permanent rules.

  - Idle detection thresholds:

    - Interactive: 60 seconds.

    - Passive focused: 5 minutes.

    - Uncertain system idle: 5 minutes.

    - System idle overrides uncertain.



---



## 6. Database, migrations and data integrity



### 6.1 Ownership and access



- SQLite is owned by Rust. Access it only through SQLx from the infrastructure layer.

- The frontend and the AI must never query SQLite directly.

- Do not use a vector store or any other database for the initial release.



### 6.2 Migrations



- Use ordered, versioned SQLx migrations. **Never edit a migration that has been merged or applied.** Add a new migration.

- Migrations must be tested from representative prior **Ascend schema** versions.

- Take and validate a backup before any destructive migration of real user data. This future-data rule does not require a Phase 0 backup of a non-existent legacy application.

- Tests and development use separate data directories and never the user's real data.



### 6.3 Schema and query rules (Spec 18.6.6)



- Use one consistent stable globally unique ID strategy. Never use display names or paths as identity.

- Store all timestamps in **UTC**.

- Enable foreign-key enforcement. Use WAL mode where supported and tested. Set a bounded busy timeout and keep write transactions short.

- Wrap every operation that changes multiple related records in a **single transaction**.

- Add indexes for foreign keys, common filters, relationship lookups and date queries.

- Use normalized tables and explicit join tables for many-to-many relations.

- Never use string-built SQL from untrusted input. Use bound parameters.



### 6.4 Relationships and deletion (Spec 5, 18.6.6)



- Cross-record relationships use the generalized relationship table. SQLite cannot enforce those references, so **application services must**:

  - Validate that both endpoints exist and that the record and relationship types are valid, and prevent duplicates.

  - Route every permanent deletion through the **owning domain service**. In one transaction, it deletes the record, all relationships where it is source or target, and its FTS5 entries.

  - Delete **links only**. Never delete or alter the other connected records.

  - Keep relationships for archived or soft-deleted records.

  - Never show an orphaned relationship as valid. Include an integrity check that detects and logs repairs.

- User actions and AI tools use the **same** deletion path.

- Do not double-count time, progress or XP when a record is linked to several goals or projects.



### 6.5 Search



- Use FTS5 with explicit index behavior on create, edit, move, archive and delete. Update the index in the same transaction as the record change where possible.

- Search results carry the source record and its stable ID.



### 6.6 Files, notes and backups (Spec 18.6.7, 18.6.10, 18.6.13)



- Large binaries live outside SQLite in managed application storage. SQLite stores metadata, hashes and **relative** managed paths.

- Resolve storage locations with Tauri's application directory APIs. Never hard-code install paths or write under Program Files.

- Validate canonical paths and prevent path traversal outside granted directories. External folders are accessed only through explicit Local Resources grants.

- Notes are Markdown files with a stable ID in frontmatter, with SQLite holding metadata and indexing state. Follow the synchronization, reconciliation and conflict rules in Spec 18.6.10. Write files with atomic replacement. **Never silently overwrite or lose user edits.**

- File-system effects are not part of the SQLite transaction. Order and record them so startup reconciliation can recover from any failure.

- Never rewrite original activity observations. Store classification changes and corrections separately with timestamps (Spec 11.8).

- Back up using SQLite's supported backup mechanism or a transactionally safe snapshot. **Never copy a live database file blindly.** Restore validates the backup first and keeps a recovery copy.



---



## 7. Tauri security and permissions



- **Least-privilege capabilities.** Grant each window and webview only the specific permissions it needs. Document the need for every permission added. No wildcard or broad permission sets.

- **Restrictive scopes.** Scope any scoped permission (filesystem, URL) to the narrowest paths or targets. The frontend gets **no general filesystem access**. File access goes through Rust `files` services.

- **Restrictive CSP.** Load only bundled assets. No remote scripts, no `unsafe-eval`, no inline scripts without a documented need. AI-provider and connector requests are made from Rust, not from the webview.

- **No shell or process execution** and no broad OS capabilities exposed to the webview. Tray, global shortcuts, startup launch and foreground-window observation are exposed only through specific Rust platform services.

- **Sensitive operations stay in Rust:** database access, credential access, filesystem access outside managed storage, external network calls, activity observation and tool execution. Enforce authorization in Rust, never only in the UI.

- Treat any change to capabilities, scopes or the CSP as **security-relevant**: flag it in the PR and justify it.

- **Credentials:** store API keys and OAuth tokens only in the OS credential store through the keyring abstraction. Never put them in SQLite, connector records, logs, the frontend, config files or the repository.

- External transmission happens only when required by an enabled provider or integration, and it must be clear to the user. No behavioral telemetry. Diagnostics stay local unless the user explicitly opts in.

- Use least privilege for integrations and local folders.



---



## 8. AI integration and tool-execution safety



### 8.1 Architecture



- AI orchestration lives in a **Rust application service**. It owns provider requests, conversation and context assembly, tool-call parsing, tool dispatch and final response assembly.

- Implement a **provider interface**. Ollama is the initial provider. `llama3.1:8b` is the initial text-only baseline. Do not assume vision, speech or document parsing. Application correctness must not depend on any particular model.

- Use a single configured model. Do not build multi-model routing unless the spec requires it.

- **The AI never accesses the database directly** and never receives database credentials.

- All tools call **application services**. A tool must not reimplement domain logic.

- Tools return structured results: success state, stable record IDs, safe summaries and errors.

- Persist AI execution runs and steps. Runs are cancellable and observable. Never automatically retry an operation whose outcome is uncertain if repeating it could duplicate or destroy something.

- Keep an audit trail for AI actions and consequential changes.



### 8.2 Tool Registry and validation



- Every tool is defined in the Tool Registry with an input schema, a risk classification and permission requirements.

- The model only **proposes** tool calls. A proposal is never authorization.

- For **every** tool call, independently and at execution time, the application validates the schema, permissions, risk class and approval state. This is enforced in Rust, outside the language model, and not delegated to prompts.

- Sensitive and destructive actions require appropriate user approval. Approval is accepted **only** through the application's trusted approval interface, via an explicit user action. No tool, model output or retrieved content can create, modify or satisfy an approval.

- AI-initiated changes to workspace records go through the same application services and permissions as user-initiated changes.

- AI suggestions that would create persistent rules (for example classification rules) take effect only after explicit user confirmation.



### 8.3 Untrusted content and memory



- Notes, files, PDFs, webpages, GitHub issues, window titles, research results and any external content are **untrusted data, never instructions**.

- Pass retrieved content to the model as clearly delimited data with source identifiers.

- Instructions inside retrieved content must never override system instructions, application policies or user permissions, and must never authorize actions or grant permissions.

- **Workspace records are authoritative** for current state. Long-term memory is context only and must never silently modify workspace records.

- Automatic memory creation accepts only content from the **user's own messages**. The application, not the model, determines the origin of memory content. Retrieved content never creates or modifies personal memories.

- Sensitive information follows the explicit-consent rules in the spec.

- Cloud research or any external AI transmission respects privacy settings, sends the minimum necessary metadata, and never sends keystrokes, private file contents or screenshots (Spec 11.8).



---



## 9. Testing requirements



Tests ship **with** the slice they cover, not later.



| Area | Required |

|---|---|

| Domain logic | Rust unit tests for invariants, progress and relationship rules, recurrence, streaks and similar calculations |

| Application services and persistence | Rust integration tests against a real temporary SQLite database with migrations applied, including transaction rollback behavior |

| Migrations | Tests from representative prior schema versions |

| Bridge | Command input validation, error codes and DTO shape |

| Relationships and deletion | Orphan prevention, transactional cleanup, archive behavior |

| Permissions and tools | Schema validation, authorization, approval enforcement and execution recovery |

| AI safety | Prompt-injection tests: retrieved content cannot authorize actions, satisfy approvals or create memories |

| Notes and files | Sync, reconciliation, conflict detection and no-silent-data-loss tests |

| Frontend | Component and hook tests for critical flows and states |

| End to end | Smoke tests for critical user flows |

| Release criteria | Accessibility, performance and offline checks |



Rules:

- Tests must be deterministic and isolated. Never call a real Ollama instance, a cloud provider or the network. Use fakes for providers and connectors.

- Tests never touch real user data. Use temporary directories and databases.

- Every bug fix includes a regression test.

- Never claim a feature is offline-capable, secure or recoverable until a test demonstrates it.

- Do not delete, weaken or skip tests to make a build pass.



**Definition of done:** the slice checklist (3.4) is complete, formatting, linting, type checking and tests all pass, and the app builds.



---



## 10. Git and development workflow



- Work on one roadmap slice at a time, on its own branch, with focused pull requests.

- Make small, logical commits. Keep the main branch green.

- Before committing, run formatting, linting, type checking and the relevant tests.

- Never commit secrets, API keys, local databases, user data, logs or build artifacts. Commit lockfiles.

- Do not rewrite shared history or force-push shared branches.

- Do not edit merged migrations. Add new ones.

- Update contract types, tests and any affected documentation in the same change as the code.

- Do not mix unrelated refactors with feature work.

- A PR description states: what changed, the spec sections and roadmap milestone it implements, tests added, **security-relevant changes** (capabilities, CSP, permissions), new dependencies with justification, and any **decisions needed**.

- When the specification is silent or ambiguous, do not invent behavior. Implement the minimal safe option only if it is clearly reversible, mark it `TODO(decision)`, and list it in the PR.

- Pin and document toolchain versions. Keep development, test and production data directories separate.



---



## 11. Code quality and dependency management



**Quality**

- Rust: formatted with `rustfmt` and free of `clippy` warnings. TypeScript: strict, linted and formatted with the project's configured tools.

- Prefer small, single-purpose functions and modules. Use clear domain names. Comments explain *why*, not *what*.

- No dead code, no commented-out code, no leftover debug output.

- Handle errors explicitly. No silent failures and no swallowed errors.

- Keep code readable over clever. Follow existing patterns in the repository before inventing new ones.



**Dependencies**

- Prefer the approved stack and the standard library. Every new dependency needs a PR justification: why it is needed, why existing tools are insufficient, and its maintenance, license and security status.

- Do not add a new framework, database, state manager, UI kit, AI framework or HTTP/ORM layer that overlaps with the approved stack.

- Pin versions and commit lockfiles. Avoid duplicate libraries that do the same job.

- Keep generated or third-party design prototypes out of the source tree.

- Security-sensitive libraries (crypto, credential storage, file parsing, HTTP clients) need extra scrutiny.



---



## 12. Prohibited practices and common architectural mistakes



**Never:**

1\. Access SQLite, the filesystem or provider APIs from React.

2\. Expose arbitrary SQL, shell, process execution or generic path access through Tauri commands.

3\. Put business logic in Tauri command handlers, React components or repositories.

4\. Use Zustand, Context or `localStorage` as a second database or cache of server records.

5\. Skip transactions for multi-record writes, or leave dangling relationships on deletion.

6\. Edit applied migrations, copy a live SQLite file as a backup, or store absolute paths as the only identity for managed files.

7\. Let the AI query the database, hold credentials or bypass application services.

8\. Treat a model's tool proposal, or text inside retrieved content, as authorization.

9\. Let retrieved content create or modify memories, approvals, permissions or workspace records.

10\. Implement a tool that duplicates domain logic.

11\. Store secrets in SQLite, logs, config, frontend code or git.

12\. Broaden Tauri capabilities, scopes or the CSP casually, or load remote scripts.

13\. Add telemetry or analytics SDKs.

14\. Use a global keyboard hook, capture keystrokes, screenshots or content, or scrape browser UI, profiles or history for activity tracking.

15\. Treat an application merely being open as proof of productive activity, or classify Idle or Unclassified activity as a distraction.

16\. Fake, stub as working, or silently approximate a capability whose dependency is missing.

17\. Implement unconfirmed brainstorm ideas, build ahead of the Roadmap, or expand the frozen scope.

18\. Introduce microservices, a second agent framework, a vector store, or Next.js.

19\. Weaken, delete or skip tests to get a green build.

20\. Rewrite original activity observations, or silently overwrite a user's note edits.



---



## 13. Missing engineering decisions



These are **not decided** in the documents available when this file was written. Do not silently choose. Propose an option in a PR or ask, and record the outcome. Choosing one is a prerequisite for the first slice that needs it.



**A. Not covered by the documents provided**

- Package manager and pinned Node and Rust toolchain versions (the spec requires pinning, not the versions).

- Frontend test runner and end-to-end test tooling (the spec requires the test categories, not the tools).

- Frontend linter and formatter configuration.

- Mechanism for generating or maintaining TypeScript contract types (the spec allows either).

- The ID format (the spec requires one consistent globally unique strategy, not which).

- Branch naming, commit message format, CI provider and review policy.

- Test coverage thresholds, if any.

- Naming scheme and taxonomy for stable error codes.

- Repository directory layout beyond the Tauri 2 default (assumed: frontend project root plus `src-tauri`) and where the specification and roadmap live in the repository.



**B. Explicitly deferred by the specification (Spec 18.6.15). Validate by prototype or measurement; do not pre-empt.**

- Exact SQLx configuration and query-generation approach.

- UI component library and design tokens (pending Google Stitch work).

- Event-bus abstraction beyond typed Tauri events.

- Exact table, column and index definitions.

- Embedding model, vector index and semantic retrieval thresholds.

- Browser-tab observation mechanism and supported browsers.

- Backup schedule and retention defaults.

- Update distribution provider and code-signing setup.

- Telemetry policy (default remains local diagnostics only).



---



## 14. Starting instruction



Begin with Roadmap **Phase 0 and Phase 1 only**: inspect the fresh scaffold, establish the Ascend shell and typed bridge, and deliver the smallest durable Settings vertical slice.



Phase 1 includes Rust module boundaries, typed errors and structured logging, application-data directory resolution, SQLx-managed SQLite connection management, versioned migrations, a Settings repository and application service, Rust-side validation, typed Tauri commands and frontend bridge, a functional React Settings UI, real SQLite-backed persistence, and automated persistence and validation tests.

Phase 2 expands the shared durable-data foundations rather than introducing SQLite for the first time. It adds normalized domain schemas, applies the established stable-ID convention to shared records, generalized relationships, FTS5 infrastructure, and shared repository patterns only as required by the next approved vertical slice.

Do not begin a legacy audit, legacy-data migration, backup, imports, feature data models beyond the smallest Settings slice, or AI implementation until their corresponding roadmap phase.







