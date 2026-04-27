# Roles: Human and AI Assistant

This document defines the responsibilities, expectations, and boundaries for both the human developer and the AI assistant when working within the Stateful Spec methodology.

## The Human Developer

### Responsibilities

- **Owns all decisions** — Architecture, scope, priorities, trade-offs
- **Provides context** — Fills the Project Definition, writes specs, describes requirements
- **Drives the process** — Decides when to advance phases, what to build next
- **Reviews and approves** — Validates AI output before committing
- **Manages the project** — Version control, deployments, stakeholder communication

### Expectations

- Fill the Project Definition before starting AI-assisted work
- Provide clear, specific requests (not vague "make it better")
- Review AI-generated code before committing — AI output is a draft, not final
- Record decisions using Architecture Decision Records when significant
- Run quality gates and fix issues the AI cannot

## The AI Assistant

### Responsibilities

- **Follows the methodology** — Stays in the current phase, produces expected outputs
- **Respects the Project Definition** — Follows the project's conventions, patterns, and constraints
- **Produces concrete artifacts** — Code, specs, plans, documentation — not just advice
- **Asks clarifying questions** — When requirements are ambiguous or incomplete
- **Identifies risks** — Points out edge cases, potential issues, and trade-offs

### Expectations

- Read the Project Definition at the start of every session
- Follow the project's naming conventions, code style, and patterns
- Write tests alongside code (not as an afterthought)
- Stay within the scope of the current specification
- Never skip quality gates or declare work done prematurely
- Be explicit about uncertainty — say "I'm not sure about X" rather than guessing

### Boundaries

The AI assistant should **NOT**:

- Make architectural decisions without human approval
- Skip methodology phases (e.g., jump from Analyze to Implement)
- Introduce dependencies, patterns, or tools not in the Project Definition without discussion
- Commit, deploy, or make irreversible changes without explicit instruction
- Remove or weaken tests without explicit direction
- Assume requirements — ask when unclear

## Collaboration Model

### Session Start

1. AI reads the Project Definition
2. AI reads any session context from a previous session (if resuming)
3. Human states the current goal and phase
4. Work begins

### During a Session

1. Human provides direction (feature request, spec, bug report)
2. AI follows the methodology phase appropriate to the request
3. AI produces artifacts (code, specs, docs)
4. Human reviews and provides feedback
5. Iterate until the work unit is complete

### Session End

1. Use the `save-session` prompt to save progress
2. The AI updates `memory.md` with: decisions made, files changed, next steps, open blockers
3. Human reviews and commits the updates

## Communication Guidelines

### AI Should

- Be concise and direct — avoid unnecessary preamble
- Use the project's terminology (from the Project Definition)
- Reference specific files, functions, and line numbers
- Provide rationale for non-obvious choices
- State assumptions explicitly
- Present selectable options (numbered or bulleted choices) when asking questions — only use open-ended free-text input when the answer cannot be anticipated (e.g., project name, description)

### AI Should Not

- Produce walls of explanation when code is what's needed
- Repeat information the human already knows
- Add unsolicited features or improvements
- Use generic advice when project-specific guidance is available
