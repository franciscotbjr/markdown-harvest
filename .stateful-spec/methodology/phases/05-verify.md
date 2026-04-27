# Phase 5: Verify

**Goal:** Confirm the work meets quality standards, update documentation, and prepare for delivery.

## When to Enter This Phase

- After completing the Implement phase
- Before merging, releasing, or deploying

## Inputs

- Implemented code with tests (from Phase 4)
- Project Definition — specifically the **Quality Gates** section
- Specification documents — for acceptance criteria verification

## Activities

### 1. Run Quality Gates

Execute every quality gate defined in the Project Definition. Common examples:

| Gate | Example Commands | Purpose |
|------|-----------------|---------|
| Linter | `eslint .`, `cargo clippy`, `ruff check .` | Code quality |
| Formatter | `prettier --check .`, `cargo fmt --check`, `black --check .` | Consistent style |
| Type checker | `tsc --noEmit`, `mypy .`, `cargo check` | Type safety |
| Tests | `jest`, `cargo test`, `pytest` | Correctness |
| Build | `npm run build`, `cargo build`, `go build ./...` | Compilation |

**All gates must pass.** If any gate fails, fix the issues before proceeding.

### 2. Verify Acceptance Criteria

Go through the specification's acceptance criteria one by one:

- [ ] Criterion 1 — Verified by [test name / manual check]
- [ ] Criterion 2 — Verified by [test name / manual check]
- [ ] ...

If any criterion is not met, return to the Implement phase.

### 3. Review Changes

Perform a self-review (or use the `prompts/operations/review-changes.md` prompt):

- **Diff review** — Are all changes intentional? Any debug code left?
- **Convention check** — Does everything follow the Project Definition?
- **Scope check** — Did the implementation stay within the spec's boundaries?
- **Security check** — No secrets, no unsafe inputs, no exposed internals?

### 4. Update Documentation

Use the `prompts/operations/update-documentation.md` prompt or manually update:

- **README** — If the feature changes setup, usage, or API surface
- **CHANGELOG** — Add entry for the change
- **API docs** — If public interfaces changed
- **Architecture docs** — If structure or patterns changed
- **ADRs** — If new decisions were made during implementation

### 5. Prepare Delivery Artifact

Depending on the project:

- **Write commit message** — Use `prompts/operations/write-commit-message.md`
- **Create PR/MR** — With description referencing the spec
- **Tag release** — If this completes a version milestone
- **Deploy** — If the project has a deployment pipeline

## Outputs

1. **All quality gates passing** — Documented or evident from CI
2. **Updated documentation** — README, CHANGELOG, API docs as needed
3. **Clean delivery artifact** — Commit, PR, release tag, or deployment

## Completion Criteria

- [ ] All quality gates from the Project Definition pass
- [ ] All acceptance criteria from the spec are verified
- [ ] Changes are self-reviewed (no debug code, no scope creep)
- [ ] Documentation is updated
- [ ] Delivery artifact is prepared (commit message, PR, etc.)

## Anti-Patterns

- **Skipping quality gates** — "It works on my machine"
- **Forgetting documentation** — Code ships without updated docs
- **Scope creep in verify** — Adding "just one more thing" during review
- **No acceptance check** — Declaring done without verifying the spec

## After Verification

The iteration is complete. Next steps:

- **More milestones remaining?** — Return to [Phase 3: Specify](03-specify.md) for the next milestone
- **New work unit?** — Return to [Phase 1: Analyze](01-analyze.md)
- **Session ending?** — Use `prompts/operations/save-session.md` to generate a session summary
