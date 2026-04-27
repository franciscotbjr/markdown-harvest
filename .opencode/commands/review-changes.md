---
description: Perform a self-review of code changes before committing
---

# Prompt: Review Changes

## Context

Use this prompt to perform a self-review of code changes before committing or creating a pull request. The AI acts as a code reviewer.

## Prerequisites

- Code changes are complete
- AI has access to the **Project Definition** for convention checking

## Input

Paste the following alongside this prompt:

1. The **diff** or **list of changed files with descriptions**
2. The **specification** or **task description** that motivated the changes
3. Your **Project Definition** (if the AI doesn't already have it in context)

## Prompt

<!-- Copy from here -->

Please review the following code changes as if you were a code reviewer. Be thorough but constructive.

**Changes:**

{{DIFF_OR_CHANGE_SUMMARY}}

**Specification/Task:**

{{SPEC_OR_TASK_DESCRIPTION}}

Review the changes against these criteria:

1. **Correctness** — Does the code do what the spec says? Are there logic errors?
2. **Completeness** — Are all acceptance criteria from the spec addressed? Anything missing?
3. **Convention Compliance** — Does the code follow the Project Definition's naming, style, and patterns?
4. **Test Coverage** — Are there tests for happy path, edge cases, and error cases? Are any scenarios missing?
5. **Security** — Any exposed secrets, unsafe inputs, missing validation, or injection risks?
6. **Performance** — Any obvious performance issues (N+1 queries, unnecessary loops, missing indexes)?
7. **Cleanliness** — Any debug code, TODO comments, commented-out code, or temporary hacks left behind?
8. **Scope** — Does the change stay within the spec's boundaries? Any scope creep?

For each issue found, provide:
- **Severity:** Critical / Warning / Suggestion
- **Location:** File and line (if applicable)
- **Description:** What the issue is
- **Recommendation:** How to fix it

If the changes look good, say so — don't invent issues.

<!-- To here -->

## Expected Output

A structured review with findings categorized by severity, or a confirmation that the changes look clean.

## Next Steps

- Fix any Critical or Warning issues
- Consider Suggestions for improvement
- Proceed to commit or PR creation
