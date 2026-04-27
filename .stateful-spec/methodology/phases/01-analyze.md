# Phase 1: Analyze

**Goal:** Understand what needs to be built, break down complexity, and identify unknowns before committing to a plan.

## When to Enter This Phase

- Starting a new feature or work unit
- Receiving a bug report or change request
- Beginning a refactoring effort
- Onboarding to an unfamiliar part of the codebase

## Inputs

- Feature request, bug report, or change description
- Existing codebase (if applicable)
- Project Definition (`templates/project/project-definition.md`)

## Activities

### 1. Requirement Decomposition

Break the request into discrete, understandable pieces:

- **What** is being asked for? (functional requirement)
- **Why** is it needed? (business context, user need)
- **Who** will use it? (end user, developer, system)
- **What are the boundaries?** (what is explicitly out of scope)

### 2. Complexity Assessment

Categorize each piece by complexity:

| Complexity | Characteristics | Typical Effort |
|------------|----------------|----------------|
| **Simple** | Few moving parts, well-understood pattern, no unknowns | Single iteration |
| **Medium** | Multiple components, some integration, minor unknowns | 1-2 iterations |
| **Complex** | Cross-cutting concerns, new patterns, significant unknowns | Multiple iterations |

### 3. Dependency Mapping

Identify what this work depends on and what depends on it:

- **Upstream:** What must exist before this can be built?
- **Downstream:** What will break or change as a result?
- **External:** Are there third-party APIs, services, or libraries involved?

### 4. Unknown Identification

List what you don't know yet:

- Technical unknowns (how does X work? can we use Y?)
- Requirement unknowns (what should happen when Z?)
- Risk unknowns (what could go wrong?)

For each unknown, decide: **research now** or **defer to planning phase**.

## Outputs

A clear understanding documented as:

1. **Requirements summary** — What we're building and why
2. **Complexity breakdown** — Simple/medium/complex categorization of sub-tasks
3. **Dependency list** — What this work touches
4. **Open questions** — Unknowns that need resolution

This can be a section in an issue, a chat message, or a dedicated analysis document. The format matters less than having the information explicitly stated.

## Completion Criteria

- [ ] Requirements are understood and stated in your own words
- [ ] Complexity is assessed for each sub-task
- [ ] Dependencies are identified
- [ ] Open questions are listed (even if empty)
- [ ] No major unknowns remain unacknowledged

## Anti-Patterns

- **Jumping to code** — Writing code before understanding what's needed
- **Assuming requirements** — Filling in gaps without confirming
- **Ignoring complexity** — Treating everything as "simple"
- **Analysis paralysis** — Spending too long analyzing when a prototype would clarify faster

## Next Phase

After analysis is complete, proceed to [Phase 2: Plan](02-plan.md).
