# Ascend OS — Product Specification

**Version:** 5.7  
**Status:** Product planning complete; specification scope frozen; implementation preparation underway  
**Purpose:** Single source of truth for product decisions, requirements and future architecture planning.

---

# 00. Product Vision

## 0.1 Product Identity

Ascend OS is an all-in-one personal operating system.

It is a desktop-first environment designed to connect personal development, productivity, learning, projects, knowledge management, activity tracking, analytics and AI assistance.

Its value comes from connecting these systems into one ecosystem rather than treating them as unrelated applications.

**Primary audience:** Programmers, developers, students and people interested in personal development and productivity.

## 0.2 Product Vision Statement

Ascend OS is a desktop-first, offline-first personal operating environment that unifies goals, habits, tasks, projects, notes, timers, activity tracking, analytics and AI assistance.

Rather than treating these as isolated productivity tools, Ascend connects them into one ecosystem where information flows between systems, progress becomes measurable, and users can understand and manage their personal workflows.

## 0.3 Core Product Principles

1. **Unified:** Features communicate and share relevant information.
2. **Personal:** The environment adapts to individual workflows.
3. **Measurable:** Progress is based on meaningful data.
4. **Intelligent:** AI understands the user's workspace and its records.
5. **Offline-first:** Core functionality works locally.
6. **Extensible:** New modules can be added without rewriting the application.
7. **Transparent:** Users understand how activity is tracked and metrics are calculated.

## 0.4 Product-Level Decisions

- Desktop-first application, initially targeting Windows.
- Single-user experience initially.
- Local data storage.
- Future possibility of connecting to a mobile application for accessibility.
- Hybrid AI model-provider strategy: Ollama and OpenAI API.
- Native desktop application experience rather than a generic website dashboard.

---

# 01. Application Shell

## 1.1 Workspace Model

**Decision: Hybrid workspace.**

The application combines:
- Persistent navigation sidebar.
- Customizable Home workspace.
- Dedicated module workspaces.
- Contextual panels.

## 1.2 Sidebar

**Decision: Collapsible sidebar.**

- Expanded navigation mode.
- Compact icon-only mode.
- Contains all primary application navigation.
- Settings positioned separately from functional navigation.
- Consistent navigation across modules.

## 1.3 Main Workspace

**Decision: Support both tabs and multiple panes.**

- Dedicated workspace for each module.
- Multiple panes where useful.
- Resizable contextual right panel.
- Inspect related information without unnecessarily leaving the current workspace.
- Preserve workspace state when practical.

## 1.4 Contextual Right Panel

**Decision: Resizable and independently accessible.**

The panel adapts to the current workspace.

Examples:
- Project workspace: related tasks, notes and project details.
- Goal workspace: milestones and associated records.
- Task workspace: linked projects, goals and notes.
- AI assistant: contextual conversations and assistance.

## 1.5 Global Navigation Capabilities

Planned:
- Search.
- Command palette.
- Notifications.
- Quick-create actions.
- Global AI access.
- Settings access.

Exact keyboard shortcuts and command definitions remain open.

---

# 02. Navigation Structure

## 2.1 Navigation Philosophy

The sidebar should retain the full navigation envisioned in the original Ascend OS brainstorm.

Major capabilities should be accessible as dedicated destinations.

## 2.2 Current Navigation Proposal

| Area | Navigation |
|---|---|
| Overview | Home |
| Personal development | Goals, Milestones, Habits, Routines, Gamification |
| Productivity | Tasks, Calendar, Timers |
| Workspace | Projects, Notes |
| Insights | Activity Tracker, Analytics |
| Intelligence | Ascend AI |
| System | Settings |

Exact grouping and ordering remain to be finalized against the original brainstorm.

Milestones are conceptually part of Goals, although their navigation presentation remains open.

Calendar (Section 07), Routines (Section 21) and Gamification (18.4) are explicit navigation destinations. They are placed within the existing groups above as a logical grouping, not a navigation redesign; their final ordering remains open together with the rest of the grouping. Detailed requirements for Tasks, Habits and Routines are in Sections 19, 20 and 21. The Gamification destination presents the progression system defined in 18.4 (XP, levels, achievements, quests, and the ability to inspect why XP or an achievement was awarded) and remains subject to the user's ability to disable gamification elements without disabling core tracking.

---

# 03. Ascend AI

## 3.1 Product Decision

Ascend AI has two access points:

1. A dedicated top-level workspace in the sidebar.
2. Global assistant access from anywhere in the application.

The interaction should resemble the integrated AI experience in VS Code.

## 3.2 Global Assistant

Planned capabilities:
- Keyboard shortcut access.
- Contextual right-panel access.
- Workspace-aware conversations.
- Relevant record inspection.
- Authorized application actions.
- Context-aware recommendations.

The assistant should understand the current workspace context without requiring the user to repeatedly explain it.

## 3.3 Dedicated AI Workspace

Planned capabilities:
- Full conversation interface.
- Conversation history.
- New conversations.
- Project-aware conversations.
- Agent activity and execution history.
- Tool permissions and approvals.
- Model and provider settings.
- AI memory and workspace context.

## 3.4 Architecture Principle

**One AI system, multiple entry points.**

The global assistant and dedicated AI workspace share:
- AI runtime.
- Conversation history and context.
- Tool execution system.
- Permission and confirmation system.
- Model-provider configuration.
- Shared application services.

They must not become independent AI implementations.

## 3.5 Workspace-Native AI Agent

Ascend AI is intended to be an agent capable of performing authorized actions across Ascend OS.

Examples:
- Create and configure projects.
- Create and organize goals.
- Generate milestones and tasks.
- Create and organize notes.
- Connect existing records.
- Update project information.
- Analyze activity and progress.
- Recommend and organize next actions.

The agent should use shared application services rather than maintaining a separate AI-only data system.

## 3.6 Agent Reliability and Permissions

- Distinguish verified information from assumptions.
- Never invent project progress or activity.
- Respect connection permissions.
- Use confirmation for consequential or destructive actions.
- Maintain execution history.
- Make completed actions and failures understandable to the user.

The detailed permission model and agent architecture remain to be designed.

---

## 3.7 Autonomy and Conversational Behavior — Confirmed Direction

Ascend AI will use an **agent with user approval** model.

It should be capable of taking useful action across the user's workspace, rather than functioning only as a question-and-answer chatbot. It may create, organize and update supported Ascend records and perform other authorized actions through shared application services.

The agent must remain conversational, responsive and collaborative. Its role includes:

- **Respond:** Understand and directly address the user's questions, instructions, corrections and follow-up messages.
- **Explain:** Explain concepts, workspace information, recommendations, plans and the reasoning behind proposed actions in clear language.
- **Suggest:** Identify relevant opportunities, useful next steps, potential improvements and contextual recommendations when appropriate.
- **Ask:** Ask focused clarifying questions when intent, requirements or important details are ambiguous or missing. It may ask the user to choose between meaningful alternatives before proceeding.
- **Act:** Carry out authorized tasks such as creating goals, projects, milestones, tasks and notes, organizing records, and updating workspace information.
- **Report:** Clearly communicate what it changed, what it did not change, any assumptions it made, and any errors or incomplete actions.

### Approval behavior

- Routine, reversible actions explicitly requested by the user may be carried out within the permissions already granted.
- The agent must request approval before consequential actions, destructive changes, sensitive operations or actions with meaningful external effects.
- It must never bypass global integration permissions or silently expand its access.
- When a request is ambiguous, it should ask rather than make a consequential guess.
- Approval prompts should describe the intended action and its relevant consequences clearly.
- The user should be able to inspect agent activity and action history.

The intended experience is a capable collaborator: it can answer, explain, suggest, ask and act—not an agent that blindly executes every instruction or a passive chatbot that only gives advice.

The exact classification of routine versus consequential actions, approval UI and autonomy controls remain to be designed.

---

## 3.8 Proactive Behavior — Confirmed Direction

Ascend AI should be **contextually proactive**, rather than strictly reactive or constantly interruptive.

It may use relevant workspace context and available activity/progress information to identify situations where a suggestion, reminder, explanation or question could help the user. Examples include:
- Noticing that a goal or project appears to have stalled.
- Identifying an upcoming deadline or milestone that may need attention.
- Suggesting a useful next task based on current goals and project state.
- Pointing out relevant patterns or potential improvements in the user's recorded activity.
- Asking whether the user wants help when an important issue or opportunity is identified.

### Proactivity principles

- Proactive messages must be relevant to the user's current goals, projects and activity—not generic engagement prompts.
- Avoid unnecessary interruptions, repetitive suggestions and excessive notifications.
- Prefer surfacing low-urgency suggestions in an appropriate workspace or notification surface; reserve interruptions for genuinely time-sensitive matters.
- Explain the context behind a suggestion so the user can understand why it appeared.
- Distinguish observed facts from estimates or inferences. For example, lack of recorded activity does not prove that the user made no progress outside Ascend.
- A suggestion does not authorize the agent to perform the suggested action. Existing permission and approval rules still apply.
- Users should be able to configure how proactive Ascend AI is and control or disable proactive suggestions.

The exact proactivity controls, triggers, frequency limits, priority rules and delivery surfaces remain to be designed.

---

## 3.9 Workspace Context and Long-Term Memory — Confirmed Direction

Ascend AI should understand the **entire authorized workspace and maintain useful long-term memory** across conversations.

### Workspace-wide context

Subject to the user's existing permissions and enabled features, the agent should be able to retrieve and reason across relevant Ascend information, including:
- Goals and milestones.
- Projects and their connected records.
- Tasks and lists.
- Habits and routines.
- Notes and knowledge.
- Calendar events and schedules.
- Timer sessions.
- Recorded activity and analytics.
- Relevant connections and integration data that the user has authorized.

The agent should use relationships between records to understand how the user's work fits together, rather than treating each module as an isolated source of information.

Workspace-wide access does not mean unrestricted access to every connected external service or local resource. The agent must respect the existing global connection permissions, local-resource authorization and any applicable exclusions.

### Long-term memory

Ascend AI should retain useful information across conversations so the user does not need to repeatedly explain stable preferences, ongoing plans, recurring workflows and previously established decisions.

Potential memory categories include:
- User preferences for how Ascend should behave.
- Long-running goals and areas of focus.
- Recurring routines and working preferences.
- Important decisions made in prior conversations.
- Useful context about ongoing projects.
- Explicitly provided instructions that should guide future assistance.

Memory should complement—not replace—the underlying workspace records. A remembered statement must not be treated as more authoritative than current, verified workspace data.

### Memory organization — Confirmed Direction

Long-term memories should be organized into understandable categories so users can review and manage them.

Initial categories:
- **Personal preferences:** Stable preferences about how the user wants Ascend AI and the application to behave.
- **Goals and ambitions:** Longer-term objectives and areas of focus.
- **Projects and ongoing work:** Useful context about active or recurring projects.
- **Learning and skills:** Learning interests, current study areas and skill-development context.
- **Important decisions:** Decisions and commitments that may matter in future conversations.
- **Routines and working patterns:** Recurring routines and stable ways of organizing work.

Organization behavior:
- Ascend AI should automatically suggest or assign the most relevant category when creating a memory.
- Users can edit a memory's category and content.
- A memory should not be forced into an inaccurate category; an uncategorized or other option should be available when needed.
- Categories are for organization and retrieval. They do not independently grant permissions or change the sensitivity of stored information.
- Users should be able to search and review memories across categories.

### Memory conflicts and updates — Confirmed Direction

When new information conflicts with an existing memory, Ascend AI should use the latest explicit user statement as the preferred current information, while preserving useful history where it adds context.

- Detect when a new statement appears to correct or supersede an existing memory.
- Prefer the user's latest explicit statement over older remembered preferences or assumptions.
- Update the current memory rather than retaining contradictory statements as equally current.
- Preserve relevant historical context, including when a preference or plan changed, where that history is useful.
- Do not treat a passing remark, hypothetical statement or uncertain inference as an explicit change.
- If it is unclear whether the user is changing a preference or describing a temporary exception, ask a concise clarification before changing a durable memory.
- Make memory updates inspectable and editable by the user.

### AI mode selection and switching — Confirmed Direction

Users should be able to select an operating mode directly from the AI conversation interface.

- **Default mode:** Assistant.
- **Mode selector:** Available within the conversation interface so users can see and choose the active mode.
- **In-conversation switching:** Users may change modes during an existing conversation without losing its history or relevant context.
- **Mode recommendations:** Ascend AI may recommend a different mode when it would better fit the user's request.
- **User control:** The AI must not switch modes automatically. The user decides whether to accept a recommendation.
- **Shared foundation:** Switching modes changes the task orientation and behavior, not the underlying AI identity, workspace access, memory, tools or permission boundaries.

The exact visual design and whether the selected mode persists across new conversations remain to be decided.

### Unified AI operating modes — Confirmed Direction

Ascend AI should provide selectable operating modes through one unified AI system, rather than separate independent assistants.

Initial modes:

- **Assistant:** Answers questions and helps with everyday work.
- **Agent:** Plans and executes tasks through authorized Ascend tools, following the established permission and approval model.
- **Researcher:** Investigates topics, gathers relevant information and produces structured findings.
- **Coach:** Helps users work toward goals, build habits and improve productivity.

### Shared foundation

All modes use the same underlying AI identity, conversation system, workspace context, long-term memory, provider/model configuration, tool registry, permission system and execution history.

Modes change the assistant's task orientation, default behavior and presentation—not the user's authorization boundaries.

### Mode boundaries

- A mode must not grant additional permissions or bypass approval requirements.
- The Agent mode follows the existing plan-and-approval rules for consequential multi-step actions.
- Researcher mode must distinguish sourced facts from inference and identify sources where applicable.
- Coach mode should support the user's stated goals without claiming authority over the user's decisions.
- Users may switch modes without losing conversation history or relevant context.
- The exact mode-selection UI, default mode, mode-specific instructions and whether custom modes will be supported remain to be designed.

### Context-aware response-style adaptation — Confirmed Direction

Ascend AI may adapt its communication style to the current task or conversation context, while retaining the user's configured response style as its default.

Examples:
- During code debugging, the assistant may use a more technical and analytical approach.
- During goal planning or difficult productivity periods, it may use a more encouraging approach.
- During quick factual requests, it may favor concise answers.
- During complex decisions, it may provide additional explanation and reasoning context.

### Adaptation boundaries

- The user's explicit style preference remains the default and takes priority over automatic adaptation.
- Adaptation should be subtle and relevant to the task, not a dramatic personality switch.
- The assistant must not infer that a user wants emotional encouragement merely because a task is difficult.
- The user should be able to override the adapted style at any time through an instruction or settings preference.
- Automatic adaptation changes presentation only; it must not change factual standards, transparency, permissions, approval requirements or safety behavior.
- The exact adaptation controls and whether users can disable adaptation globally remain to be designed.

### AI personality and response-style customization — Confirmed Direction

Users should be able to customize how Ascend AI communicates. Response style is a user-facing preference and should not change the agent's underlying reliability, permission or safety requirements.

The available style dimensions should include:

- **Conciseness:** concise responses or more detailed explanations.
- **Tone:** professional or casual.
- **Approach:** analytical or encouraging.

The interface may present these as simple presets rather than requiring users to configure every dimension independently. The selected style should apply consistently across AI entry points, including the global assistant and dedicated Ascend AI workspace.

### Behavioral invariants

- Style preferences affect wording, explanation depth and conversational tone—not factual standards.
- The AI must remain accurate, transparent about uncertainty and honest about what it has or has not done.
- The AI must not become less careful with permissions, approvals or consequential actions because a user selects a more casual or concise style.
- The assistant should still provide enough context to make important decisions and approval requests understandable, even when concise mode is selected.
- Users should be able to change their preference later; the exact presets, default and settings UI remain to be designed.

### Memory and workspace data separation — Confirmed Direction

Ascend AI must keep long-term conversational memory separate from authoritative Ascend workspace records.

**Long-term memory** stores useful context such as:
- Stable user preferences.
- Conversational context and background.
- Historical decisions and changes in preferences.
- Ongoing interests and working patterns that help personalize assistance.

**Workspace data** stores the actual application records, including:
- Goals and milestones.
- Projects and tasks.
- Habits and routines.
- Notes and knowledge.
- Calendar entries, timer sessions and recorded activity.
- Other structured records managed by Ascend modules.

### Authority and conflict rules

- Workspace records are the authoritative source for current application state, such as task status, goal progress, project details and recorded activity.
- Long-term memory provides context; it must not silently overwrite, modify or contradict current workspace records.
- When a memory conflicts with a current workspace record, use the workspace record for claims about current application state.
- When a user explicitly asks to change a workspace record, make that change through the appropriate application tool and permissions. Do not treat a memory update as a substitute.
- Historical conversational context may explain why a workspace record exists or how a decision evolved, but must be identified as historical context rather than current record state.
- The assistant should make the source of important information clear when answering: workspace record, long-term memory, current conversation or inference.

This separation is intended to prevent remembered statements from becoming accidental sources of truth for live productivity data.

### Memory source rules — Confirmed Direction

Long-term personal memory records what the user has told Ascend AI. It must not be populated from material that Ascend AI merely read on the user's behalf.

- Only information explicitly communicated by the user can be automatically considered for personal long-term memory.
- Retrieved content must never independently create or modify personal memories. Retrieved content has the meaning given in 3.11: notes, files, PDFs, webpages, GitHub issues, external services and other sources. Preferences or facts about the user that are inferred from retrieved content are never saved automatically.
- An instruction inside retrieved content to remember something is ignored as an instruction (3.11).
- Project information found in retrieved content remains sourced project context, identified by its source. It is not personal memory.
- If the user explicitly asks Ascend AI to remember information from a document or other retrieved source, it may save the requested information with its source identified. The user's explicit request is the authorization; the retrieved content cannot authorize itself.
- Sensitive information continues to follow the existing explicit-consent policy (see "User control and reliability" below). Nothing in these rules relaxes it, including requests to save sourced information.
- Workspace records remain authoritative (see "Authority and conflict rules" above). Saving a memory must never silently modify a goal, task, project or other workspace record. Changing a workspace record requires the appropriate tool, permissions and approvals.

| Information source | Memory behavior |
|---|---|
| User says, "I prefer concise answers." | May be saved as a personal preference |
| User says, "Remember that my project uses Rust." | May be saved, with appropriate project context |
| A PDF says, "The user prefers Python." | Never saved automatically as a user preference |
| A webpage instructs the AI to remember something | Ignored as an instruction |
| A GitHub issue contains project information | Kept as sourced project context, not personal memory |
| The user asks the AI to remember specific information from a document | May be saved as requested, with its source identified |

Saving remains subject to the existing selectivity rules below: not every statement is saved.

### User control and reliability

- Ascend AI may automatically save useful, non-sensitive information that the user has explicitly communicated in conversations (see "Memory source rules" above) when it is likely to improve future assistance, such as stable preferences, ongoing project context and explicit working decisions.
- Automatic memory creation should be selective: do not save every conversational detail, transient statement or speculative inference.
- Sensitive personal information must not be automatically saved as long-term memory; it requires an explicit user request or consent under the product's defined sensitive-data policy.
- Avoid duplicate memories and prefer updating or superseding outdated information when a newer statement clearly corrects it.
- Users must be able to inspect what Ascend AI remembers.
- Users must be able to correct inaccurate or outdated memories.
- Users must be able to request that information no longer be used or retained, subject to the product's defined deletion and data-retention behavior.
- The system should make it clear when information comes from workspace records, long-term memory, the current conversation or an inference.
- The agent must not present an uncertain inference as a verified fact.
- Memory must not override current user instructions or silently change workspace records.
- Memory and workspace retrieval must respect privacy controls and access permissions.

The detailed memory lifecycle, storage model, retention rules, deletion behavior, retrieval strategy, sensitive-data policy and user-interface design remain to be designed.

---

## 3.10 Retrieval Architecture — Confirmed Direction

Ascend OS will use a **hybrid retrieval architecture**:

- The Rust application backend owns database access, persistence and the trusted retrieval services.
- The AI orchestration layer decides what information is needed, which retrieval capabilities to invoke, and how to use the returned information.
- The AI layer accesses workspace information through controlled application tools and service interfaces. It must not bypass the backend and query the database directly.
- The same retrieval and permission rules apply whether the request originates from the global assistant or the dedicated Ascend AI workspace.

### Retrieval responsibilities

**Rust backend**
- Owns SQLite access and structured workspace queries.
- Provides controlled retrieval interfaces for application and AI use.
- Enforces authorization, local-resource access rules and applicable exclusions.
- Provides keyword-search capabilities and relationship-aware record retrieval.
- Returns relevant records with stable identifiers and source information.
- Supports freshness and consistency by retrieving current authoritative records when needed.

**AI orchestration layer**
- Interprets the user's request and determines what information is required.
- Chooses and coordinates appropriate retrieval tools.
- Can combine structured queries, text search, relationship traversal and semantic retrieval.
- Requests only the information needed to answer the current request.
- Organizes retrieved results for model context and reasoning.
- Distinguishes retrieved facts from memory, estimates and model-generated inference.

### Retrieval methods

The retrieval system should support multiple complementary methods:

1. **Structured queries:** Use application services and SQL-backed queries for exact records, statuses, dates, counts, progress and other structured information.
2. **Full-text search:** Use SQLite FTS5 or an equivalent local search capability to find matching words and phrases across indexed workspace content.
3. **Relationship-aware retrieval:** Follow relevant connections between records, such as goals, milestones, projects, tasks, habits and notes, without indiscriminately loading the entire graph.
4. **Semantic retrieval:** Add embeddings and vector similarity search to find conceptually related content even when the wording differs.
5. **Long-term memory retrieval:** Retrieve relevant user memory as a distinct source, keeping it distinguishable from current workspace records.

### Context construction and evidence

- Retrieval results should be filtered by permissions before being provided to the model.
- A context-building stage should select, rank and organize relevant results within model context limits.
- Retrieved information should retain source identifiers and enough provenance for the assistant to explain which records support its answer.
- Structured facts should be retrieved from authoritative records rather than inferred from loosely related text.
- Current database state should take precedence over stale summaries or remembered values.
- The assistant must communicate uncertainty and avoid presenting an inference as a verified fact.
- The system should avoid sending the entire database or unrelated records into model context.
- Retrieved content is untrusted data and is never treated as instructions. Context assembly should present it as clearly delimited data with its source identifiers. See "Prompt-injection and untrusted-content protections" in 3.11.

### Implementation sequence — planned

The recommended implementation order is:
1. Establish trusted application services and structured SQLite retrieval.
2. Add local full-text search.
3. Add relationship-aware retrieval.
4. Build context assembly, relevance selection and source attribution.
5. Add a local embedding model and semantic retrieval.
6. Combine keyword and semantic results through hybrid ranking.
7. Evaluate retrieval quality, permission enforcement, freshness and performance.

Semantic retrieval is a complementary capability, not a replacement for structured queries. A separate vector database is not an initial requirement; storage and indexing choices should be evaluated against the expected scale and local-first constraints.

### Still to be designed

- Exact retrieval tool contracts and API boundaries.
- Whether context assembly is performed entirely in Rust or split between backend and AI orchestration.
- Embedding model selection and local inference integration.
- Vector storage and indexing strategy.
- Ranking, filtering, chunking and context-budget policies.
- Incremental indexing, updates and deletion consistency.
- Retrieval evaluation datasets and quality metrics.
- Caching and freshness guarantees.

---

## 3.11 Centralized AI Tool Registry — Confirmed Direction

Ascend AI will execute application actions through a **centralized Tool Registry**, rather than through unrestricted database access or arbitrary application internals.

### Tool definition and discovery

- Each supported action is represented by a named tool with a clear description, typed input schema and structured result.
- The AI orchestrator can discover eligible tools and choose tools relevant to the user's request.
- The registry exposes only tools and capabilities available under the user's effective permissions.
- Tool inputs must be validated before execution, and failures must return actionable error information.
- Tools use shared application services and the trusted Rust backend for record access and mutations.

### Authorization and approval

- Every tool invocation is subject to authorization and capability checks at execution time, not only when the tool is listed.
- Routine, reversible actions explicitly requested by the user may proceed within existing permissions.
- Consequential, destructive, sensitive or externally impactful actions require explicit approval before execution.
- Approval requests must identify the intended operation and meaningful consequences.
- The orchestrator must not bypass approvals by invoking lower-level tools or composing equivalent actions.
- Connected-service tools respect global integration Read/Write permissions and local-resource authorization.

### Execution and transparency

- Tools return structured success, partial-success or failure results, with identifiers of affected records when relevant.
- Execution history records the tool, outcome and relevant action metadata without exposing secrets.
- The assistant reports what changed and distinguishes completed actions from planned or failed actions.
- Multi-step workflows should not assume later steps succeeded merely because earlier ones did.
- The precise transaction, rollback, retry, idempotency and batch-approval behavior remains to be designed.

### Prompt-injection and untrusted-content protections — Non-negotiable

The Tool Registry and AI orchestration must enforce the following rules. They apply to every conversation, mode, entry point and tool, and they complement the authorization and approval rules above.

- **Retrieved content is untrusted data.** Content retrieved from notes, files, PDFs, webpages, GitHub issues, external services and any other source is data to be read and reasoned about, never a source of instructions.
- **Instructions inside retrieved content never take effect.** They must not override system instructions, application policies or user permissions.
- **Retrieved content cannot authorize anything.** It cannot authorize an action, grant or expand permissions, or satisfy an approval requirement.
- **Retrieved content cannot create or modify personal memory.** Only information explicitly communicated by the user can be automatically considered for long-term memory (3.9, Memory source rules).
- **Only the user can approve.** Only an explicit user action through the application's trusted approval interface can authorize an action that requires approval.
- **Every tool call is independently validated at execution time** by the application, regardless of how the call was proposed or what the model's context contained.
- **Enforcement lives outside the language model.** Tool permissions, input schemas, risk classifications and approval requirements are enforced by the application (Rust backend and Tool Registry), not by prompt instructions or model behavior.
- **Proposing is not authorizing.** The model proposing an action does not mean the action is authorized.

These rules apply equally to the Plan → Approve → Execute workflow (3.12): approval of a plan is given only by the user through the trusted approval interface, never by content the agent has read. Implementation requirements are in 18.6.8 and 18.7.

### Initial tool families — Proposed, not yet individually approved

- Goals and milestones: create, read, update and connect records.
- Projects: create, configure, inspect and organize.
- Tasks and habits: create, inspect, update and organize.
- Notes and knowledge: create, search, read and edit.
- Calendar and timers: inspect schedules and manage supported sessions.
- Activity and analytics: read verified metrics and summaries.
- Relationships: link, unlink and inspect connected records.
- Integrations: read or act through authorized capabilities only.

The initial tool catalogue, per-tool risk classifications, approval experience and execution architecture remain open decisions.

---

## 3.12 Multi-Step Task Execution — Confirmed Direction

For multi-step requests, Ascend AI will follow a **Plan → Approve → Execute** workflow when the requested operation is consequential or requires meaningful approval.

### Planning
- Interpret the user's desired outcome and identify the required actions.
- Break the request into an ordered plan of discrete tool operations.
- Identify dependencies between steps and any information that is still missing.
- Ask focused clarification questions before presenting an execution plan if important requirements are ambiguous.
- Check tool availability, current permissions and relevant workspace state while preparing the plan.

### Review and approval
- Present a clear, understandable summary of the proposed outcome and actions before executing consequential multi-step work.
- Make meaningful changes, external effects and destructive operations visible in the plan.
- Allow the user to approve or reject the proposed execution.
- Do not treat approval for one plan as blanket permission for unrelated future actions.
- If the plan changes materially after approval, request approval for the changed consequential actions.

### Execution
- Execute approved operations through the centralized Tool Registry and shared application services.
- Respect dependencies and execute steps in the required order.
- Simple, reversible actions explicitly requested by the user may still execute directly within existing permissions; they do not need an unnecessary planning ceremony.
- Routine actions within a larger approved plan may be grouped where safe, but each tool operation must still enforce its own authorization and validation.
- Never claim that the entire workflow succeeded based only on the first successful operation.

### Results and recovery — Confirmed Direction

- Report completed, partially completed, failed and skipped steps clearly.
- Identify records created or changed and provide links or identifiers where the application supports them.
- Explain errors and any remaining work.
- Retry transient failures automatically within bounded retry limits; do not retry indefinitely.
- Preserve the execution state and results of completed steps when a later step fails.
- Resume from the first safely recoverable unfinished step where possible, rather than blindly replaying the entire workflow.
- Use idempotency protections or verify current state before retrying an operation that could otherwise create duplicate records or repeat external effects.
- Do not automatically repeat an operation when its outcome is uncertain and repetition could cause duplicate, destructive or externally consequential effects. Verify the outcome or ask the user.
- If recovery requires a changed plan, new permissions or another consequential decision, pause and ask the user.
- If the model or provider becomes unavailable, preserve the workflow state and explain whether the failure occurred during planning, tool execution or result reporting.
- Never claim that an operation succeeded unless its result was confirmed by the application service.

The detailed retry limits, persistence format for workflow state, rollback and undo capabilities, and exact resume interface remain to be designed.

This approach provides a review point for meaningful multi-step changes while keeping simple everyday interactions responsive.

---

## 3.13 Conversation Organization — Confirmed Direction

Ascend AI will use a **hybrid conversation system** that supports general conversations as well as conversations connected to specific workspace entities.

### Conversation types

- **General conversations:** Everyday questions, brainstorming and discussions that are not tied to one particular workspace record.
- **Project conversations:** Conversations associated with a project and able to use that project's authorized context.
- **Goal conversations:** Conversations associated with a goal and its relevant milestones and connected records.
- **Custom conversations:** User-created conversations for other topics or workflows.

### Conversation management

- Users can create multiple conversations.
- Users can rename, search and archive conversations.
- Each conversation retains its own message history and relevant conversation context.
- Project- and goal-linked conversations should provide a clear route back to the associated workspace record.
- Archiving a conversation should not delete the project, goal or other records linked to it.

### Shared memory and contextual boundaries

- Long-term AI memory can be available across conversations, subject to user controls and privacy settings.
- A conversation's local history remains distinct from shared long-term memory.
- Project- or goal-specific conversations should prioritize the relevant entity and its connected records, rather than automatically treating every workspace record as equally relevant.
- The AI may retrieve broader workspace context when needed and authorized, but should make the source and relevance of that context clear.
- Linking a conversation to a project or goal does not grant additional permissions to external services or local resources.
- Users should be able to understand which project or goal a conversation is associated with and change or remove that association where appropriate.

### Still to be designed

- Conversation folders, tags and organization beyond the initial types.
- Whether a conversation can be linked to multiple projects or goals.
- Conversation export, deletion and retention behavior.
- Context selection and switching within a conversation.
- Attachments, voice input and other conversation input modes.
- Conversation sharing or collaboration, if supported in a future multi-user version.

---

## 3.14 Conversation Input and Output Capabilities — Confirmed Direction

Ascend AI conversations should support the following capabilities:

- **File uploads:** Support relevant documents and files, including PDF, DOCX, CSV, images and code files, subject to format and size limits to be defined. PDF and DOCX understanding depends on document extraction and parsing infrastructure (see "Capability dependencies and phasing" below).
- **Image understanding:** Allow users to provide images for visual analysis and discussion. Requires a vision-capable model; the `llama3.1:8b` text-model baseline does not provide it (see "Capability dependencies and phasing" below).
- **Voice input and spoken responses:** Support speech input and audio responses as conversation modes. Requires speech-recognition and text-to-speech components (see "Capability dependencies and phasing" below).
- **Code presentation:** Render code blocks with syntax highlighting and convenient copying.
- **Rich responses:** Support Markdown, tables and interactive elements such as checklists where appropriate.
- **Generated artifacts:** Allow Ascend AI to create useful outputs such as reports, plans and documents.

These are product capabilities to support; exact file limits, parsing behavior, voice providers, streaming behavior, artifact formats, storage and implementation priorities remain to be designed.

### Capability dependencies and phasing

Several of these capabilities depend on components that the initial text-model baseline (`llama3.1:8b`, see 3.15) does not provide. They remain planned capabilities with explicit dependencies; this specification does not select a model or component for them and does not promise that they will work with the current baseline.

| Capability | Technical dependency | Status |
|---|---|---|
| Rich responses (Markdown, tables, checklists) and code presentation | Application UI rendering; no additional model capability required | Planned; compatible with the text-model baseline |
| Generated artifacts | Text generation by the configured model, plus artifact formats and storage (to be designed) | Planned; format and storage design pending |
| File uploads (attachment and local storage) | Managed local attachment storage (18.6.7); format and size limits to be defined | Planned |
| Text-based file understanding (for example CSV and code files) | Reading file content as text through application file services, within context limits | Planned; limits to be defined |
| PDF and DOCX understanding | Document extraction and parsing infrastructure that converts these files into text or structured content before it is used as model context; this is not provided by the language model | Future phase; infrastructure not yet designed or selected |
| Image understanding | A vision-capable model; `llama3.1:8b` is a text model and does not provide image understanding | Future phase; no model selected |
| Voice input | A speech-recognition (speech-to-text) component | Future phase; component not yet selected |
| Spoken responses | A text-to-speech component | Future phase; component not yet selected |

Rules:
- A dependency-gated capability must not be presented in the interface as available until its dependency is configured and actually available, consistent with the provider and model rules in 3.15.
- "Future phase" indicates dependency-gated sequencing only; scheduling belongs to the separate Codex Implementation Roadmap.
- Adding one of these capabilities does not change the permission, approval or memory rules that apply to all Ascend AI interactions.

---

## 3.15 AI Model and Provider Strategy — Confirmed Direction

Ascend AI should support a provider abstraction that can accommodate multiple providers over time, while the initial setup remains intentionally simple.

### Initial configuration
- The initial configured provider is Ollama.
- The user's current baseline model is `llama3.1:8b`, the initial text-model baseline. It is a text-only model and does not provide image understanding; image understanding requires a separate vision-capable model (see 3.14).
- The initial application must work with one configured model; multiple models or API keys are not prerequisites.
- The same configured model is used for conversations and agent operations unless the user changes the configuration.
- Do not require automatic routing between multiple models in the initial version.

### Future provider flexibility
- Keep provider integration modular so additional providers, including cloud providers such as the OpenAI API, can be configured later.
- Provider support does not imply that the user has credentials, subscriptions or model access configured.
- Model selection and provider configuration should reflect only models and credentials actually available to the user.
- If the configured provider or model is unavailable, communicate the failure clearly rather than silently pretending the request succeeded.

### Model evaluation before switching
- Keep `llama3.1:8b` as the current baseline.
- Evaluate alternatives such as Qwen2.5 7B Instruct and Qwen3 8B before choosing to replace the baseline.
- Compare candidates using the same Ascend-specific evaluation tasks: conversation quality, instruction following, structured output reliability, tool selection and calling, multi-step workflow performance, error recovery, response speed and resource consumption.
- Treat public benchmark results as useful evidence, not a substitute for testing the actual Ollama model builds and quantizations on the user's hardware.
- Do not assume that a model's parameter count alone predicts its suitability.

### Hardware and runtime considerations
- Account for the user's current RTX 4050 Laptop GPU with 6GB VRAM and 16GB system RAM when evaluating local models.
- Benchmark actual quantization, context length, GPU offloading, memory use and latency.
- Do not assume an 8B model will fit entirely in GPU memory.

### Architecture principle
The application must not depend on a particular model for correctness. The LLM interprets requests and proposes actions; application services enforce schemas, permissions, validation, approvals and execution outcomes.

Automatic multi-model routing, provider fallback policies, model-specific tool adapters and advanced model management remain future decisions.

---

# 04. Home Workspace

## 4.1 Product Decision

Home is a customizable personal command center, not a conventional dashboard filled with generic cards.

## 4.2 Layout Model

**Decision: Structured customizable layout.**

- Rearrangeable widgets and sections.
- Predefined column arrangements.
- Adjustable widget sizes.
- Add and remove widgets.
- Persistent layout preferences.
- Sensible default layout.

Unrestricted free-form drag-anywhere positioning is not part of the initial design.

## 4.3 Potential Home Content

- Today's priorities and tasks.
- Active goals.
- Upcoming milestones.
- Habit progress.
- Current timer and focus session.
- Recent projects.
- Activity summary.
- Quick access to AI.

The final widget catalogue and default layout remain open.

---

# 05. Connected Records and Relationships

## 5.1 Core Product Decision

Ascend OS consists of independent modules that can connect to one another.

**Many-to-many relationships are a fundamental product requirement.**

There should be no arbitrary one-to-one or one-to-many restrictions.

## 5.2 Relationship Examples

- One task can connect to multiple goals and projects.
- One goal can connect to multiple tasks, milestones, habits and notes.
- One note can be relevant to multiple projects.
- One habit can support multiple goals.
- One project can contribute to multiple goals.
- One record can have relationships with multiple record types.

## 5.3 Independence

Records do not need to belong to another record.

Examples:
- Tasks can exist independently of projects.
- Habits can exist independently of goals.
- Notes can exist independently of projects.
- Projects can exist without linked goals.

## 5.4 Relationship Types

Proposed semantics:

| Type | Meaning |
|---|---|
| Supports | Helps achieve another objective |
| Contributes to | Counts toward progress under defined rules |
| Depends on | Requires another record |
| References | Provides useful context |
| Related to | General association |

Exact relationship taxonomy and directionality remain open.

## 5.5 Relationship Metadata

Potential metadata:
- Relationship type.
- Creation date.
- Optional description.
- Direction, where meaningful.

Identical relationships should not be duplicated accidentally. Different relationship types between the same two records may be allowed.

## 5.6 Navigation Requirements

Every record should support:
- Viewing related items.
- Linking existing records.
- Creating and linking new records.
- Searching across modules.
- Quick preview.
- Opening in the current workspace.
- Opening in a split pane.
- Removing a relationship without deleting either record.

## 5.7 Progress and Gamification Safeguards

Relationships must be separate from progress calculations.

Connecting one task to three goals must not automatically produce three independent XP awards.

Progress calculations and event-based XP awarding need explicit rules.

---

# 06. Goals and Milestones

## 6.1 Goals — Confirmed Functionality

- Create short-term and long-term goals.
- Goal title, description, category and optional target deadline.
- Goal statuses: Not Started, In Progress, On Hold, Completed and Archived.
- Manual or automatically calculated progress.
- Measurable targets (for example, completing a target number of projects).
- Connect goals to multiple projects, habits, tasks, notes and milestones.
- Goal progress history and statistics.
- Optional parent goals and subgoals.

## 6.2 Milestones — Confirmed Functionality

- Create multiple milestones within a goal.
- Each milestone has a title, description and optional deadline.
- Milestone statuses: Pending, In Progress and Completed.
- Connect tasks, projects and notes to milestones.
- Support manual milestone completion and completion based on linked required tasks.
- Milestone dependencies, where one stage follows another.
- Visual timeline showing completed and upcoming milestones.

## 6.3 Goal and Milestone Progress Principles

- Completing a connected task does not automatically complete the entire goal.
- Goal progress follows the goal's selected tracking method.
- Milestones represent meaningful stages within a goal and provide visibility into progress.
- Goal progress, milestone progress and task completion remain related but distinct.

## 6.4 Persistent Progress Widget

- Retain the planned optional Windows desktop/taskbar progress widget.
- It can show the current long-term goal, overall progress and next milestone.
- Exact Windows integration and implementation are subject to technical feasibility.

# 07. Calendar

## 7.1 Calendar Views — Confirmed

- Day, week and month views.
- Calendar displays relevant scheduled items and activity from connected Ascend modules.
- Users can select a date to inspect that day's information.

## 7.2 Selected-Day Overview — Confirmed

When a user selects a date, the calendar should show a consolidated view of that day, including:

- Tasks scheduled for that date, with their completion state.
- Habits scheduled for that date, showing whether each was completed, skipped or missed as applicable.
- Completed work and other relevant daily records.
- Project activity, but only when activity was actually recorded for that project on the selected date. Projects with no recorded activity that day should not be shown as if work occurred.

## 7.3 Individual Consistency and History — Confirmed

- Users should be able to inspect consistency individually for habits and other trackable items over time.
- The calendar should make it possible to distinguish what was planned, what was completed and what was actually worked on.
- A selected date should reflect recorded data; the absence of a project activity record must not be presented as project work.

# 08. Notes and Knowledge Workspace

## 8.1 Product Direction — Confirmed

- Notes & Knowledge is a full workspace, inspired by Obsidian's flexible, interconnected knowledge-management approach.
- Notes use Markdown as their underlying format.
- Notes are stored locally as Markdown files and remain accessible offline.
- Users can organize notes into folders and nested folders, similar to a local vault-style structure.
- The editing experience should provide a polished visual interface; users should not need to write raw Markdown to format notes.

## 8.2 Note Editing and Content — Confirmed

- Rich visual formatting for titles, headings, text and emphasis.
- Code blocks with appropriate code formatting and highlighting.
- Checklists and structured content.
- Images embedded in notes.
- File attachments and other embedded content.
- Markdown compatibility and portability should be preserved.

## 8.3 Connected Knowledge — Confirmed

- Notes can link to other notes.
- Backlinks and connected-note discovery.
- Notes can connect to projects and other Ascend records using the shared relationship system.
- Connected notes and projects should be presented visually, inspired by Obsidian's graph and linked-knowledge experiences.
- A visual graph should help users explore relationships between notes and projects.

## 8.4 Organization and Discovery — Planned

- Folder and nested-folder organization.
- Tags and searchable note content.
- Search and navigation across the knowledge workspace.
- Visual exploration of connected notes and projects.

Exact editor interactions, graph controls, attachment management and advanced knowledge-management capabilities can be refined during UI/UX and implementation planning.

# 09. Timers Workspace

## 9.1 Timer Types — Confirmed

Ascend includes four timer modes in one unified Timers workspace:

- **Pomodoro:** Work and break intervals, customizable work/break durations, configurable long breaks and session history.
- **Stopwatch:** Start, pause and resume, lap tracking and elapsed-time recording.
- **Countdown:** Custom duration, start, pause, reset and completion notifications.
- **Deep Work:** Dedicated focus sessions, session duration, elapsed time, focus history and statistics.

## 9.2 Connections — Confirmed

- Timer sessions can connect to tasks and projects.
- Sessions can be associated with goals and habits.
- Recorded time can contribute to project and task time tracking.
- Timer sessions are included in Analytics.
- Timer history should be accessible from relevant tasks and projects.

## 9.3 Global Timer Access — Confirmed

- Provide persistent mini-timer access from anywhere in Ascend.
- The timer continues running when users switch workspaces or minimize the application.

## 9.4 Time Tracking Principles — Confirmed

- Timer sessions are explicitly started by the user.
- Activity Tracker independently observes computer activity.
- Timer records and observed activity may be associated, but starting a timer alone does not prove that the user was actively working.
- Keep manually recorded focus sessions distinct from observed activity so productivity statistics are not inflated.

# 10. Projects Workspace

## 10.1 Product Decision: General-Purpose and Development Projects

Projects support both general-purpose and software-development workflows.

Examples:
- Software: Personal AI Assistant.
- Learning: Master Python and DSA.
- Personal: Build a YouTube channel.
- Research: Explore local AI models.
- Content creation: Produce educational videos.

All use the same underlying Project system.

## 10.2 Project Identity

A project is an independent workspace for organizing and tracking a larger piece of work.

It can connect to:
- Tasks.
- Goals and milestones.
- Habits.
- Notes.
- Files and directories.
- Timers.
- Observed activity.
- External application data.
- Relevant events and resources.

Connected records remain managed by their respective systems.

## 10.3 Universal Project Context

Project tracking must not be limited to GitHub or software-development tools.

Projects should be able to connect to relevant application data and activity from supported sources.

Potential sources:
- Desktop applications.
- Local directories and files.
- External services and platforms.
- Internal Ascend modules.
- Timers and observed activity.
- Relevant events and records.

The system must be extensible so new integrations can be added without redesigning the core Project model.

## 10.4 Developer-Specific Context

Potential information:
- Technology stack.
- GitHub repositories.
- Local project directories.
- Commit history.
- Branch information.
- Development activity.
- Relevant IDE and development-tool activity.

These are specialized project-context capabilities, not requirements for every project.

GitHub is one possible integration, not a mandatory project requirement.

## 10.5 Project Types

**Decision: Projects support selectable and customizable types.**

Potential built-in types:
- Development.
- Learning.
- Career.
- Content Creation.
- Research.
- Personal.
- Custom user-created types.

Users should be able to create their own project types.

Project types help determine:
- Relevant metadata fields.
- Suggested project structure.
- Useful sections.
- Relevant connection suggestions.
- Appropriate defaults and templates.

Project types should not create entirely separate project systems. They customize the same underlying Project model.

## 10.6 Project Connections

**Decision: Connections are managed globally through Settings and selected individually for projects.**

Expected workflow:

1. User opens global Settings.
2. User connects supported applications and services.
3. Ascend maintains a catalogue of available connections.
4. User creates or edits a project.
5. User selects which available connections are relevant to that project.
6. The selected connections provide project-specific context and data.

Examples:
- A development project selects GitHub, a local repository and relevant IDE activity.
- A learning project selects learning resources, notes and a study directory.
- A content project selects relevant media tools, files and publishing services.

Connections should be reusable across multiple projects.

A project must not automatically receive access to every globally connected service.

## 10.7 Custom Project Fields

Projects should support type-specific fields.

Examples:
- Development: tech stack, repository, branch.
- Learning: subject, learning resources, study target.
- Content Creation: content format, publishing platform, assets.
- Research: research question, references, source material.

Users should be able to define their own project types and associated fields.

The precise custom-field builder and field types remain open.

## 10.8 AI-Assisted Project Creation

**Decision: Ascend AI should be capable of creating and configuring projects from natural-language descriptions.**

Example request:

"Create a project for learning full-stack development. Connect my learning notes, local code directory and relevant development tools. Give it milestones for frontend, backend and deployment."

Expected agent workflow:

1. Understand the user's project description.
2. Identify the appropriate project type.
3. Determine relevant project fields.
4. Identify relevant available connections.
5. Propose an appropriate project structure.
6. Create the project and its initial records.
7. Establish the requested relationships.
8. Report what was created and which connections were configured.

The user should also be able to request changes conversationally after creation.

### Connection and Permission Rules

- The agent may use already-authorized connections within their granted permissions.
- It must not silently connect a new external service.
- Missing connections should be clearly identified.
- New authorization requires user action.
- Consequential actions should follow the application's confirmation rules.
- The agent should report unsuccessful or incomplete configuration honestly.

### AI Project Configuration

The agent should be able to configure:
- Project type.
- Relevant fields.
- Project sections.
- Initial milestones.
- Initial tasks.
- Related notes.
- Existing record relationships.
- Relevant available connections.

The exact degree of automatic configuration versus confirmation remains to be refined.

## 10.9 Project Workspace Sections

Proposed sections:

| Section | Responsibility |
|---|---|
| Overview | Description, status, priority, deadline and progress |
| Tasks | Linked tasks and task creation |
| Notes | Linked notes and documentation |
| Activity | Time invested, observed activity and timeline |
| Insights | Progress trends and project analytics |
| Connections | Connected applications and resources |
| Project Settings | Metadata, project type and preferences |

These are proposed sections; their exact layout and interactions remain open.

## 10.10 Project Metadata

From the original brainstorm:
- Description.
- Status.
- Priority.
- Deadline.
- Tech stack.
- Repository.
- Local directory.
- Related goals.
- Tasks.
- Notes.
- Time spent.
- Commits.
- Timeline.
- Project analytics.

The model should accommodate universal metadata and optional type-specific context.

## 10.11 Project Lifecycle

Proposed statuses:

| Status | Meaning |
|---|---|
| Planning | Still being planned |
| Active | Currently being worked on |
| On hold | Temporarily paused |
| Completed | Successfully finished |
| Archived | Retained for reference |

Start date, deadline, priority and optional completion date should be supported.

## 10.12 Project Progress

Project progress must use explicit rules.

It should not simply be calculated as completed tasks divided by total tasks in every situation.

The progress model must account for relevant milestones, task weights or other explicitly selected criteria.

## 10.13 Project Activity Attribution

Project-linked activity should have an identifiable source and an explicit or sufficiently reliable association with the project.

Merely having an application open does not necessarily establish that activity belongs to a particular project.

Activity attribution, user confirmation, automatic detection and privacy controls require further design.

---


## 10.14 Global Connections Settings

### Product decision

Connections will have a dedicated section inside Settings, inspired by the Connections interface shown in the user's reference screenshot.

It should feel like a native application settings workspace, not a separate marketplace or a collection of integration cards scattered throughout the application.

### Navigation structure

Settings → Connections

Connections contains three confirmed views:

#### 1. Discover

- Browse supported applications and services.
- Search available integrations.
- Explore integration categories.
- Understand what capabilities each integration provides.
- Initiate authorization or configuration.

#### 2. Connected

- View already authorized applications and services.
- Manage existing connections.
- Review permissions and capabilities.
- Check connection status.
- Disconnect services.
- Access integration-specific settings.

#### 3. Local Resources

- Manage local directories and folders authorized for Ascend OS.
- Review which local resources are accessible.
- Add or revoke access to directories.
- Manage machine-level resources separately from external service authorization.

### Connection categories

Proposed categories:

- Development.
- Files and Storage.
- Productivity.
- Learning.
- Content Creation.
- Communication.
- Other.

The catalogue should support search and filtering.

These categories are organizational suggestions, not a commitment to support every listed service.

### Integration information

Each integration should communicate:

- Name and description.
- Connection status.
- Available capabilities.
- Required permissions.
- Connection and disconnection controls.
- Configuration options.
- Last synchronization or connection status, where applicable.

### Global connection versus project assignment

These are separate operations.

**Global authorization:**

The user connects an application or authorizes a local resource through Settings → Connections.

**Project assignment:**

The user selects which already-authorized services, accounts, repositories, folders or resources are relevant to an individual project.

A global connection must not automatically give every project access to all available data.

### AI integration

Ascend AI should be able to:

- Discover available integrations.
- Explain their capabilities.
- Recommend connections based on a project description.
- Identify missing connections.
- Request authorization through the appropriate user-controlled flow.
- Assign existing authorized connections to projects.
- Configure project context using available permissions.
- Report exactly what it successfully configured.

The agent must not silently authorize new services or expand permissions.

### Security and privacy requirements

- External service authorization must be explicit.
- Local resource access must be explicitly granted.
- Users must be able to review and revoke access.
- Project-level resource assignment must remain distinct from global authorization.
- Integrations should follow least-privilege principles.
- Sensitive credentials must not be exposed to ordinary project records or conversation history.

### Permission model — confirmed product decision

Read and write permissions must be separate, user-controlled permissions.

- Users can grant read access without granting write access.
- Users can grant or revoke write access independently, where an integration supports it.
- Permissions are configured by the user, not assumed from the fact that an integration is connected.
- The application and AI agent must respect the granted permission scope.
- Write operations that are consequential or destructive must follow the application's confirmation rules.
- The interface must make the requested and granted capabilities understandable before authorization.

The exact permission granularity (per integration, account, resource, project, or individual capability) remains to be designed. The product requirement is that read and write access are separate and controlled by the user.

### Permission configuration granularity — confirmed product decision

**Decision: Support both integration-level defaults and capability-level permissions.**

Users should be able to choose the permission configuration that matches how much access they want to grant:

- **Broad access:** A user can enable Read and Write for all supported capabilities of an integration, where the service supports those permissions.
- **Read-only access:** A user can enable Read while leaving Write disabled.
- **Selective access:** A user can grant Read or Write only for selected capabilities.
- **Mixed access:** A user can grant Read for some capabilities and Write for others.

The permission interface should make broad choices quick while still allowing detailed per-capability control. The available controls and permission names will depend on the specific application or service; Ascend OS must not display unsupported permissions as if they were available.

Integration-level settings act as convenient defaults. More specific capability permissions determine the effective access when a user customizes individual capabilities.

The effective permission state must be visible and understandable to the user and enforced consistently for normal application actions and Ascend AI agent tool calls.

### Still to be designed

- Exact integration catalogue.
- Initial supported integrations.
- OAuth and authorization architecture.
- Local directory permission handling.
- Integration-specific capability definitions.
- Synchronization and background refresh behavior.
- Error handling and connection recovery.
- Credential storage.
- Per-project permission granularity.

---

# 11. Activity Tracker

## 11.1 Product Direction — Planned

Activity Tracker automatically records computer activity to provide reliable information about application usage, focus, idle periods and project work.

## 11.2 Activity Detection — Confirmed Direction

Track both foreground application activity and browser-level activity, including websites and individual browser tabs where technically supported.

Relevant signals may include:
- Foreground application.
- Window title and document context.
- Browser website and individual tab context.
- Keyboard interaction activity, without recording typed content.
- Mouse interaction activity.
- Idle duration.
- Local project directory or other project context where available.

Activity classification should combine relevant signals rather than relying on application names alone.

**Browser-tab tracking dependency — Confirmed Direction**

- Detailed website and individual-tab tracking requires a browser extension or another explicitly supported integration. Without one, browser activity is limited to what the operating system exposes about the foreground window (for example the application and window title).
- Browser-extension-based tracking is a **deferred capability** and is not part of the initial implementation. The mechanism and supported browser coverage remain open (see 18.6.15 and Section 16).
- Implementations must not invent fragile or unsupported techniques to obtain browser-tab data, such as scraping browser interface elements, reading browser profile or history files, or intercepting input.
- Any future browser integration must be explicit, user-authorized, honor activity exclusions and privacy controls (11.6), and never record page content or typed content.

## 11.3 Activity Classification and History — Planned

- Categorize activity such as coding, learning, research, productivity, browsing, entertainment, communication, distraction and other, using the layered classification model in 11.8 (activity category and focus classification are separate properties, and idle is an engagement state rather than a category).
- Allow users to customize application categories, including assigning categories to applications and creating custom categories (11.8).
- Maintain activity sessions and historical records.
- Provide a visual timeline of application usage, active periods and idle periods.
- Allow users to inspect and correct incorrectly classified or associated sessions.
- Keep unassigned activity visible separately rather than guessing its project.

## 11.4 Project-Aware Activity — Planned

- Associate observed activity with relevant projects where reliable context is available.
- Support local project directories and relevant application context.
- Distinguish between multiple projects used in the same application where possible.
- Show time spent per project.
- Permit manual correction of incorrect project associations.
- Opening an application or project alone does not prove active work.

## 11.5 Accuracy Rules — Confirmed Direction

- Detect interaction activity without recording actual keyboard content.
- Detect idle periods and exclude idle time from productive activity.
- Opening an application alone does not establish that the user is actively working.
- Prolonged inactivity should pause active tracking sessions, following the idle-handling rules in 11.7.
- Timer sessions and observed activity remain distinct records; starting a timer alone does not prove active work.

## 11.6 Privacy and User Control — Planned

- Users can enable, disable, pause and configure tracking.
- Users can exclude applications or activity sources.
- Activity records are stored locally in keeping with Ascend's offline-first direction.
- Users can inspect and correct recorded activity.

## 11.7 Idle Detection and Engagement Classification — Confirmed Direction

Idle handling uses two separate layers. Activity is classified into one of three engagement modes — Interactive, Passive Focused or Uncertain — and each mode has its own rule. One universal idle threshold is not applied to every activity. A lack of keyboard input does not by itself mean the user is inactive.

### System-level idle detection

- System idle state is measured as time since the last user input on the machine, using the Windows `GetLastInputInfo` API behind the platform service interface (18.6.9).
- Do not implement a global keyboard hook, or any mechanism that observes keystrokes, for idle detection. Typed content is never captured (11.5).
- System idle detection is category-agnostic. It reports how long it has been since the last system input; it does not decide whether the user is working or what the activity is.

### Category-specific engagement classification

Activity classification (11.3) determines which engagement mode applies when the system idle signal is interpreted.

| Engagement mode | Examples | Rule | Initial default (configurable) |
|---|---|---|---|
| Interactive | Coding; actively navigating applications | No input for the threshold and no other engagement evidence → pause active tracking | 60 seconds |
| Passive Focused | Reading documentation; studying PDFs; watching educational tutorials; reviewing technical material | No input for the grace period and no other engagement evidence → pause active tracking | 5 minutes |
| Uncertain | Unknown applications; insufficient context; ambiguous activity | No productivity credit; time is recorded separately as uncertain | Not applicable |

- **Interactive:** when time since last input exceeds the interactive threshold and no other engagement evidence is present, active tracking pauses and the inactive time is excluded from verified active time (11.5, 18.3).
- **Passive Focused:** continuous keyboard activity is not required. No input within the grace period does not make the activity inactive. Once the grace period has elapsed with no engagement evidence, active tracking pauses and the time is treated as idle under the same exclusion rule.
- **Uncertain:**
  - The activity is not automatically classified as productive or idle.
  - The time is kept separately marked as uncertain.
  - It is excluded from verified active-time and productivity calculations.
  - The user can review and correct it (11.3).
- **Mode identification rule:** when the application cannot confidently identify the engagement mode, it must classify the activity as Uncertain. It must not guess Interactive or Passive Focused. This avoids applying the short Interactive threshold to unknown applications, which would wrongly penalize legitimate reading, while also avoiding inflated productivity statistics.

### Engagement evidence

The following observable signals help determine whether the user is still engaged and which engagement mode applies:
- Foreground application and window context.
- Browser tab or document context where available (detailed tab context depends on 11.2 and is deferred).
- Recent user interaction, including scrolling, navigation, page changes and document interaction.
- Reliable project context (11.4) and other observable signals.

Rules:
- Observable engagement evidence counts as recent interaction for the applicable threshold or grace period.
- An application being open or in the foreground does not by itself prove productive activity. Foreground context identifies the kind of activity and which rule applies; it does not establish active work beyond the applicable threshold or grace period.
- Signals record that interaction occurred, never keystrokes or content (11.5). Exclusions and privacy controls (11.6) apply.

### Activity states

- **Active:** the activity is classified as Interactive or Passive Focused and engagement evidence exists within the applicable threshold or grace period. Only this time counts as verified active time.
- **Idle:** the activity is classified as Interactive or Passive Focused and system idle time has exceeded the applicable threshold or grace period with no engagement evidence. Excluded from verified active time.
- **Uncertain:** the engagement mode, category or project cannot be confidently identified (for example an unknown application, insufficient context, conflicting signals or ambiguous activity). Productivity is not invented. The time is neither productive nor idle by default, is kept visible separately (11.3), is excluded from verified active-time and productivity calculations, and can be reviewed and corrected by the user.
- Verified active time is the basis for productivity calculations. If the user corrects an Uncertain period, the corrected classification flows into analytics under the existing correction rules (18.3).

### Related rules

- Timer sessions remain explicit user intent and a separate record (9.4, 11.5). A running timer is not engagement evidence, and the passive grace period does not apply to timer durations. Observed active time and timer duration must not be treated as equivalent (12.6).
- All thresholds are configurable in Settings → Activity Tracking (14.2): the Interactive threshold and the Passive Focused grace period.
- Users can inspect and correct classifications (11.3); corrections flow into downstream analytics (18.3). The interface should make clear why a period was classified as active, idle or uncertain.
- Engagement state (Active, Idle, Uncertain) is separate from activity category and focus classification, which are defined in 11.8. Idle is never an application category and is never automatically a distraction.

### Still to be designed

- Exact signal weighting and how document-interaction signals are obtained for specific applications.
- Accounting for the threshold interval itself (whether time before a pause is counted).
- Default engagement-mode mapping for each activity category.
- The confidence criteria for deciding that an engagement mode is identified rather than Uncertain.
- How system-level idle periods interact with Uncertain time (for example whether prolonged system idle during Uncertain activity is recorded as idle).

## 11.8 Activity Classification Intelligence — Final Decisions

The decisions in this section are final and frozen as of v5.5. Any change requires an explicit decision recorded under Section 17. The calibration items at the end of the section are values to be tuned during implementation and testing, not open product decisions.

**Guiding principle:** Ascend AI helps users classify their digital environment. It does not decide what counts as productive for them. The user has final authority over persistent classifications.

Ascend does not judge an application as productive or distracting from its name. Observed activity is described by separate properties that are determined independently.

### Layers

1. **Activity signals:** application, window, website (where available), interaction and project context (11.2).
2. **Application identity:** the application being used (for example VS Code or Chrome), identified by its name, publisher and executable information.
3. **Engagement state:** Active, Idle or Uncertain, determined by the engagement rules in 11.7.
4. **Activity category:** what kind of activity it is.
5. **Focus classification:** how the activity relates to the user's focus.
6. **User corrections and Ascend AI suggestions:** the AI suggests classifications and asks for confirmation; the user confirms, edits or dismisses; user-approved rules are remembered and applied to future activity.

Activity category, focus classification and engagement state are independent properties of the same recorded activity. Changing one does not silently change another.

### Unclassified applications

- When Ascend detects an application with no existing classification rule, the application is marked **Unclassified** and its usage is recorded, along with its application identity, process information, window title and engagement state.
- Ascend does not immediately decide whether an unclassified application is productive or distracting. **Unclassified never means Distraction.**
- Unclassified is the default for activity category and for focus classification until the application or activity is classified. It is different from the category Other, which means the user classified the activity as not fitting any other category.
- Because the engagement mode of an unrecognized application cannot be confidently identified, its time remains Uncertain under 11.7: it is recorded and visible, receives no productivity credit and is excluded from verified active time until it is classified or corrected. Input activity in an unclassified application is still recorded as engagement evidence.
- A user-confirmed classification can help the application identify the engagement mode. The mapping from category to engagement mode is part of the design item in 11.7.
- Users can manually classify an application at any time.

### Activity categories

- Default categories: Coding, Learning, Research, Productivity, Communication, Entertainment, Browsing, Distraction and Other.
- Users can assign any category to an application and can create custom categories.
- Distraction is a user-assignable category. It is distinct from the focus classification **Distracting**: an application is Distracting because of its focus classification, which depends on the rules and context below, and not because its name or type suggests it.
- Entertainment is not Distraction. Entertainment is an activity category and is not automatically distracting.
- "Learning" replaces "studying" as the category name used in earlier versions of this specification.

### Focus classification

| Value | Meaning |
|---|---|
| Focus-related | Activity that supports the user's current focus or work |
| Neutral | Activity that neither supports nor undermines the user's current focus |
| Distracting | Activity that pulls away from an active focus |
| Unclassified | Not yet determined |

**Default focus classification per category**

| Activity category | Default focus classification |
|---|---|
| Coding | Focus-related |
| Learning | Focus-related |
| Research | Focus-related |
| Productivity | Focus-related |
| Communication | Neutral |
| Entertainment | Neutral |
| Browsing | Neutral (a browser is a tool, not inherently productive or distracting; more specific website rules and user-defined classifications override this default; see Browsers) |
| Distraction | Distracting |
| Other | Unclassified |
| Unclassified | Unclassified |

Rules:
- These are defaults, not permanent classifications.
- Users can customize categories and focus classifications.
- Application-specific user rules override category defaults.
- Context can influence suggestions but must not silently rewrite user-defined rules.
- Entertainment is not automatically Distracting, and Unclassified activity is not automatically Distracting.
- Idle is an independent engagement state. It is not an activity category and not a focus classification.
- The same application can serve different purposes depending on context. For example, YouTube can be Learning when used for a programming tutorial, Entertainment when used for leisure viewing, and potentially Distracting when unrelated videos are watched during an explicitly started study session.

### Active focus context

Ascend must not assume that an application is distracting simply because it is not normally associated with studying or focused work.

An active focus or study context can be established only through explicit user designation:
1. A manually started focus or study timer.
2. An explicitly selected active task or project.
3. A user-started focus session.
4. Another clearly user-designated focus context.

Rules:
- Only reliable, explicitly established context is used for automatic distraction decisions.
- If no reliable focus context exists, an activity is not automatically labeled Distracting.
- A running timer may provide context for this judgment but does not itself prove active work (11.5).
- Ascend AI may suggest a classification but must not invent the user's intention.
- Example: watching YouTube during a manually started study session may be potentially distracting, while watching a programming tutorial may actually be Learning.

### Idle is an engagement state

- Idle is neither an application category nor automatically a distraction. A person taking a break is not necessarily distracted.
- Idle time is excluded from active-work duration and from any XP or progression credit that depends on observed activity time (18.4).
- Analytics may show idle duration separately. It must not be mixed into distraction time (12.6).

### Examples (illustrative)

| Situation | Activity category | Focus classification | Engagement state |
|---|---|---|---|
| Coding in VS Code | Coding | Focus-related | Active |
| Reading documentation | Learning | Focus-related | Active |
| Watching gaming videos during leisure time | Entertainment | Neutral (not automatically Distracting) | Active |
| Unrelated browsing during an explicitly started study session | Browsing | Distracting | Active |
| PC left unattended | Not applicable | Not applicable | Idle |
| Unknown application with insufficient context | Unclassified | Unclassified | Uncertain (11.7) |

These examples illustrate how the properties combine. They are not a fixed mapping from applications to values.

### Browsers

- A browser is not itself a category. Chrome, for example, is not automatically classified as Learning or Entertainment, because one browser can contain several kinds of activity.
- Browser-tab-level classification requires an authorized browser extension (11.2). Without one, Ascend uses only the application and window metadata that the operating system exposes and marks the confidence of the resulting classification accordingly.
- A recognized browser's application-wide classification is the category Browsing with the default focus classification Neutral. Unidentified browsing activity is not automatically labeled productive or distracting.
- When confidence in a more specific classification (for example Learning or Entertainment) is low, the activity stays in Browsing rather than being guessed. Its engagement state still follows 11.7 and is Uncertain when the engagement mode cannot be confidently identified.
- Researching documentation can be Focus-related and entertainment browsing can be Neutral or potentially Distracting depending on context; these are reached through website rules, user-defined classifications or explicit focus context, not by assuming from the browser.

### Classification confidence

Ascend AI must evaluate classification confidence using multiple signals rather than relying on an application name alone. Signals can include:
- Application identity, publisher and executable information.
- Window titles and available application metadata.
- User-provided application descriptions.
- Public research about the application.
- Usage patterns across multiple sessions.
- Active task, project or focus-session context (see Active focus context).
- Existing user-defined classification rules.

| Confidence | Behavior |
|---|---|
| High | Prepare a classification suggestion for the user |
| Medium | Gather more evidence or ask the user for clarification when appropriate |
| Low | Keep the application Unclassified |

- Confidence thresholds initially use configurable values and are calibrated through testing.
- High confidence does not mean automatic permanent classification. AI suggestions require user confirmation before becoming persistent rules.
- Ascend AI must record the evidence and reasoning behind each suggestion, and the suggestion shows its basis to the user.
- The user can review, correct and override any classification (11.3). The application remembers the user's classification and applies it to future activity.
- Each classification also records how it was determined (user-assigned, user-approved AI suggestion, rule match, or inferred from available metadata) and its confidence.
- Researching an application and determining what the user is doing inside it are different questions. Research can establish what an application is (for example, an IDE). Window metadata and session context can suggest what the current activity is (for example, coding). Neither is treated as absolute proof.

### When suggestions are prepared

- Background investigation begins when an unknown application is detected and has been used for a meaningful amount of time. Incidental or transient applications are not researched.
- Ascend gathers evidence across sessions instead of suggesting a classification immediately.
- **Default suggestion threshold: 3 separate sessions.** This is a default and not a hard requirement.
- If sufficient evidence is available earlier, Ascend AI may prepare a suggestion after the current session ends.
- A suggestion is prepared only when confidence is High (see Classification confidence).
- Suggestions are presented without interrupting the user's work: after the session ends or when the user has been inactive for a while, consistent with the proactivity principles in 3.8 (the user can control or disable proactive suggestions). A suggestion is not held back unnecessarily; it does not appear hours or days after the session.
- Dismissed suggestions are tracked and do not reappear unless meaningful new evidence emerges. Ascend does not prompt after every session.
- A user's manual classification always overrides a suggestion.

### Unclassified App Intelligence workflow

This workflow is part of the Activity Intelligence subsystem. It is not a separate AI agent.

1. **Unknown application detected.** Record its identity and permitted metadata.
2. **Background AI investigation.** Research the application, gather context and evaluate evidence.
3. **Confidence evaluation.** High, Medium or Low.
4. **Session-end evaluation.** Prepare a suggestion when sufficient evidence exists.
5. **User decision.** Accept, Edit or Dismiss. The suggestion shows the application, the confidence, the suggested activity category and focus classification, and the basis for the suggestion.
6. **Persistent classification rule.** The confirmed classification becomes a user-approved rule, is applied to future activity, and can optionally be applied to historical records (see Retroactive reclassification).

Rules:
- **Local first:** local detection and rule matching must work without cloud AI.
- **Cloud research is optional and off by default.** It runs only after the user explicitly enables it in Settings → Privacy & Security (14.2), and it must then continue to respect the user's privacy settings. With it off, classification relies on local detection, local evidence and the user's own rules. Public research is performed only when the user's settings allow web or cloud access (consistent with the offline-first principle and 18.1), and the initial version must work with one configured model (3.15) without requiring automatic multi-model routing.
- **Minimum necessary metadata** is sent to any cloud provider.
- **Never upload** keystrokes, private file contents or screenshots for application classification. Ascend AI must not inspect private application content (for example screen, document, page or typed content) without the user's explicit permission, and this specification defines no content-inspection mechanism.
- **Window titles may contain sensitive information** (for example private document names). Appropriate redaction must be applied before any cloud transmission, and transmission requires the applicable privacy setting.
- **Research results and window titles are untrusted data** (3.11). Instructions inside them never take effect, and they cannot authorize an action, grant permissions or create or modify memories (3.9).
- **No write access:** the background investigation can only produce a suggestion record containing its evidence and reasoning. It cannot create, modify or delete classification rules. Ascend AI must never create a permanent classification rule without the user's approval, and a suggestion takes effect only after the user explicitly confirms it through the application interface.
- Suggestions use the existing Tool Registry read access to activity data under the existing permission rules (3.11) and require no new autonomous capability. A suggested category must be a valid default or custom category.
- Provider use is subject to the user's provider configuration and privacy controls (3.15, 11.6). Activity exclusions (11.6) apply.
- Confirmed classifications and rules are application data, not Ascend AI long-term memory (3.9).

### Rule matching

Ascend uses a deterministic, priority-based rule-matching engine implemented in Rust. Matching must work without cloud AI.

Matching principle: **specificity first, user authority second.**

Matching priority, highest first:

| Priority | Rule |
|---|---|
| 1 | Exact website/domain or URL rule, when browser integration is authorized |
| 2 | Application-specific window-title rule |
| 3 | Application-wide user-defined rule |
| 4 | General category defaults |
| 5 | AI suggestions that have not been approved (suggestions only, not matching rules) |

Rules:
- The application is identified by exact application identity and verified executable information. This determines which application-specific rules apply.
- Website rules override the browser's general application classification for that browsing activity.
- Window-title rules apply only to their associated application.
- More specific user-defined rules override broader user-defined rules.
- User-defined rules always override category defaults and AI-generated suggestions.
- Conflicting rules must be detected rather than arbitrarily selected. If equally specific rules conflict, the conflict is flagged and the activity is classified as Unclassified until the user resolves it.
- AI cannot silently create or modify permanent rules. An unapproved suggestion is never applied by the engine; it takes effect only when the user approves it, at which point it becomes a user-defined rule.
- Users can create, edit, disable and delete rules.
- Website and URL rules, and browser-tab-level classification, require an authorized browser extension. Without one, only the available application and window metadata is used (11.2).
- Example: Chrome is generally Neutral (Browsing). A programming tutorial website can be classified as Learning, and a particular entertainment website can be classified as Entertainment.

### Retroactive reclassification

When the user changes an application's classification, Ascend must ask whether the change applies to future activity only, or to past and future activity.

For historical changes:
- Original activity observations are preserved.
- Classification changes are stored with their timestamps.
- Affected analytics are recalculated using the updated classification.
- Enough history is preserved to understand what the original classification was.
- The user can review the affected historical period before a bulk change is applied.
- Original activity records are never silently rewritten.

### Architectural responsibility

| Component | Responsibility |
|---|---|
| Rust activity engine | Detection, session tracking, rule matching and persistence |
| Classification engine | Deterministic category and focus assignment; a Rust component of the activity engine |
| Local AI | Context interpretation and classification assistance |
| Cloud AI | Optional public application research and deeper investigation |
| User | Final authority over persistent classifications |

### Calibration items

These are implementation values to be tuned and tested, not open product decisions:
- Numeric confidence thresholds for High, Medium and Low.
- The minimum usage that counts as a meaningful amount of time before investigation begins.
- What evidence counts as sufficient for a suggestion earlier than the 3-session default, and what counts as meaningful new evidence after a dismissal.
- What counts as a separate session for the 3-session count (activity sessions, 11.3).
- The exact rule by which an explicit focus context results in a Distracting classification.
- What counts as verified executable information.
- The redaction rules for window titles applied before any cloud transmission.
- Where suggestions are surfaced (in-app or notification), within the rules in 3.8 and Section 13.

# 12. Analytics Workspace

## 12.1 Product Direction — Confirmed

- Analytics uses one unified central workspace with filters and dedicated detailed sections.
- It brings together data from Tasks, Habits, Goals, Milestones, Projects, Timers and Activity Tracker.
- Users can inspect overall patterns and then explore individual areas in more detail.

## 12.2 Metrics and Coverage — Confirmed

- Productivity overview: activity, completed work and consistency.
- Time analytics: active time, focus time, idle time and time distribution.
- Activity analytics: coding, learning, research, productivity, browsing, entertainment, communication, distraction, other and application usage, with focus classification (11.8) available as a separate dimension.
- Individual application usage and time spent in each app.
- Category-level time distribution, including pie charts.
- Task analytics: completed, pending, overdue tasks and completion trends.
- Habit analytics: completion rates, streaks and consistency.
- Goal and milestone analytics: progress and progress history.
- Project analytics: time invested, current project activity, progress and neglected projects.

## 12.3 Visualizations — Confirmed

- Activity timelines.
- Charts and graphs.
- Heatmaps for consistency and activity history.
- Pie charts for category distribution.
- Individual application usage visualizations.
- Current project activity and progress visualizations.
- Goal, milestone, task and habit progress visualizations.

## 12.4 Reporting and Exploration — Planned

- Daily, weekly, monthly and yearly reports.
- Custom date ranges.
- Compare different time periods.
- Filter by project, category or activity.
- Explore overall analytics and individual module details.

## 12.5 Assistant Insights — Confirmed

- Ascend AI can provide insights and recommendations based on available analytics.
- Insights should help identify patterns, productivity changes and neglected goals or projects.
- Assistant-generated interpretations should be distinguishable from directly recorded metrics.

## 12.6 Data Integrity Principle — Confirmed

- Analytics must distinguish recorded facts from estimates and inferred information.
- Timer duration and observed active time must not automatically be treated as equivalent.
- Avoid double-counting time or progress when the same activity is represented across connected modules.
- Idle duration may be shown separately but is never counted as distraction time (11.8).
- Time classified as Uncertain (11.7) is shown separately and is excluded from verified active-time and productivity calculations unless the user corrects its classification.

# 13. Notifications

## 13.1 Product Direction — Confirmed

- Ascend uses a smart notification approach: scheduled reminders plus contextual notifications based on activity, deadlines and progress.
- Notifications should keep users informed about relevant events without creating unnecessary interruptions.

## 13.2 Notification Categories — Confirmed

- **Tasks:** Upcoming deadlines, overdue tasks and scheduled tasks.
- **Habits:** Scheduled habits, incomplete habits and routine reminders.
- **Goals and milestones:** Upcoming target dates, milestone achievements and progress updates.
- **Projects:** Deadlines, inactivity and important project events.
- **Timers:** Pomodoro breaks, completed countdowns and finished sessions.
- **Streaks:** Habit streak milestones and consistency achievements.
- **Ascend AI:** Useful insights, recommendations and action approvals.

## 13.3 Notification Centre — Confirmed

- Central notification centre accessible from the application shell.
- Read and unread states.
- Grouping by category.
- Mark as read or dismiss notifications.
- Open the relevant task, project, habit or goal from a notification.
- Notification history.
- Clear all or clear selected notifications.

## 13.4 Contextual and AI Notifications — Confirmed Direction

- Provide contextual notifications based on relevant activity, deadlines and progress.
- Examples include upcoming task deadlines, stalled project progress, habit consistency changes and approaching milestone deadlines.
- AI-generated notifications must be useful, grounded in available information and non-repetitive.
- AI may notify users about action approvals that require their attention.

## 13.5 Notification Customization — Confirmed

- Enable or disable individual notification categories.
- Choose desktop notifications and/or in-app notifications.
- Configure reminder timing.
- Configure quiet hours.
- Configure notification frequency.
- Configure priority levels.

## 13.6 Future Extensibility

- Keep the notification system extensible for more adaptive, preference-aware behavior in the future.
- Adaptive AI learning of notification preferences is not required for the initial confirmed direction.

# 14. Settings

## 14.1 Product Direction — Confirmed

Settings is the central control area for configuring Ascend OS. It should feel like a native desktop application settings experience rather than a collection of dashboard cards.

The preferred interaction model is inspired by applications such as VS Code:
- A persistent settings navigation area.
- Dedicated pages for each settings category.
- Search across setting names and descriptions.
- Clear explanations of what each setting changes.
- Consistent controls and layout across categories.

## 14.2 Settings Categories — Confirmed Structure

The Settings workspace will contain the following categories:

### General
- Application startup behavior.
- Minimize-to-tray behavior.
- Language and regional preferences.
- Date and time preferences.
- Default workspace and general application behavior.

### Appearance
- Light, dark and system theme options.
- Accent color.
- Font size and interface density.
- Sidebar and panel preferences.
- Other appearance and layout preferences as the design system is finalized.

### Activity Tracking
- Enable, pause and resume tracking.
- Idle detection configuration, including separate configurable inactivity thresholds for interactive activity and passive focused activity (see 11.7).
- Application classification: review and edit application classifications, custom categories and remembered classification rules (11.8).
- Application and browser tracking preferences.
- Excluded applications and sources.
- Activity classification preferences and related controls.

### Notifications
- Notification categories and channel preferences.
- Desktop and in-app notification controls.
- Reminder timing and frequency.
- Quiet hours and interruption preferences.
- Detailed notification behavior remains governed by the Notifications requirements.

### Connections
- Discover supported integrations.
- Manage connected services and their permissions.
- Manage explicitly authorized local resources.
- Configure global Read and Write access.
- This reuses the Connections system already defined in the Projects and Global Connections requirements; it must not create a second integration-management system.

### Ascend AI
- Configure available AI providers, including local and cloud providers.
- Select models.
- Configure AI tool access and action approvals.
- Manage AI context and memory-related controls.
- Detailed provider, agent and tool behavior remains subject to the dedicated AI planning work.

### Data & Backup
- View and manage local data storage.
- Configure backup behavior and locations.
- Restore from a backup.
- Export and import supported user data.
- View storage usage where practical.
- Backup, restore and destructive data operations must include appropriate confirmation and clear outcome reporting.

### Privacy & Security
- Review data and activity collection controls.
- Configure sensitive activity exclusions.
- Enable or disable cloud research for application classification (off by default; see 11.8).
- Manage credential and access protection preferences.
- Provide clear controls for deleting user data.
- Privacy controls must be understandable and must not imply that data is collected or transmitted when it is not.

### Keyboard Shortcuts
- Browse and search available application shortcuts.
- Customize shortcut assignments.
- Provide sensible default shortcuts.
- Detect conflicting assignments and explain the conflict before saving.
- Provide a way to restore default shortcuts.
- Shortcuts should cover common global and workspace actions, including opening Ascend AI, creating tasks, starting timers, opening search and switching workspaces.
- Exact shortcut combinations and platform-specific behavior are deferred to interaction design.

### About & Updates
- Display application version and relevant build information.
- Check for application updates.
- Show release notes or changelog information.
- Provide access to diagnostics and application information where appropriate.

## 14.3 Settings Interaction Principles — Confirmed Direction

- Settings must be searchable so users can find an option without manually browsing every category.
- Most ordinary preferences should save automatically when changed and apply immediately where practical.
- Settings that require restart or another action must clearly communicate that requirement.
- Users should be able to reset an individual setting or restore a category's defaults where appropriate.
- Controls must include concise descriptions and explain dependencies.
- Settings that depend on another feature being enabled should be disabled or clearly explained rather than behaving unpredictably.
- Consequential operations, especially restore, deletion and other destructive data actions, require appropriate confirmation.
- Settings should remain consistent with the application's overall desktop workspace and visual design.

## 14.4 Still to Be Designed

- Exact page layouts and settings navigation interactions.
- Final setting inventory and default values.
- Shortcut key combinations, scopes and platform-specific rules.
- Backup format, schedule, retention and restore behavior.
- Data export/import formats and migration behavior.
- Detailed AI provider and model configuration.
- Update delivery and rollback behavior.
- Which settings apply globally versus to a particular workspace or record.

---

# 15. Original Brainstorm — Preserved Ideas

As of v5.2 this section retains only ideas from the original brainstorm that have not been confirmed elsewhere. Confirmed requirements that were previously recorded here have been moved to dedicated sections: Tasks (Section 19), Habits (Section 20) and Routines (Section 21). Brainstorm items that are already covered by a confirmed section are marked with a pointer rather than duplicated.

## Core Features
- Habits.
- Routines.
- Calendar.
- Goals.
- Tasks.
- Notes.
- Timers.
- Tracking system.
- Programmer-focused capabilities.
- Advanced analytics.

## Page Philosophy

- Track = What you're trying to accomplish.
- Plan = How you're organizing it.
- Tracker = What you're actually doing.
- Analytics = What Ascend learns from it.
- Home = What matters right now.

## Additional Brainstorm Details

### Habits

Confirmed; moved to Section 20.

### Routines

Confirmed; moved to Section 21 (including the habit and routine interaction rules in 21.2).

### Calendar

Covered by the confirmed requirements in Section 07 (day, week and month views; visualizing consistency and progress over time). No unconfirmed ideas remain here.

### Tasks

Confirmed; moved to Section 19.

### Notes

Covered by the confirmed requirements in Section 08 (attachments, checklists, links and connections to projects and other records). No unconfirmed ideas remain here.

### Timers

Covered by the confirmed requirements in Section 09 (Pomodoro, stopwatch, countdown, deep work, and connections to habits and tasks). No unconfirmed ideas remain here.

### Notifications

Covered by the confirmed requirements in Section 13 (task and habit reminders, deadline warnings, streak notifications and smart notifications). No unconfirmed ideas remain here.

### Analytics

Most original items are covered by the confirmed requirements in Section 12. The following was not explicitly confirmed there and remains a brainstorm idea:

- Goal progress, velocity and time invested.

(Goal and milestone progress and progress history are confirmed in 12.2; velocity and goal time invested are not.)

### Activity Tracker

Most original items are covered by the confirmed requirements in Section 11. The following were not explicitly confirmed there and remain brainstorm ideas:

- Live activity.
- Current project, if connected.

(Distraction, previously listed here, is now confirmed in 11.8 as both a default activity category and the focus classification Distracting.)

### Settings

The original items are covered by the confirmed categories in Section 14, except the following, whose scope was never defined and which remains a brainstorm idea:

- Interactions.

---

# 16. Open Decisions and Future Planning

As of v5.2 this list contains only genuinely unresolved decisions. Resolving these items completes design details within the existing scope and does not expand the scope frozen in v5.7 (Section 17). Items finalized elsewhere in this document have been removed, including: Goals and Milestones behavior (06, 18.3), the Calendar experience (07, 18.3), the Timer system (09), the backend and native Windows architecture (18.6), persistence and security baselines (18.6–18.7), and Tasks, Habits and Routines product behavior (Sections 19–21, 18.3). Implementation sequencing is governed by the separate Codex Implementation Roadmap and is not tracked here.

1. Final grouping and ordering of navigation destinations within the structure in 2.2.
2. Detailed workspace layouts and interaction design for each workspace, to be produced with Google Stitch (18.5, 18.6.1), including the Notes editor and graph controls (Section 08) and the final Home widget catalogue and default layout (4.3).
3. Tasks deferred details: natural-language date parsing, quick-capture behavior, saved-view customization and detailed recurring-task rules beyond the defaults in 18.3 (19.5).
4. Activity Tracker: project attribution rules, user-confirmation and automatic-detection design (10.13, 11.4); the browser-extension mechanism and supported browser coverage, which is deferred (11.2, 18.6.15); the remaining design items in 11.7; and the calibration items in 11.8.
5. Analytics: exact metric definitions documenting numerator, denominator, time zone, excluded states and data source (18.3).
6. Notifications: default values for reminder timing, frequency limits and priority rules (13.5, 18.3).
7. Ascend AI: per-tool risk classifications, approval experience and autonomy controls (3.7, 3.11); proactivity triggers, frequency limits and delivery surfaces (3.8); mode-selection UI and persistence, custom modes, and response-style presets and adaptation controls (3.9); and the remaining items in the "Still to be designed" lists in 3.10 (including embedding model and semantic retrieval) and 3.13.
8. Integrations: the exact integration catalogue and integration-specific capability definitions, the OAuth and authorization architecture, and per-project permission granularity (10.14, 18.2).
9. Projects: the custom project type and field builder and its field types (10.7), and the degree of automatic configuration versus confirmation in AI-assisted project creation (10.8).
10. Design system: exact component library and design tokens (18.5, 18.6.15).
11. Exact database tables, columns and indexes (18.6.6, 18.6.15).
12. Backup schedule and retention defaults, export/import formats, update distribution and code signing, the telemetry policy and the sensitive-data policy (14.4, 18.6.13–18.6.15, 3.9).

---

# 17. Decision Management Rules

This document is the working source of truth for Ascend OS.

For every future planning topic:

1. Discuss the topic.
2. Explore alternatives and trade-offs.
3. Agree on product behavior.
4. Record finalized decisions in this document.
5. Clearly label unresolved questions and recommendations.
6. Increment the document version when meaningful changes are made.
7. Only after product planning and architecture approval, prepare implementation documentation for Codex.

No implementation work should begin merely because a feature has been discussed.

**Specification freeze (v5.7):** the scope of this specification is frozen. No further expansion of scope is planned. Any change after this point requires an explicit decision recorded under this section and a version increment.

## Change Log

| Version | Change |
|---|---|
| 0.1 | Initial product vision, application shell, navigation, AI, Home, relationships, Goals and Projects direction |
| 0.2 | Expanded Projects into a universal project-context and integration system |
| 0.3 | Added customizable project types, global connection management, per-project connection selection and AI-assisted project creation |
| 0.4 | Confirmed the three-view Connections settings structure: Discover, Connected and Local Resources; documented global authorization, project assignment, AI behavior and privacy requirements |
| 0.5 | Confirmed separate, user-controlled read and write permissions for integrations |
| 0.6 | Confirmed flexible permission configuration: integration-level defaults plus selective per-capability Read and Write controls, including broad, read-only and mixed access |
| 0.7 | Simplified integration permissions to one global configuration; confirmed hybrid Tasks views and a combined lists + connected-records organization model |
| 0.8 | Confirmed a deliberately lightweight initial task field structure; defer additional fields and customization until development demonstrates a need |
| 0.9 | Confirmed the planned Habits and Routines functionality and how routine activities can record existing habit completions without duplication |
| 1.0 | Confirmed Goals and Milestones functionality, progress principles, relationships and the optional Windows progress widget |
| 1.1 | Confirmed Calendar views, selected-day task/habit/project activity overview and individual consistency history |
| 1.2 | Confirmed Obsidian-inspired Notes & Knowledge workspace with local Markdown files, folders, visual formatting, images and visual note/project connections |
| 1.3 | Confirmed unified Timers workspace with Pomodoro, stopwatch, countdown and deep-work modes, global access, record connections and separation from observed activity |
| 1.4 | Confirmed Activity Tracker coverage of foreground apps and browser websites/tabs; documented detection signals, project attribution, idle handling and privacy direction |
| 1.5 | Confirmed unified Analytics workspace with timelines, assistant insights, heatmaps, charts, category pie charts, individual app usage and project/progress visualizations |
| 1.6 | Confirmed smart Notifications with contextual reminders, central notification centre, category controls, quiet hours and customization |
| 1.7 | Confirmed Settings categories, searchable native desktop settings experience, immediate-save direction and customizable keyboard shortcuts with defaults and conflict detection |
| 1.8 | Confirmed Ascend AI's agent-with-approval autonomy model and its collaborative behavior: respond, explain, suggest, ask, act and report |
| 1.9 | Confirmed contextually proactive Ascend AI behavior with user-configurable proactivity, relevance, low-interruption defaults and clear separation between suggestions and authorized actions |
| 2.0 | Confirmed workspace-wide context across authorized Ascend records and long-term AI memory, with user inspection, correction, privacy boundaries and source transparency |
| 2.1 | Confirmed hybrid retrieval architecture: Rust backend owns trusted data access and retrieval services; AI orchestration chooses and coordinates retrieval tools, with structured queries, FTS, relationship-aware retrieval and semantic search planned in stages |
| 2.2 | Confirmed centralized AI Tool Registry with typed tools, shared application services, execution-time permission checks, approval requirements, structured results and action history |
| 2.3 | Confirmed Plan → Approve → Execute for consequential multi-step AI workflows, while allowing explicitly requested simple reversible actions to execute directly; documented result reporting and recovery questions for later design |
| 2.4 | Confirmed hybrid AI conversation organization: general, project-linked, goal-linked and custom conversations, with separate conversation histories and shared user-controlled long-term memory |
| 2.5 | Confirmed AI conversation support for file uploads, image understanding, voice input/output, rich code and Markdown presentation, interactive elements and generated artifacts |
| 2.6 | Confirmed single-model initial setup using Ollama `llama3.1:8b`, future multi-provider flexibility and benchmark-based evaluation of alternatives before switching |
| 2.7 | Confirmed intelligent AI workflow recovery: bounded retries for transient failures, preserve completed steps, safely resume unfinished work, prevent unsafe duplicate execution and request user input when needed |
| 2.8 | Confirmed selective automatic saving of useful non-sensitive long-term memories, with explicit consent required for sensitive information and user inspection, correction and deletion controls |
| 2.9 | Confirmed categorized long-term memory organization: preferences, goals, projects, learning, decisions and routines, with automatic categorization and user-editable categories |
| 3.0 | Confirmed memory conflict handling: prefer the latest explicit user statement, update current memory, preserve useful history and clarify ambiguous or temporary changes |
| 3.1 | Confirmed separation of long-term AI memory from authoritative workspace records; workspace data governs current application state while memory provides conversational context |
| 3.2 | Confirmed customizable AI response styles (conciseness, tone and approach), without changing accuracy, transparency, permissions or safety requirements |
| 3.3 | Confirmed subtle context-aware response-style adaptation, with the user's configured preference remaining the default and explicit override taking priority |
| 3.4 | Confirmed four selectable operating modes—Assistant, Agent, Researcher and Coach—within one unified AI system and shared permission model |
| 3.5 | Confirmed AI mode selection in the conversation interface, Assistant as default, user-controlled in-conversation switching and recommendations without automatic switching |
| 4.0 | Added a working baseline for remaining AI, integrations, module rules, gamification, UI/UX, architecture, security and planning exit criteria; routine defaults are decided to avoid unnecessary planning loops |
| 5.0 | Finalized the technical architecture baseline: Tauri 2, React + TypeScript + Vite, Rust modular monolith, SQLite/SQLx/FTS5, typed bridge, AI orchestration, storage, integrations, background services, backup, build and testing boundaries |
| 5.2 | Specification corrections and clarifications; no product redesign. Consistent v5.2 versioning and repaired changelog; Projects sections renumbered 10.x to remove the Calendar numbering conflict; Calendar, Routines and Gamification added to navigation; confirmed Tasks, Habits and Routines requirements promoted from the brainstorm into Sections 19–21; AI capability dependencies (image understanding, voice, PDF/DOCX parsing) clarified against the text-only `llama3.1:8b` baseline; separate interactive and passive-focus idle rules with Windows `GetLastInputInfo` system idle detection (11.7); browser-tab tracking dependency clarified and deferred (11.2); AI prompt-injection and untrusted-content protections (3.11, 18.6.8, 18.7); Markdown/SQLite note synchronization (18.6.10); Tauri least-privilege, scope and CSP requirements (18.6.5); relationship deletion cleanup and integrity (18.6.6); Home and Gamification added to frontend structure (18.6.4); open decisions reconciled (Section 16) |
| 5.3 | Activity tracking: introduced Uncertain as a third engagement mode alongside Interactive (60 seconds) and Passive Focused (5 minutes); Uncertain receives no productivity credit, is recorded separately, is excluded from verified active time and productivity calculations, is reviewable and correctable, and is always used instead of guessing when the engagement mode cannot be confidently identified (11.7, 12.6, 18.3). AI memory: added memory source rules so that only information explicitly communicated by the user can be automatically considered for personal long-term memory, retrieved content can never create or modify personal memories, explicit user requests can authorize saving sourced information, and workspace records remain authoritative (3.9, 3.11, 18.6.8, 18.7) |
| 5.4 | Activity classification architecture (new 11.8): application classification separated into engagement state, activity category and focus classification; Unclassified as the default for unrecognized applications (never automatically a Distraction); Entertainment distinct from Distraction; Idle defined as an engagement state and not a category or distraction; user-assigned and custom categories; Ascend AI classification suggestions that require user confirmation and use permitted metadata only; browsers not classified by application name alone, with classification confidence marked. Related updates to 11.3, 11.7, 12.2, 12.6, 14.2, 15, 16, 18.3 and 18.4 |
| 5.5 | Section 11.8 finalized and frozen: confidence thresholds (High, Medium, Low) with evidence and reasoning recorded; default of 3 sessions before a suggestion, with earlier suggestions when evidence is sufficient and no repeated prompts for dismissed suggestions; active focus context established only by explicit user-designated context; default focus classification per category (Productivity and Distraction added to the default categories); retroactive reclassification that preserves original observations; deterministic priority-based rule matching in Rust; new Unclassified App Intelligence background research workflow with privacy and execution requirements and an architectural responsibility split. Related updates to 11.3, 12.2, 15 and 18.3 |
| 5.6 | Section 11.8: cloud research for application classification is **off by default** and requires an explicit user opt-in; control added to Settings → Privacy & Security (14.2). Redaction rules for window titles remain a calibration item |
| 5.7 | Section 11.8 rule matching finalized as specificity first, user authority second, with a five-level priority (website or URL rule, application-specific window-title rule, application-wide user rule, category defaults, unapproved AI suggestions that are never matching rules); Browsing default focus classification set to Neutral. Specification scope frozen |

*Note: the previous revision (document version 5.0) was distributed under the file name "v5_1"; no separate 5.1 changes were recorded. v5.2 supersedes it.*

**Current phase:** Product planning is complete. Implementation preparation is underway; implementation proceeds through the separate Codex Implementation Roadmap as incremental vertical slices (backend and frontend developed together, module by module).

**Current version:** 5.7

# 18. Remaining Product and Engineering Decisions — Working Baseline

This section closes routine open questions with practical defaults so planning can progress without repeatedly asking the user to approve obvious supporting behavior. These decisions are the working baseline for the next design and engineering stages. They remain revisable if implementation evidence or a meaningful product trade-off justifies a change.

## 18.1 Ascend AI — Mode Capabilities and Defaults

### Mode capability profiles

All modes use one AI runtime, shared conversation system, workspace context, memory, tool registry and permission enforcement. The active mode supplies task-specific instructions, preferred workflows and relevant tool subsets.

- **Assistant:** General questions, explanations, everyday workspace help, quick capture and contextual assistance. It may use read tools and perform explicitly requested low-risk actions within existing permissions.
- **Agent:** Multi-step workspace operations, planning, record creation and updates, relationship management and workflow execution. It uses the established plan-review-execute process for consequential operations.
- **Researcher:** Web research when explicitly enabled and available, source discovery, comparison, synthesis, citation and research-note creation. External information must be distinguished from local workspace facts. It must not claim to have checked a source it did not access.
- **Coach:** Goal reviews, habit and routine reflection, planning, progress interpretation and practical next-step suggestions. It should support user agency, avoid shame-based language and ground progress claims in recorded data.

Modes are behavior profiles, not separate agents or permission domains. Mode switching never grants additional access.

### Tool catalogue baseline

The initial tool catalogue should cover:
- Read/search workspace records.
- Create, update, complete, archive and delete supported records, with risk-based confirmation.
- Create and manage relationships between records.
- Retrieve project, goal and task context.
- Read analytics and activity summaries.
- Start, pause and stop supported timers when explicitly requested.
- Search authorized local resources and connected services.
- Create research summaries and notes.
- Prepare multi-step plans and report execution results.

Tools must use typed inputs, validated outputs, stable record identifiers, execution-time authorization and audit events. Destructive operations should prefer archive or reversible soft-delete where practical. Permanent deletion requires explicit confirmation.

### Conversation and memory defaults

- Conversations support rename, search, archive and delete.
- Project- and goal-linked conversations retain links to their related records.
- Attachments belong to the conversation or artifact that uses them and are stored locally unless an explicitly authorized external integration is involved.
- Users can export a conversation and delete it.
- Long-term memory remains separate from workspace records. Workspace records are authoritative for current state.
- Memory management should provide inspect, edit, disable automatic saving, and delete controls.
- Deleting a conversation does not automatically delete linked workspace records or independently stored memories. The UI must explain these boundaries.
- Memory retrieval should be selective and source-aware; do not inject every memory into every prompt.

### Agent execution defaults

- Persist execution state for multi-step workflows so interrupted work can be inspected and safely resumed.
- Show plan, current step, completed steps, failures and pending approvals.
- Retry transient failures a small bounded number of times; never blindly retry uncertain external or destructive effects.
- Use idempotency keys or state verification for operations that could create duplicates.
- Provide undo for reversible internal changes where reliable. Where undo is not technically safe, explain that limitation before approval.
- Never claim an action succeeded until the relevant application service confirms it.

## 18.2 Integrations and Connections — Initial Scope

### Integration strategy

Ship the Connections framework before attempting a large integration catalogue. Integrations should be added incrementally based on real workflows and maintainability.

Initial priorities:
1. Local file and folder resources, with explicit folder authorization.
2. GitHub for development projects: repositories, issues, pull requests and relevant activity, subject to supported API capabilities.
3. Web research capability for Researcher mode, with clear source attribution.
4. Calendar integration only after the internal Ascend Calendar behavior and synchronization rules are stable.
5. Additional productivity and cloud-storage services after the first integrations prove the connector framework.

Do not promise integrations merely because a category appears in Discover. Show only integrations that are implemented or clearly label planned availability.

### Connection behavior

- Global authorization is separate from project assignment.
- Read and Write permissions are separate and capability-specific where the provider supports it.
- Request the least access necessary.
- Local folders are explicitly selected; do not scan the entire computer by default.
- Show connection status, last successful sync, available capabilities and permission scope.
- Provide disconnect and revoke controls.
- Store credentials in the operating system's secure credential store, not in ordinary application tables, logs, notes or AI conversation text.
- External sync must preserve source identifiers and timestamps where available.
- On conflict, do not silently overwrite. Prefer an explicit conflict state and user choice; use source-specific safe defaults only when the direction of authority is unambiguous.
- Offline mode continues to work with local data. External operations are queued only when they are safe to defer and are visibly marked pending.

## 18.3 Module Rules — Default Behavior

These defaults resolve common edge cases without requiring a separate design decision for every field.

### Goals and milestones

- Goal progress uses the goal's selected progress method: manual, measurable target or configured linked-record completion.
- Linked records contribute only when explicitly included in that goal's progress rule.
- A task completion may update a configured goal calculation, but never automatically completes the goal itself.
- Milestones are independently tracked; completing one does not automatically complete its parent goal.
- Parent/subgoal progress is opt-in and must not double-count the same contribution.

### Projects

- Project progress is configurable and based on explicit project milestones, measurable targets or selected linked work—not a universal completed-task percentage.
- Default project lifecycle: Planning, Active, On Hold, Completed, Archived.
- Custom project types use the same core project model with optional templates and fields.
- A project may have multiple local directories and external connections.
- Activity attribution requires reliable evidence; ambiguous activity remains unassigned until corrected.
- Completing a project does not delete its tasks, notes or history.

### Tasks

Detailed requirements: Section 19.

- Default statuses: To Do, In Progress, Blocked, Done.
- Default priorities: Low, Medium, High, Urgent.
- Subtasks are child tasks, but remain independently addressable records.
- Completing a parent task does not silently complete unfinished subtasks; offer an explicit completion choice.
- Recurring tasks create a new occurrence after completion or at the next scheduled interval; preserve prior occurrences for history.
- Due dates and start dates are distinct. Overdue means an unfinished task whose due date has passed.
- Archive is preferred to permanent deletion for ordinary cleanup.

### Habits and routines

Detailed requirements: Sections 20 and 21.

- Habit schedules define expected occurrences; a missed occurrence is not inferred until its tracking window closes.
- Skipped is distinct from missed and completed.
- Streak rules must be visible and consistent with the chosen schedule; flexible weekly targets should not be judged by daily streak rules.
- A routine can record completion of a linked habit once, avoiding duplicate habit entries.
- Editing a schedule affects future expectations and should preserve historical records.

### Calendar

- Internal Ascend records are the initial source of truth for Ascend-created schedules.
- External calendar events remain attributed to their source.
- Distinguish all-day events, timed events, deadlines and scheduled work.
- Dragging an item to a new time updates the underlying record only after a successful save.
- Calendar views must not imply that planned work was completed.

### Notes and knowledge

- Markdown remains the portable underlying format.
- Notes are stored locally and remain usable offline.
- Autosave with visible save/error state; avoid silent loss on app close.
- Internal links and backlinks use stable note identifiers rather than relying only on filenames.
- Renaming or moving a note must preserve links.
- Local Markdown files and SQLite metadata are kept synchronized as specified in 18.6.10 (stable IDs in frontmatter, file watching, startup reconciliation and conflict detection).
- Attachments are stored in a managed local assets area and referenced by the note.
- Graph view is a navigation and discovery aid, not a replacement for folders, search or backlinks.

### Timers and activity tracking

- A timer session is explicit user intent; observed computer activity is a separate evidence stream.
- Timer sessions survive workspace switching and app minimization; app restart should restore a clear recoverable state.
- Prevent overlapping sessions of the same timer type unless the user explicitly chooses a supported concurrent workflow.
- Idle time is excluded from active-work totals by default. Prolonged inactivity pauses active tracking sessions. Inactivity handling depends on the engagement mode (11.7): Interactive activity uses a 60-second initial default threshold, and Passive Focused activity such as reading documentation uses a separate, longer grace period (initial proposed default 5 minutes). Both are configurable. Activity whose engagement mode cannot be confidently identified is classified as Uncertain, receives no productivity credit, is recorded separately, and is excluded from verified active-time and productivity calculations.
- Application classification separates engagement state, activity category and focus classification (11.8). Unrecognized applications are Unclassified, never automatically Distraction. Classification rules are matched by a deterministic Rust engine. Ascend AI may suggest a classification from permitted metadata, but a suggestion takes effect only after the user confirms it.
- Keyboard content and typed text are never captured.
- Project attribution requires reliable signals; uncertain activity remains unassigned.
- Corrections to activity classification must be reflected in downstream analytics.

### Analytics and notifications

- Analytics definitions must document numerator, denominator, time zone, excluded states and data source.
- Never combine estimated active time and manually recorded timer duration as though they were the same measurement.
- Avoid double-counting a single completed record across linked goals and projects.
- Reports use local time zone and clearly display the selected period.
- Notification defaults should be conservative: deadlines and explicitly scheduled reminders first; AI insights and suggestions lower priority.
- Provide quiet hours, category controls, frequency controls and a notification history.
- Repeated notifications for the same unresolved event should be deduplicated or consolidated.

## 18.4 Gamification and Progression — Initial Product Direction

Gamification should reinforce meaningful progress, not encourage meaningless activity or inflate productivity metrics.

### Core system

- XP is awarded for verified completion events and explicitly configured milestones.
- A single underlying completion event must not generate duplicate XP because the record is linked to multiple goals or projects.
- Reopening a completed record should not silently award XP again.
- Levels represent cumulative XP and are primarily motivational; they do not unlock essential functionality.
- Achievements recognize meaningful milestones, consistency and completed work.
- Quests are optional suggested challenges built from existing goals, tasks and habits.
- Streaks belong to the relevant habit or schedule and must respect skipped days and flexible schedules.
- Users can inspect why XP or an achievement was awarded.
- Provide controls to disable gamification elements without disabling core tracking.
- No penalties, lost levels or shame-based messaging for missed work.

### Integrity rules

- Do not award XP for merely opening an application, leaving a timer running or generating large numbers of trivial tasks.
- Idle time earns no XP or progression credit. Uncertain time receives no productivity credit (11.7, 11.8).
- AI-created records do not earn completion rewards until the user actually completes the relevant work.
- Analytics remain grounded in underlying records; XP is not a substitute for productivity measurement.
- Exact XP values, level thresholds and achievement catalogue should be tuned after usage testing rather than guessed during product planning.

## 18.5 UI/UX — Design System Baseline

### Visual direction

- Desktop-first, dark-first interface with a restrained grayscale foundation and one configurable accent color.
- Clear hierarchy, strong typography and deliberate spacing.
- Avoid generic SaaS dashboards, excessive cards, decorative gradients and unnecessary borders.
- Prefer persistent workspaces, panes, lists, tables, timelines and focused detail views.
- Use cards only when they communicate a genuinely self-contained object or summary.

### Interaction principles

- Keyboard-first operation with discoverable shortcuts.
- Consistent command palette and global search.
- Persistent sidebar with compact and expanded states.
- Main workspace tabs and resizable panes.
- Contextual right panel for related information and quick inspection.
- Fast quick-create flows and inline editing where safe.
- Every asynchronous operation needs a visible loading, success, empty or error state.
- Destructive actions require confirmation; ordinary reversible edits should not be burdened with repeated dialogs.
- Support sensible scaling, keyboard navigation, readable contrast and reduced-motion preferences.

### Responsive and layout behavior

- Optimize the primary experience for desktop and laptop screens.
- At narrower widths, collapse secondary panels and use navigation overlays rather than compressing every pane.
- Persist user layout choices.
- Do not allow a user-customized layout to make core actions inaccessible; provide reset-to-default.

## 18.6 Technical Architecture — Finalized Baseline

This section records the selected architecture for the first major Ascend OS rebuild. It is a modular monolith: one desktop application, one local database and a set of well-separated internal services. The architecture avoids premature distributed infrastructure while keeping boundaries clear enough to evolve.

### 18.6.1 Selected technology stack

| Layer | Selected technology | Responsibility |
|---|---|---|
| Desktop runtime | Tauri 2 | Windows desktop shell, lifecycle, native capabilities and packaging |
| Frontend | React + TypeScript + Vite | Desktop UI and client-side application state |
| Routing | React Router | Workspace and settings navigation |
| Server-state synchronization | TanStack Query | Loading, caching, invalidation and mutation state for backend commands |
| UI-only state | Zustand | Temporary interface state such as open panels, selected tabs and layout state |
| Backend | Rust | Domain logic, persistence, local services, security and background work |
| Desktop bridge | Tauri commands and typed events | Narrow, validated frontend/backend boundary |
| Database | SQLite | Authoritative local application data |
| Database access | SQLx with SQLite and versioned migrations | Typed queries, transactions and schema migration |
| Full-text search | SQLite FTS5 | Initial local search across supported records and note content |
| AI providers | Ollama initially; provider adapter for future OpenAI-compatible APIs | Model inference |
| AI orchestration | Rust application service | Context assembly, tool selection/execution loop, approvals and recovery |
| Secret storage | OS credential store through a Rust keyring abstraction | API keys, OAuth tokens and other secrets |
| Logging | Rust `tracing` with structured, redacted file output | Diagnostics and operational events |
| Testing | Rust unit/integration tests, frontend tests and end-to-end smoke tests | Correctness and release confidence |

**Frontend framework decision:** use React + Vite rather than Next.js for the rebuilt desktop application. Ascend is a local desktop application and does not need server-side rendering, public web routing or a Next.js server runtime. A Vite-based frontend produces a straightforward static bundle for Tauri and reduces runtime complexity. Existing React components and business logic may be migrated where useful, but Next.js-specific server assumptions must not become architectural dependencies.

**UI design workflow:** Google Stitch will be used later to explore and specify UI/UX. Stitch output is a design input, not a runtime dependency. Generated layouts should be translated into maintainable React components and shared design-system primitives; do not embed generated prototypes as the production application architecture.

### 18.6.2 Architecture shape and dependency direction

The application is divided into these logical layers:

1. **Presentation layer:** React pages, workspace components, forms, navigation, panels and visual state.
2. **Desktop bridge:** typed Tauri commands and typed events. It performs input-shape validation at the boundary and exposes only supported application capabilities.
3. **Application layer:** use cases and orchestration such as create task, link project, complete habit, build analytics report, execute AI tool and restore backup.
4. **Domain layer:** entities, value types, invariants, progress calculations, relationship rules and domain events. This layer should not depend on React or Tauri UI details.
5. **Infrastructure layer:** SQLite repositories, file system access, provider clients, operating-system integration, logging and external connectors.

Dependency direction is inward: presentation calls application services through the desktop bridge; application services use domain rules and infrastructure interfaces. Repositories and external providers must not contain UI policy. The frontend must not import database drivers or access SQLite directly.

### 18.6.3 Backend module boundaries

Use one Rust application with clear internal modules/crates where complexity justifies them. Start with a single Rust crate and enforce module boundaries; split crates only when there is a concrete build, ownership or testing benefit.

Initial backend domains:

- `goals`: goals, milestones and progress rules.
- `projects`: project records, project types, fields and project progress.
- `tasks`: tasks, subtasks, lists, recurrence and completion.
- `habits`: habits, schedules, occurrences, routines and streak calculations.
- `calendar`: internal calendar entries and scheduled record projections.
- `notes`: Markdown note metadata, links, backlinks, attachments and search indexing.
- `timers`: timer configuration, active sessions and history.
- `activity`: foreground application/browser observations, classification, idle state and attribution.
- `analytics`: metric definitions, aggregation and reports.
- `notifications`: scheduling, deduplication, delivery and history.
- `connections`: integration registry, authorization metadata, sync state and local resource grants.
- `ai`: provider adapters, conversation services, context retrieval, memory, mode profiles and orchestration.
- `tool_registry`: tool definitions, schemas, risk classification, authorization and execution.
- `files`: managed storage, authorized local resource access, import/export and backup.
- `settings`: application preferences and feature configuration.
- `gamification`: XP ledger, levels, quests and achievements.

Cross-domain operations must use application services and explicit transactions rather than reaching into another module's tables from arbitrary code. A module may query another domain through a documented repository/service interface.

### 18.6.4 Frontend organization and state ownership

Organize the frontend by feature/workspace, with shared application shell and shared UI primitives. Avoid one giant global component tree or a generic components folder containing domain logic.

Recommended structure:

- `app/`: startup, routing, providers and application-level composition.
- `shell/`: sidebar, tabs, command palette, global search, contextual panel and notifications.
- `features/`: home, goals, projects, tasks, habits (including routines), calendar, notes, timers, activity, analytics, gamification, connections, AI and settings.
- `components/`: reusable visual primitives and shared interaction patterns.
- `lib/`: typed desktop bridge client, formatting and frontend utilities.
- `state/`: UI-only global state and persisted layout preferences.
- `types/`: shared frontend-facing DTOs generated or maintained against backend contracts.

Notes on specific features:
- `home` composes the customizable Home widgets (Section 04). It has no domain of its own: it reads data from the other features' application services through the same typed desktop bridge, and its layout preferences are persisted through backend settings services (see state ownership below).
- `gamification` mirrors the backend `gamification` domain (18.4): XP, levels, achievements, quests and the explanation of why XP or an achievement was awarded. It respects the user's ability to disable gamification elements without disabling core tracking.
- `habits` covers both habits and routines, mirroring the backend `habits` domain.

State ownership:
- SQLite is the durable source of truth.
- Rust application services own domain mutations and validation.
- TanStack Query owns frontend cache and synchronization of backend data.
- Zustand owns ephemeral UI state only; it must not become a second database for goals, tasks or projects.
- Persist user layout and preference state through backend settings services.
- After a mutation, return the updated record or invalidate the relevant query keys. Avoid manually maintaining multiple contradictory copies of the same record.

### 18.6.5 Desktop bridge and contracts

- Expose application use cases through named Tauri commands; do not expose arbitrary SQL or generic filesystem execution to the frontend.
- Define request and response DTOs with explicit fields and stable identifiers.
- Validate command inputs in Rust even if the frontend already validates them.
- Return structured errors with stable error codes, safe user-facing messages and diagnostic context.
- Use typed Tauri events for meaningful asynchronous updates such as timer changes, tracking state, sync progress and long-running AI execution.
- Keep event payloads small and versionable. Events notify the frontend; they are not the durable source of truth.
- Generate or centrally maintain TypeScript contract types so frontend and backend do not drift.
- Avoid exposing unrestricted shell, process execution or arbitrary path access through generic commands.

#### Tauri security requirements

- **Least-privilege capabilities:** each window and webview is granted only the specific Tauri permissions it needs. No broad or wildcard permission sets are enabled by default, and each added permission has a documented need.
- **Restrictive permission scopes:** any permission that supports scopes (for example filesystem or URL access) is limited to the narrowest paths or targets required. The frontend is not granted general filesystem access; note, attachment and authorized-resource access goes through Rust file services (18.6.10).
- **Content Security Policy:** configure a restrictive CSP in the Tauri configuration. Load only bundled application assets, no remote scripts, no `unsafe-eval`, and no inline scripts unless a documented need exists. AI provider and connector requests are made by Rust services (18.6.8, 18.6.11), not by the webview, so the webview does not need broad network access.
- **No unnecessary system access:** do not expose shell, process-execution or other broad operating-system capabilities to the webview. Tray, global shortcuts, startup launch and foreground-window observation are exposed only through specific Rust platform services (18.6.9).
- **Sensitive operations stay in Rust:** database access, credential access, filesystem access outside managed storage, external network calls, activity observation and tool execution remain behind controlled Rust application services reached through typed commands. Authorization is enforced in Rust, not in the UI.
- Changes to capabilities, scopes or the CSP are treated as security-relevant changes.

### 18.6.6 SQLite data architecture

SQLite is the authoritative local store for structured Ascend records. Use normalized relational tables, stable IDs and explicit many-to-many join tables.

Core schema groups:
- Identity/configuration: settings, workspace preferences, schema metadata.
- Core records: goals, milestones, projects, project types, tasks, task lists, subtasks, habits, routines, notes and calendar entries.
- Relationships: a generalized record relationship table with source record, target record, relationship type, optional metadata and timestamps. Enforce valid record references through application-level validation and, where practical, typed record tables/foreign keys.
- Tracking: timer sessions, activity observations, classified activity sessions, app rules and project attribution.
- Intelligence: conversations, messages, attachments, long-term memory items, memory categories, AI execution runs, execution steps, approvals and tool audit events.
- Integrations: connector definitions, connection metadata, granted capabilities, local resource grants and synchronization cursors. Secrets themselves are stored outside SQLite.
- Gamification: XP ledger, levels/configuration, achievements and quest progress.
- Operations: notification records, backup metadata and migration history.

Database rules:
- Use UUIDs or another stable globally unique identifier strategy consistently; do not use display names as identity.
- Store timestamps in UTC and convert to the user's configured local time zone for display and calendar calculations.
- Use transactions for operations that update multiple related records.
- Add indexes for foreign keys, common filters, relationship lookups and date-based queries.
- Enable SQLite foreign-key enforcement and WAL mode where supported and tested.
- Configure a bounded busy timeout and keep write transactions short.
- Use FTS5 virtual tables for initial full-text search, with explicit indexing/update/delete behavior.
- Do not add vector storage or an external database for the first release. Evaluate embeddings only after FTS5 and relationship-aware retrieval are measured against real search tasks.
- Store large binary attachments outside the SQLite database in managed application storage; keep metadata, hashes and relative managed paths in SQLite.
- Never store absolute local paths as the only identity for managed files. For user-authorized external folders, retain the canonical path and authorization metadata, and handle moved/missing paths as recoverable states.

#### Relationship integrity and deletion

The generalized relationship table cannot use database foreign keys to enforce references across different record types, so integrity is enforced by application services and schema safeguards.
- **Validation on creation:** a relationship may be created only when both endpoint records exist and the record types and relationship type are valid. A uniqueness rule prevents accidental duplicates of the same relationship (5.5).
- **Single deletion path:** permanent deletion of a record goes through the owning domain's application service. In one SQLite transaction it removes the record, every relationship in which the record is the source or the target, and the record's FTS5 index entries. The transaction commits entirely or rolls back entirely, so a relationship never points to a deleted record and a failed deletion leaves no partial state.
- **Cleanup removes links only:** deleting a record removes its relationships but never deletes or alters the other records it was connected to (5.6).
- **Archive and soft-delete:** archived or soft-deleted records still exist, so their relationships are retained. Relationships are removed only by permanent deletion or when the user explicitly removes the relationship.
- **File-system effects:** changes to managed files (for example note files and attachments) are not part of the SQLite transaction. They must be ordered and recorded so that any failure leaves a state that startup reconciliation (18.6.10) can detect and resolve without leaving dangling relationships.
- **Same path for every caller:** user actions and Ascend AI tools use the same deletion path, subject to the approval rules in 3.11.
- **Defense in depth:** queries resolve relationship endpoints against existing records so an orphaned relationship is never shown as valid. An integrity check detects relationships whose endpoint is missing; any orphan found is removed and the repair is logged.

### 18.6.7 Data and storage layout

Use Tauri's platform application data and cache directories rather than hard-coded installation paths.

Proposed managed layout:

```text
Ascend/
  data/
    ascend.sqlite
    migrations/
  files/
    notes/
    attachments/
    generated/
    imports/
  backups/
  logs/
  cache/
```

The actual root path must be resolved through Tauri's application directory APIs. Keep database, user files and backups in user-writable application data locations, not under Program Files. Cache and rebuildable indexes may be stored separately from durable user data.

### 18.6.8 AI runtime and tool execution architecture

- Ollama is the initial local provider and `llama3.1:8b` is the initial baseline model until benchmark results justify a change. It is a text model and does not provide image understanding (see 3.14).
- Implement a provider interface so a future OpenAI-compatible provider can be added without rewriting application tools.
- The Rust AI orchestration service owns provider requests, conversation assembly, mode instructions, context retrieval, tool-call parsing, tool dispatch and final response assembly.
- The model proposes tool calls; the application validates schemas, permissions, risk class and approval state before execution.
- The model never receives database credentials and never queries SQLite directly.
- All tools call application services. A tool is not allowed to implement a parallel version of domain logic.
- Tool outputs use structured results containing success state, stable record IDs, safe summaries and errors.
- Context retrieval combines structured queries, FTS5, relationship traversal and selective long-term memory retrieval. Workspace records remain authoritative for current state.
- Provider unavailability must not disable non-AI application features.
- AI execution runs as a cancellable, observable workflow. Persist run and step state so the user can inspect interrupted or failed operations.
- Do not introduce a second AI agent framework unless a concrete requirement cannot be met by the application's own orchestrator and Tool Registry.

#### Prompt-injection and untrusted-content enforcement

These requirements implement the rules in 3.11:
- Tool permission checks, schema validation, risk classification and approval-state checks are performed by the Rust Tool Registry and application services at dispatch time, independently for every tool call, and never delegated to model instructions.
- Retrieved content is passed to the model as clearly delimited data with source identifiers. Nothing found in retrieved content may create, modify or satisfy an approval, change a tool's permissions or risk class, or alter system instructions.
- Memory writes are validated by the backend. Automatic memory creation accepts only content that originates from the user's own messages, and the origin of memory content is determined by the application rather than asserted by the model. A memory created from retrieved material at the user's explicit request records that source (3.9).
- Approval decisions are accepted only through the application's trusted approval path, invoked by an explicit user action in the approval interface. No tool, model output or retrieved content can create or modify an approval record.
- A tool call proposed by the model is a proposal only; it is executed only after the application's own validation, authorization and approval checks pass.

### 18.6.9 Background services and Windows integration

Run long-lived work in Rust-managed services with explicit lifecycle and shutdown behavior:

- Activity sampling and classification.
- Timer state and notifications.
- Scheduled notification evaluation.
- Integration synchronization.
- AI task execution.
- Search indexing and file processing.
- Backup scheduling.

Use a shared application event mechanism to publish state changes. Background work must be bounded, cancellable where appropriate and designed not to block the UI thread.

Windows-specific behavior such as startup launch, tray integration, global shortcuts and foreground-window observation belongs behind platform service interfaces. Keep the domain layer platform-independent where practical. Activity tracking must honor user exclusions and privacy controls.

### 18.6.10 File and note storage

- Markdown note files are the portable source content for Notes and Knowledge.
- SQLite stores note metadata, stable IDs, relationships and indexing state.
- Managed attachments live under the application data directory and are referenced by relative managed paths.
- External folders are accessed only through explicit Local Resources grants.
- File operations must validate canonical paths and prevent path traversal outside the granted directory.
- Renames and moves should preserve stable note identity and internal links.
- Import/export operations should use temporary files and atomic replacement where supported to reduce partial-write risk.

#### Markdown synchronization with SQLite metadata

The local Markdown-first architecture is unchanged: Markdown files remain the portable source content and SQLite remains the metadata and index store. The following rules keep them consistent.

**Stable identity**
- Each note has a stable ID (consistent with the identifier strategy in 18.6.6) stored in the Markdown file's YAML frontmatter, in a dedicated field (proposed name `ascend_id`). Other frontmatter written by the user is preserved untouched.
- A Markdown file discovered without an ID is assigned one. Where the file is writable, the ID is written to its frontmatter using an atomic write; otherwise the note is tracked provisionally by path and content hash and flagged.
- SQLite records the note ID, relative managed path, content hash, last-known size and modification time, and indexing state. Internal links and relationships reference the note ID, never only the filename or path.

**Detecting changes**
- Where supported, a file watcher observes the managed notes directory for external creation, modification, rename, move and deletion. Events are debounced, and Ascend's own writes are recognized so they are not treated as external changes.
- On application startup, and whenever watcher events may have been missed, a reindex and reconciliation pass compares the files on disk with the SQLite metadata using IDs and content hashes. The watcher is an optimization; reconciliation is the source of consistency.

**Handling file events**
- **Modified:** the content hash differs from the last synchronized hash. The note's metadata, FTS5 index, links and backlinks are reindexed.
- **Renamed or moved:** matched by note ID (or by content hash when no ID is present). The stored path is updated and the note's identity, links and relationships are preserved.
- **Deleted:** a file that is missing on disk is marked as missing, which is a recoverable state (18.6.6). Its metadata and relationships are retained and the user is notified, and they can restore or relocate the file or remove the note. Relationships are removed only when the note is removed through the deletion flow in 18.6.6.
- **Duplicate IDs:** if the same ID appears in more than one file (for example a copied file), the original keeps the ID and the copy is assigned a new one and flagged. Neither file is discarded.
- **New file:** indexed as a new note and given an ID as described above.

**Conflicts and protection against silent loss**
- Ascend records the content hash it last loaded or saved for each note. Before writing a note, it compares the file on disk with that hash. If the file changed outside Ascend, this is a conflict and Ascend does not overwrite it.
- A conflict is also raised when an external change arrives while the note has unsaved edits in Ascend, or when the same note was modified both inside and outside Ascend since the last synchronization, including while the application was closed.
- Both versions are preserved and the user chooses how to resolve the conflict (for example keep the Ascend version, keep the disk version, or keep both as separate notes). The version that is not kept remains recoverable rather than being discarded.
- Notes are written with atomic replacement. Ascend does not rewrite a note merely to reindex it, apart from the one-time ID insertion above, and does not modify files outside granted locations.
- Autosave state and save errors remain visible to the user (18.3).

### 18.6.11 Integration and synchronization architecture

- Each connector implements a common internal interface for capability discovery, authorization status, read/write operations where supported, synchronization and error reporting.
- Keep provider-specific API models inside connector adapters; map them to stable Ascend domain DTOs.
- Track external IDs, source timestamps, sync cursors and last successful sync.
- Use explicit conflict states when both local and remote data changed.
- Do not silently overwrite ambiguous conflicts.
- Run sync in background services with bounded retries and backoff for transient failures.
- Local-only use must continue when connectors are unavailable.
- Never store OAuth tokens or API keys in connector records or logs.

### 18.6.12 Search and retrieval

Initial search stack:
1. Structured SQL for exact filters, statuses, dates and relationships.
2. SQLite FTS5 for full-text search across indexed records and note content.
3. Relationship traversal for connected workspace context.
4. Selective long-term memory retrieval.
5. Optional local embeddings and semantic retrieval in a later phase if measured quality requires them.

Search results must identify their source record and provide stable IDs so AI responses can cite or open the underlying record. Indexing must be updated on create, edit, move, archive and delete operations.

### 18.6.13 Backup, restore and migration

- Use SQLite's supported backup mechanism or a transactionally safe snapshot strategy; do not copy a live database file blindly.
- Back up database and managed files together with a manifest, schema version, timestamp and integrity metadata.
- Support manual backup before destructive migrations and scheduled backups as a configurable feature.
- Restore must validate the backup before replacing active data and preserve a recovery copy of the current database.
- Schema changes use ordered, versioned migrations and are tested from representative prior versions.
- Legacy import is a separate, inspectable process with a preview, mapping report and error report. Do not automatically import every old JSON file or unused table.
- Export should use documented portable formats where practical, especially Markdown for notes and CSV/JSON for structured records.

### 18.6.14 Build, packaging and update strategy

- Build and package the Windows application through Tauri's supported build pipeline.
- Keep development, test and production data directories separate.
- Pin and document toolchain versions for Rust, Node and package dependencies.
- Use CI to run formatting, linting, type checking, unit tests, integration tests and production builds.
- Sign production releases when signing credentials and distribution setup are available.
- Updates must not overwrite user data. Run database migrations with backup/recovery safeguards.
- Provide release notes and a clear update failure path. Automatic rollback should only be implemented when it can be made reliable and tested.

### 18.6.15 Architecture decisions intentionally deferred

These are implementation choices to validate with prototypes or measurements, not reasons to delay the architecture baseline:
- Exact SQLx feature configuration and query-generation approach.
- Exact frontend component library and design tokens; UI/UX will be designed later with Google Stitch and translated into production components.
- Exact event-bus abstraction beyond typed Tauri events.
- Exact database table/column definitions and indexes.
- Embedding model, vector index and semantic retrieval thresholds.
- Browser-tab observation mechanism and supported browser coverage. Detailed browser-tab tracking requires a browser extension or another explicitly supported integration and is deferred (see 11.2).
- Exact backup schedule and retention defaults.
- Update distribution provider and code-signing setup.
- Exact telemetry policy; default should remain local diagnostics without behavioral telemetry unless explicitly opted into.

## 18.7 Security, Reliability and Quality Baseline

- Local-first storage by default; external transmission occurs only when required by an enabled provider or integration and is made clear to the user.
- Store API keys and OAuth credentials in the OS secure credential store.
- Never log secrets, raw credentials or sensitive note contents unnecessarily.
- Validate all tool inputs and backend commands.
- Enforce authorization at execution time, not only in the UI.
- Treat retrieved content (notes, files, PDFs, webpages, GitHub issues, external services) as untrusted data that can never authorize actions, grant permissions or satisfy approvals; enforce tool permissions, schemas, risk classes and approvals outside the language model (3.11, 18.6.8). Retrieved content also never creates or modifies personal memory (3.9).
- Configure Tauri with least-privilege capabilities, restrictive permission scopes and a restrictive Content Security Policy (18.6.5).
- Use least privilege for integrations and local folders.
- Keep an audit trail for AI actions and consequential changes.
- Provide backup, restore and export; verify backups rather than assuming success.
- Use database transactions for multi-record consistency.
- Handle app crashes and interrupted workflows without corrupting records or duplicating external effects.
- Provide structured logs and user-readable diagnostics with privacy-conscious redaction.
- Test domain services, migrations, permissions, tool execution, recovery and critical UI flows.
- Include accessibility, performance and offline tests in release criteria.
- Never claim a feature is offline-capable, secure or recoverable until it has been tested.

## 18.8 Planning Exit Criteria

Product planning is considered sufficiently complete when:
- Every primary workspace has a defined purpose, core records, major interactions and important edge cases.
- Cross-module relationships and progress rules are coherent.
- AI behavior, permissions, memory, tools and execution safety are specified.
- Initial integrations and their access boundaries are defined.
- Gamification cannot distort or double-count real work.
- The design system and major workspace layouts are documented.
- The architecture has clear ownership boundaries and data migration principles.
- Security, backup, recovery and testing expectations are explicit.

The product direction and technical architecture are now sufficiently defined to proceed. UI/UX design will be developed separately with Google Stitch; Stitch is not a runtime dependency. After the Stitch design work is approved, translate it into production components and proceed through the Codex implementation roadmap. Do not reopen settled product questions without new evidence or a meaningful trade-off.


---

# 19. Tasks Workspace

Relocated from the original brainstorm (Section 15) in v5.2. The confirmed requirements are unchanged. Default behavior rules are in 18.3 (Tasks); relationship rules are in Section 05; timer connections are in 9.2.

## 19.1 Organization Model — Confirmed: Lists + Connected Records

- Tasks can be organized into user-created lists (for example: Personal, College, Work or Development).
- Tasks can also exist independently of a list.
- Lists are an organizational mechanism; they do not replace project, goal or milestone relationships.
- A task can belong to a list while independently connecting to one or more projects, goals and milestones.
- The same task is represented once and can appear in multiple views without duplication.
- Relationships remain governed by the shared connected-record model.

## 19.2 Views — Confirmed: Hybrid Task Workspace

- List view for everyday task management.
- Board/Kanban view for status-based workflows.
- Calendar view for date-based planning.
- Custom views using saved filters, grouping and sorting.
- All views operate on the same underlying task records; changes in one view are reflected everywhere.

## 19.3 Capabilities and Interaction Direction

- Tags, search, filters, sorting and customization.
- Completed, incomplete and overdue states.
- Quick task capture and keyboard-first interactions are desired.
- Natural-language date entry and repeating-task rules are useful reference capabilities.
- Notes and file attachments can be associated with tasks.
- Tasks connect to goals, milestones, projects and other supported records.
- Offline task access is consistent with Ascend's offline-first product principle.

## 19.4 Initial Task Structure — Confirmed

The initial task record should stay deliberately lightweight and use the following fields:
- Title
- Description
- Status
- Priority
- Start date
- Due date
- Tags
- List
- Relationships
- Subtasks

Do not expand the initial task model with additional fields or advanced customization pre-emptively. If real use during development reveals a missing field or capability, evaluate and add it then. The initial structure is approved as a sufficient starting point.

## 19.5 Deferred Details

exact status and priority options, recurring-task rules, natural-language date parsing, quick-capture behavior, saved-view customization and task completion semantics can be decided when their implementation becomes relevant. Custom task fields are not part of the initial scope.

Statuses, priorities, subtask completion and recurring-task creation defaults were subsequently decided in 18.3 (Tasks); the remaining deferred items above stay open.

---

# 20. Habits Workspace

Relocated from the original brainstorm (Section 15) in v5.2. The confirmed requirements are unchanged. Default behavior rules are in 18.3 (Habits and routines); how habits and routines interact is in 21.2.

## 20.1 Planned Functionality — Confirmed

- Habit types: Boolean (yes/no), measurable quantity, duration and count-based.
- Flexible schedules: daily, selected weekdays, weekly targets and custom intervals.
- Completion tracking and history.
- Streaks and consistency tracking.
- Missed, skipped and completed states.
- Reminders and notifications.
- Habit statistics and visualizations, including heatmaps.
- Connections to goals, milestones, projects and notes.

---

# 21. Routines Workspace

Relocated from the original brainstorm (Section 15) in v5.2. The confirmed requirements are unchanged. Default behavior rules are in 18.3 (Habits and routines).

## 21.1 Planned Functionality — Confirmed

- Morning, afternoon, evening and custom routines.
- Ordered activities within each routine.
- Individual activity durations.
- Optional tasks and habits inside routines.
- Routine schedules and reminders.
- Start, pause, resume and complete routine sessions.
- Progress tracking for individual activities and the overall routine.
- Completion history and statistics.
- Connections to goals and other workspace records.

## 21.2 Habit and Routine Interaction

- Habits track repeated behaviours over time; routines organize activities into structured sequences.
- A routine activity can reference an existing habit.
- Completing that activity can record the corresponding habit completion without counting it twice.
- Habit and routine tracking history can contribute to connected goal progress and analytics, subject to their respective progress rules.
