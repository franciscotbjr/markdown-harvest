---
description: Write tests for existing or newly implemented code
---

# Prompt: Write Tests

## Context

Use this prompt when you need the AI to write tests for existing or newly implemented code. This is useful for improving test coverage or when tests weren't written alongside implementation.

## Prerequisites

- AI has access to the **Project Definition** (for testing framework and conventions)
- The code to be tested exists
- Optionally: a specification with test scenarios defined

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. The **code to be tested** (paste the source or reference the files)
3. Optionally: the **specification** with test scenarios
4. Optionally: **existing tests** for context on testing patterns used in the project

## Prompt

<!-- Copy from here -->

Please write tests for the following code. Follow the project's testing conventions from the Project Definition.

**Code to test:**

{{CODE_OR_FILE_REFERENCES}}

**Specification with test scenarios (if available):**

{{SPEC_OR_NONE}}

**Existing test examples for pattern reference (if available):**

{{EXISTING_TESTS_OR_NONE}}

**Instructions:**
1. Use the testing framework and patterns specified in the Project Definition
2. Follow the project's test naming convention
3. Write tests for:
   - **Happy path** — Normal, expected usage
   - **Edge cases** — Boundary values, empty inputs, large inputs, null/undefined
   - **Error cases** — Invalid inputs, failure scenarios, error handling
4. Each test should be independent — no test should depend on another test's state
5. Use descriptive test names that explain what is being verified
6. Include setup/teardown if needed (fixtures, mocks, test data)
7. Keep tests focused — one assertion concept per test

Place tests in the location specified by the Project Definition (co-located with source, tests/ directory, etc.).

<!-- To here -->

## Expected Output

Complete test file(s) with tests covering happy path, edge cases, and error cases, following the project's testing conventions.

## Next Steps

- Review the generated tests
- Run them to verify they pass
- Add any additional scenarios specific to your domain knowledge
