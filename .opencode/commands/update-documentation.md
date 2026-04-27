---
description: Update project documentation after implementing changes
---

# Prompt: Update Documentation

## Context

Use this prompt after implementing a feature or change to ensure all relevant documentation is updated. This covers README, CHANGELOG, API docs, and architecture docs.

## Prerequisites

- Implementation is complete (Phase 4 done)
- AI has context about what was changed

## Input

Paste the following alongside this prompt:

1. A **summary of what was implemented** (or confirm the AI has it in context)
2. The **current version** of the project (for CHANGELOG entries)
3. Optionally: the current content of files that need updating

## Prompt

<!-- Copy from here -->

Please help me update the project documentation after the following changes.

**What was implemented:**

{{IMPLEMENTATION_SUMMARY}}

**Current project version:** {{VERSION}}

For each documentation file below, determine if it needs updating and produce the updated content:

1. **README.md** — Does the change affect setup instructions, usage examples, API surface, or feature list? If yes, provide the updated sections.

2. **CHANGELOG.md** — Add an entry for this change under the current version. Use this format:
   ```
   ## [version] - YYYY-MM-DD
   ### Added / Changed / Fixed / Removed
   - Description of the change
   ```

3. **API Documentation** — If public interfaces changed, update or create the relevant API documentation.

4. **Architecture Documentation** — If the project structure, module organization, or patterns changed, update architecture docs.

5. **Other** — Are there any other docs that should be updated? (e.g., deployment guides, configuration docs)

For each file, either provide the full updated content or clearly mark the sections that changed.

<!-- To here -->

## Expected Output

Updated content for each documentation file that needs changes, clearly indicating what was added or modified.

## Next Steps

- Review and apply the documentation updates
- Include doc updates in the same commit or a follow-up commit
