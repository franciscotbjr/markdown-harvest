---
description: Generate a well-structured commit message following project conventions
---

# Prompt: Write Commit Message

## Context

Use this prompt after completing a unit of work to generate a well-structured commit message that follows the project's conventions.

## Prerequisites

- Code changes are staged or ready to commit
- AI has context about what was changed and why

## Input

Paste the following alongside this prompt:

1. A **summary of changes** (or a diff/file list)
2. The **specification or task** that motivated the changes (optional but recommended)

## Prompt

<!-- Copy from here -->

Please write a commit message for the following changes.

**Changes made:**

{{CHANGE_SUMMARY_OR_DIFF}}

**Task/specification context:**

{{TASK_CONTEXT_OR_NONE}}

**Rules for the commit message:**
1. Use a short, imperative subject line (50 characters max) — e.g., "Add user authentication endpoint"
2. Leave a blank line after the subject
3. Write a body explaining WHAT changed and WHY (not HOW — the code shows how)
4. If this closes an issue or relates to a ticket, include the reference
5. Keep the total message concise — aim for clarity, not completeness

Format:
```
<subject line>

<body — what changed and why>

<optional: references>
```

<!-- To here -->

## Expected Output

A ready-to-use commit message.

## Next Steps

- Review the message, adjust if needed
- Commit with the generated message
