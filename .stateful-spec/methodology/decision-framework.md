# Decision Framework

A lightweight process for making, recording, and revisiting architectural and design decisions during AI-assisted development.

## When to Use This Framework

Use this framework when facing a choice that:

- Affects the project's structure, patterns, or conventions
- Is difficult or costly to reverse later
- Has multiple reasonable alternatives
- Might be questioned by future contributors (including future you)

**Not every choice needs a formal record.** Variable names, minor refactors, and obvious patterns don't require an ADR. Use judgment.

## Decision Process

### Step 1: Identify the Decision

State the decision clearly:

> "We need to decide [what] because [why]."

Examples:
- "We need to decide on the error handling pattern because multiple approaches are viable."
- "We need to decide whether to use a monorepo or multi-repo because it affects CI and dependency management."

### Step 2: List Alternatives

For each alternative, document:

| Alternative | Pros | Cons |
|------------|------|------|
| Option A | ... | ... |
| Option B | ... | ... |
| Option C | ... | ... |

### Step 3: Apply Decision Criteria

Evaluate alternatives using these criteria (in order of priority):

1. **Consistency** — Does it align with existing patterns in the project?
2. **Simplicity** — Is it the simplest solution that works?
3. **Reversibility** — Can we change this later without major rework?
4. **Convention** — Is it idiomatic for the technology stack?
5. **Team familiarity** — Will contributors understand it?

### Step 4: Decide and Record

Make the decision and record it using the Architecture Decision Record template (`templates/project/architecture-decision.md`).

The record should capture:
- **What** was decided
- **Why** this option was chosen
- **What alternatives** were considered
- **What consequences** are expected

### Step 5: Communicate

Ensure the decision is visible:
- Reference the ADR in relevant code comments
- Mention it in PR descriptions
- Update the Project Definition if the decision changes conventions

## Handling Uncertainty

When you're unsure:

| Situation | Action |
|-----------|--------|
| Missing information | Ask the human — don't guess |
| Multiple good options | Pick the simplest; record why in an ADR |
| Conflicting requirements | Surface the conflict to the human for resolution |
| No precedent in the project | Check the Project Definition, then propose and ask |
| Time pressure | Make the reversible choice; create a follow-up task for the better solution |

## Revisiting Decisions

Decisions are not permanent. Revisit when:

- New information makes the original choice suboptimal
- The project's constraints have changed
- The decision is causing recurring problems

When revisiting, create a **new ADR** that supersedes the old one. Don't delete old ADRs — they provide context for why things changed.

## Quick Reference

```
Is this decision significant?
  ├── No  → Just decide and move on
  └── Yes → 
       ├── List alternatives
       ├── Apply criteria (consistency > simplicity > reversibility)
       ├── Choose and record (ADR)
       └── Communicate (PR, comments, Profile update)
```
