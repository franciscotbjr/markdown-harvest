# Phase 4: Implement

**Goal:** Build, test, and integrate the work described in the specification, following the project's conventions.

## When to Enter This Phase

- After completing the Specify phase (or after Analyze for trivial work)
- When specifications are reviewed and ready for coding

## Inputs

- Specification documents (from Phase 3)
- Implementation plan (if created using `templates/implementation/implementation-plan.md`)
- Project Definition (`templates/project/project-definition.md`)

## Activities

### 1. Implementation Order

Follow this sequence for each unit of work:

```
1. Data/Types     — Define the data structures, types, schemas
2. Core Logic     — Implement the business logic or algorithm
3. Integration    — Wire into the rest of the system (routes, handlers, UI)
4. Tests          — Write tests alongside or immediately after code
5. Documentation  — Inline docs, comments for non-obvious logic
```

This order can vary by project type, but the principle is: **build from the inside out** — start with the data, then the logic, then the integration layer.

### 2. Follow the Project Definition

Reference the Project Definition during implementation for:

- **Naming conventions** — Variables, functions, files, classes
- **Code style** — Formatting, indentation, import order
- **Patterns** — Error handling, logging, configuration
- **File organization** — Where new files go, module structure

### 3. Write Tests

Testing strategy depends on the project (see Project Definition), but the methodology requires:

- **At minimum:** Tests that verify the acceptance criteria from the spec
- **Recommended:** Tests for edge cases and error scenarios listed in the spec
- **If applicable:** Integration tests that verify the component works within the system

Use the `templates/implementation/test-plan.md` template for complex testing scenarios.

### 4. Commit Incrementally

Make small, logical commits as you implement:

- Each commit should leave the codebase in a working state
- Group related changes together
- Use the `prompts/operations/write-commit-message.md` prompt for consistent commit messages

### 5. Handle Blockers During Implementation

If you discover something unexpected:

1. **Minor issue** — Note it and continue; address after the current unit
2. **Spec gap** — Return to the Specify phase for the affected area
3. **Architecture change needed** — Return to the Plan phase
4. **Requirement unclear** — Pause and clarify with the requester

## Outputs

1. **Working code** — Implements the specification
2. **Tests** — Verify the acceptance criteria
3. **Inline documentation** — Comments and doc strings as needed
4. **Commits** — Clean, incremental commit history

## Completion Criteria

- [ ] All acceptance criteria from the spec are met
- [ ] Tests pass (unit, integration, as applicable)
- [ ] Code follows the Project Definition's conventions
- [ ] No hardcoded values, secrets, or temporary hacks left in
- [ ] Commits are clean and incremental

## Anti-Patterns

- **Implementing without a spec** — Writing code without knowing what "done" looks like
- **Big bang commits** — One commit with all changes
- **Tests as afterthought** — Writing all tests after all code is done
- **Ignoring conventions** — Deviating from the Project Definition without an ADR
- **Gold plating** — Adding features or polish not in the specification

## Next Phase

After implementation is complete, proceed to [Phase 5: Verify](05-verify.md).
