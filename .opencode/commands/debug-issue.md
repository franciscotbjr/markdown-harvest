---
description: Diagnose and fix a bug with root cause analysis
---

# Prompt: Debug Issue

## Context

Use this prompt when you encounter a bug or unexpected behavior and need AI assistance to diagnose and fix it.

## Prerequisites

- AI has access to the **Project Definition**
- You can describe the problem (error message, unexpected behavior, failing test)

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. The **problem description** (error message, unexpected behavior, steps to reproduce)
3. The **relevant code** (paste source files, stack traces, logs)
4. What you've **already tried** (if anything)

## Prompt

<!-- Copy from here -->

I need help debugging an issue. Please help me find the root cause and fix it.

**Problem:**

{{PROBLEM_DESCRIPTION}}

**Error message / stack trace / logs:**

{{ERROR_OUTPUT}}

**Relevant code:**

{{CODE_OR_FILE_REFERENCES}}

**What I've already tried:**

{{ATTEMPTS_OR_NONE}}

**Instructions:**
1. Analyze the error and the relevant code
2. Identify the most likely root cause — explain your reasoning
3. If you're not certain, list the top 2-3 possible causes ranked by likelihood
4. For each possible cause, suggest a diagnostic step (a log statement, a test, a check) to confirm or eliminate it
5. Once the root cause is identified, propose a minimal fix
6. The fix should:
   - Address the root cause, not just the symptom
   - Be as small as possible — prefer single-line changes when sufficient
   - Not introduce new issues
7. Suggest a regression test that would catch this bug in the future

Do NOT make broad changes or refactor during debugging. Focus on understanding and fixing the specific issue.

<!-- To here -->

## Expected Output

1. Root cause analysis with reasoning
2. Diagnostic steps (if cause is uncertain)
3. Minimal fix with explanation
4. Suggested regression test

## Next Steps

- Verify the root cause with the suggested diagnostic steps
- Apply the fix
- Add the regression test
- Run quality gates to confirm nothing else broke
