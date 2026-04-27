---
description: Restructure existing code without changing external behavior
---

# Prompt: Refactor Code

## Context

Use this prompt when you need to restructure existing code without changing its external behavior. The AI will help plan and execute the refactoring safely.

## Prerequisites

- AI has access to the **Project Definition**
- You know what code needs refactoring and why
- Existing tests pass (baseline for behavior preservation)

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. The **code to refactor** (paste the source or reference the files)
3. The **reason for refactoring** (what problem the current structure causes)
4. Optionally: what the **desired outcome** looks like

## Prompt

<!-- Copy from here -->

I need to refactor the following code. The refactoring must NOT change any external behavior — all existing tests must continue to pass.

**Code to refactor:**

{{CODE_OR_FILE_REFERENCES}}

**Why this needs refactoring:**

{{REASON}}

**Desired outcome (if known):**

{{DESIRED_STRUCTURE_OR_NONE}}

**Instructions:**
1. Analyze the current code and identify the specific problems
2. Propose a refactoring plan with ordered steps — each step should be independently committable
3. For each step, explain what changes and why
4. Implement the refactoring step by step
5. After each step, confirm that:
   - External behavior is unchanged
   - All existing tests should still pass
   - The code follows the Project Definition's conventions
6. If new tests are needed to cover the refactored code, write them
7. Do NOT change any public API, interface, or behavior unless explicitly discussed

If you think the refactoring should also change behavior (e.g., fix a bug discovered during refactoring), flag it separately — do not mix behavior changes with structural refactoring.

<!-- To here -->

## Expected Output

1. Analysis of current problems
2. Step-by-step refactoring plan
3. Refactored code (incrementally, step by step)
4. Confirmation of behavioral preservation at each step

## Next Steps

- Review each refactoring step
- Run tests after each step to confirm behavior is preserved
- Commit each step independently if possible
