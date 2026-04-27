# Phase 3: Specify

**Goal:** Write detailed technical refinement documents for each unit of work so implementation can proceed without ambiguity.

## When to Enter This Phase

- After completing the Plan phase
- When a milestone from the plan needs detailed specification before coding

## Inputs

- Plan output (architecture, milestones, blockers resolved)
- Project Definition (`templates/project/project-definition.md`)
- Specification templates from `templates/specification/`

## Activities

### 1. Select the Right Template

Choose the specification template that matches the type of work:

| Work Type | Template | When to Use |
|-----------|----------|-------------|
| New feature | `feature-spec.md` | Any new capability being added |
| API endpoint | `endpoint-spec.md` | REST, GraphQL, gRPC, or WebSocket endpoint |
| UI/Module/Service | `component-spec.md` | Frontend component, backend module, or service |
| Bug fix | `bugfix-spec.md` | Analyzing and fixing a defect |
| Refactoring | `refactor-spec.md` | Restructuring code without changing behavior |

### 2. Fill the Specification

For each unit of work within the current milestone, fill out the selected template. Key sections common to all specs:

- **Description** — What is being built or changed
- **Acceptance Criteria** — How to verify the work is complete
- **Data/Type Definitions** — Inputs, outputs, data structures
- **Test Scenarios** — What to test (happy path, edge cases, error cases)
- **Dependencies** — What this spec depends on
- **Implementation Notes** — Hints, constraints, known pitfalls

### 3. Review the Specification

Before moving to implementation, verify:

- Is the spec **complete enough** to implement without guesswork?
- Are the **acceptance criteria measurable** and testable?
- Are **edge cases** and **error scenarios** covered?
- Does it **align with the Project Definition** conventions?

### 4. Store the Specification

Save completed specs in a location defined by the project. Common patterns:

- `docs/specs/` or `spec/` directory in the repo
- Issue tracker (GitHub Issues, Jira, Linear)
- Wiki or documentation system

The methodology does not prescribe where specs live — that's a project-level decision.

## Outputs

1. **One or more filled specification documents** — Each covering a discrete unit of work
2. **Updated milestone plan** — If the spec reveals new sub-tasks or changes

## Completion Criteria

- [ ] Every unit of work in the current milestone has a specification
- [ ] Each spec has measurable acceptance criteria
- [ ] Test scenarios cover happy path, edge cases, and error cases
- [ ] Specs reference the Project Definition's conventions where applicable
- [ ] No implementation ambiguity remains

## Anti-Patterns

- **Spec as formality** — Writing a spec after the code is already written
- **Spec as novel** — Over-specifying trivial work
- **Missing error cases** — Only specifying the happy path
- **Detached from conventions** — Ignoring the Project Definition's patterns and naming rules

## Guidance: When to Skip or Shorten

Not every task needs a full spec:

- **Trivial changes** (typos, config, dependency bumps) — Skip this phase entirely
- **Simple bug fixes** (obvious cause) — A brief `bugfix-spec.md` with root cause and fix is sufficient
- **Exploration/prototyping** — Write a minimal spec with the question you're trying to answer

## Next Phase

After specifications are complete, proceed to [Phase 4: Implement](04-implement.md).
