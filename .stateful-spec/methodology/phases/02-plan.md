# Phase 2: Plan

**Goal:** Define the architecture, structure, delivery milestones, and identify blockers before writing specifications.

## When to Enter This Phase

- After completing the Analyze phase
- When the scope or architecture of existing work needs to change significantly

## Inputs

- Analysis output (requirements, complexity, dependencies, open questions)
- Project Definition (`templates/project/project-definition.md`)
- Existing codebase architecture (if applicable)

## Activities

### 1. Architecture Design

Define how the work fits into the system:

- **Where does this code live?** — Which modules, packages, directories, or services?
- **What are the interfaces?** — Public APIs, data contracts, event boundaries
- **What patterns apply?** — Reference the Project Definition's conventions section
- **What changes to existing code?** — New files, modified files, deleted files

For significant decisions, use the Architecture Decision Record template (`templates/project/architecture-decision.md`).

### 2. Milestone Definition

Break the work into ordered, deliverable milestones:

```
Milestone 1: [Description] — delivers [what's usable]
Milestone 2: [Description] — delivers [what's usable]
Milestone 3: [Description] — delivers [what's usable]
```

Each milestone should:
- Be independently testable
- Produce working (not broken) software
- Be small enough to complete in one session

### 3. Blocker Identification

List anything that could prevent progress:

| Blocker | Type | Resolution |
|---------|------|------------|
| Need decision on X | Decision | Ask stakeholder / decide now |
| Dependency Y not ready | Technical | Wait / mock / work around |
| Unclear requirement Z | Requirement | Clarify with requester |

### 4. Risk Assessment

For complex work, identify risks:

- **What could go wrong?** (technical failure, scope creep, integration issues)
- **What's the fallback?** (simpler approach, feature flag, rollback plan)
- **What needs a spike/prototype first?** (unknowns that need code to resolve)

## Outputs

1. **Architecture sketch** — Where code lives, what interfaces look like
2. **Milestone list** — Ordered delivery plan with clear boundaries
3. **Blocker list** — Issues that need resolution (may be empty)
4. **ADRs** — Architecture Decision Records for significant choices (if any)

## Completion Criteria

- [ ] Architecture is defined (modules, interfaces, patterns)
- [ ] Milestones are listed and ordered
- [ ] Each milestone is independently testable
- [ ] Blockers are identified and have resolution paths
- [ ] Decisions are recorded (ADRs if significant)

## Anti-Patterns

- **Over-planning** — Planning every detail of a simple task
- **Big bang delivery** — One milestone that delivers everything at once
- **Ignoring blockers** — Starting implementation with unresolved blockers
- **Planning without the Project Definition** — Making architecture decisions without referencing the project's conventions

## Next Phase

After planning is complete, proceed to [Phase 3: Specify](03-specify.md).
