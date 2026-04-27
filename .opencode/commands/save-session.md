---
description: Save session progress by updating memory.md and iteration files
---

# Save Session

> Use at the end of a work session to persist progress. The AI automatically updates `.stateful-spec/memory.md` and the active iteration file.

## Instructions

You are helping the developer save their session progress. Your job is to update the project's memory files so the next session can resume seamlessly — whether it's the same developer or someone else.

**Your role:**
- Review what was accomplished in this session
- Update `.stateful-spec/memory.md` with current state
- Update the active iteration file with completed tasks
- Ensure nothing is lost between sessions

---

### STEP 1 — Review Session Work

Ask the developer:

> "Let me save your session progress. Can you briefly describe what we accomplished today, or should I summarize from our conversation?"

If the developer provides a summary, use it. Otherwise, generate a summary from the conversation context including:
- Tasks completed
- Files created or modified
- Decisions made
- Issues encountered

### STEP 2 — Read Current State

Read the following files:
- `.stateful-spec/memory.md` — Current project memory
- **Iteration files** in `.stateful-spec/history/` — identify the active one (status `in-progress` or referenced from memory.md **Active Work**), if any

### STEP 2.5 — No active iteration file?

If there was **substantial work** this session but **no** suitable iteration file exists (or none was in progress):

- **Preferred:** Create a **retroactive** iteration file `.stateful-spec/history/NNN-[name].md` using `templates/implementation/iteration.md`, with description and tasks reflecting what was done, status `done` or `review`, and link any commits in **References**.
- **Minimum:** Update `.stateful-spec/memory.md` only — add a row to **Recent Completions** or **Key Decisions** so the session is not lost.

Then continue with STEP 3 if you now have an iteration file to update; otherwise skip to STEP 4 (memory-only save).

### STEP 3 — Update Iteration File

**If there is no iteration file to update** (after STEP 2.5 you chose memory-only): skip to STEP 4.

Open the active iteration file (when present) and update it:

**Update the checklist:**
- Mark completed tasks as done (`- [x]`)
- Add any new tasks that were discovered
- Note any blockers in the Blockers section

**Add decisions:**
- Record any decisions made during this session in the Decisions table

**Update status:**
- If all acceptance criteria are met, change status to `done`
- If blocked, change status to `blocked` and note why

Show the developer the changes and ask:
> "Here are the updates to the iteration file. Does this look correct?"

Wait for confirmation before saving.

### STEP 4 — Update Memory

Update `.stateful-spec/memory.md`:

**Active Work section:**
- Update the status/description of active work
- If work was completed, move it to Recent Completions

**Recent Completions section:**
- Add any completed iterations with today's date

**Key Decisions section:**
- Add any significant decisions that should persist beyond this iteration

**Constraints & Reminders section:**
- Add any new constraints or important reminders discovered

Show the developer the changes and ask:
> "Here are the updates to memory.md. Does this look correct?"

Wait for confirmation before saving.

### STEP 5 — Confirm Save

After updating `.stateful-spec/memory.md` and (if applicable) an iteration file, tell the developer:

> "Session saved. Updated:"
> - `.stateful-spec/history/[iteration-file].md` — [brief summary of changes, or "skipped — memory-only save"]
> - `.stateful-spec/memory.md` — [brief summary of changes]
>
> "Next time, use `@resume-session` (Cursor), your agent's resume command, or `.stateful-spec/operations/resume-session.md` if native commands aren't set up, to pick up where you left off."

### STEP 6 — Suggest Commit (Optional)

If there are uncommitted changes, ask:

> "Would you like me to commit these memory updates? Suggested message:"
> ```
> chore: save session progress for [iteration name]
> ```

If yes, stage and commit the .stateful-spec/ changes.

## Output

After completing the save flow:

1. **Iteration file updated** — Tasks checked off, decisions recorded (or retroactive file created in STEP 2.5, or skipped if memory-only)
2. **Memory file updated** — Active work, completions, decisions persisted
3. **Ready for next session** — Any developer can resume with full context

## Quick Save

If the developer just says "save session" without details, do your best to:
1. Infer what was done from the conversation
2. Show proposed updates
3. Ask for confirmation before saving

Keep it fast — the developer wants to wrap up, not answer many questions.
