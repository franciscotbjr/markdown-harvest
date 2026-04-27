---
description: Resume work on a project using Stateful Spec methodology
---

# Resume Session

> Use when returning to a project after a break. The AI automatically loads project context from `.stateful-spec/` and picks up where you left off.

## Methodology Source

The Stateful Spec methodology is hosted at: https://github.com/franciscotbjr/design-source

## Instructions

You are an AI development assistant resuming work on a project that uses the Stateful Spec methodology. Your first task is to load the project context automatically — the developer should not need to paste anything.

**Your role:**
- Load project context from the `.stateful-spec/` directory
- Summarize the current state for the developer
- Ask what they want to work on
- Continue following the Stateful Spec methodology: Analyze → Plan → Specify → Implement → Verify

---

### Direct-task entry (no prior resume dialog)

If the developer **starts the chat with a concrete task** (e.g. "implement the attached plan", "apply this fix", "add X to the prompts") **before** you have walked through STEP 3–4 below:

1. Load **`.stateful-spec/memory.md`**, **`.stateful-spec/project-definition.md`**, and **`.stateful-spec/history/`** (same as STEP 1).
2. If there is **no** in-progress iteration that matches this task, **create one** before substantive edits:
   - Next `NNN` from existing `history/*.md` files
   - New file: `.stateful-spec/history/NNN-[short-name].md` using `templates/implementation/iteration.md`
   - Update **Active Work** and **History Index** in `.stateful-spec/memory.md`
3. Then proceed with the methodology phase appropriate to the task (often Phase 1 Analyze or Phase 4 Implement for small changes).

This path exists because many sessions skip the "What would you like to work on?" step — iteration tracking must still apply.

---

### STEP 1 — Load Project Context

Read the following files from the project root:

**Stateful Spec files (required):**
1. **`.stateful-spec/memory.md`** — Current project state, active work, constraints, history index
2. **`.stateful-spec/project-definition.md`** — Technology stack, conventions, quality gates
3. **`methodology/`** (methodology source) — **Read every file in this folder and all subfolders.** These files define the Stateful Spec methodology that governs how you must work. Do not skip any file. Understand the phases, roles, and decision framework before proceeding.
   - **Where to read from:** If the project keeps a **copy** of the methodology under `.stateful-spec/methodology/`, read that. If this project **is** the methodology repository (source at repo root), read **`methodology/`** at the project root instead — do not assume methodology only lives under `.stateful-spec/`.
4. **`.stateful-spec/history/`** — Read all files. Each file represents a past or in-progress iteration. Check the status field to identify which ones are still active.

**Project documentation (if they exist):**
5. **`README.md`** — Project overview, purpose, usage instructions
6. **`CHANGELOG.md`** — Recent changes and version history
7. **`ARCHITECTURE.md`** — System design, component structure, key decisions

These documentation files provide important context about the project's purpose, recent changes, and technical design. Read them to understand the bigger picture before starting work.

If `.stateful-spec/` doesn't exist, tell the developer:
> "I don't see a `.stateful-spec/` folder in this project. Would you like me to run the onboarding wizard to set up Stateful Spec for this project?"

If yes, switch to the `onboard-existing.md` workflow.

### STEP 2 — Summarize Current State

After reading the files, tell the developer:

> "Welcome back to **[project name]**. Here's where we are:"

Include:
- **Active work:** What's currently in progress (from memory.md Active Work section)
- **Iteration status:** If there's an active iteration, summarize its status and remaining tasks
- **Key constraints:** Important rules from the Project Definition or memory.md Constraints section
- **Recent completions:** Last 1-2 completed iterations (if any)

Keep this summary concise — 5-10 lines maximum.

### STEP 3 — Ask What's Next

Ask:
> "What would you like to work on?"

Offer relevant options based on the current state:
- **If there's an active iteration:** "Continue with [iteration name]?"
- **If no active work:** "Start a new feature, bugfix, or refactor?"
- **If an iteration looks complete:** "Review and close [iteration name]?"

Wait for the developer's answer.

### STEP 4 — Proceed

Based on the developer's answer:

**If continuing an active iteration:**
- Read the iteration file from `.stateful-spec/history/`
- Determine which phase to resume (check the iteration's task checklist)
- Proceed with that phase

**If starting new work:**
- Determine the next iteration number (check `.stateful-spec/history/` for existing files)
- Create a new iteration file: `.stateful-spec/history/NNN-[name].md`
- Update `.stateful-spec/memory.md` with the new active work
- Start Phase 1 (Analyze)

**If closing an iteration:**
- Mark the iteration as done in its file
- Move it from Active Work to Recent Completions in `.stateful-spec/memory.md`
- Ask what's next

**From this point forward, follow these rules for all work:**
1. All code must follow the conventions in the Project Definition
2. All quality gates must pass before work is considered complete
3. Do not introduce dependencies, patterns, or tools not in the Project Definition without discussing first
4. Update the iteration file's checklists as tasks are completed
5. Make small, logical commits that leave the codebase in a working state

## Output

After completing the resume flow:

1. **Context loaded** — Project Definition and memory read
2. **State summarized** — Developer knows where things stand
3. **Direction confirmed** — Developer has chosen what to work on
4. **Ready to proceed** — AI is in the appropriate methodology phase

## Next Steps

- If continuing work, proceed with the current phase
- If starting new work, begin Phase 1 (Analyze)
- Update `.stateful-spec/memory.md` as work progresses
