---
description: Create a technical specification for a new piece of work
---

# Prompt: Create Technical Specification

## Context

Use this prompt when you need the AI to help you write a technical specification document for a new piece of work. This is a shortcut that combines analysis and specification into a single interaction.

## Prerequisites

- AI has access to the **Project Definition**
- You have a description of what needs to be built or changed
- You know which specification template to use

## Input

Paste the following alongside this prompt:

1. Your **Project Definition** (if the AI doesn't already have it in context)
2. A **description of the work** (feature request, bug report, refactoring goal, etc.)
3. The **specification template** you want to use (copy from `templates/specification/`)

## Prompt

<!-- Copy from here -->

I need a technical specification for the following work. Please analyze the request and produce a complete specification using the template provided.

**Work description:**

{{WORK_DESCRIPTION}}

**Specification template to follow:**

{{PASTE_THE_TEMPLATE_HERE}}

**Instructions:**
1. First, briefly analyze the work (complexity, dependencies, key considerations)
2. Then fill in every section of the template with specific, actionable content
3. Ensure acceptance criteria are measurable and testable
4. Include test scenarios for happy path, edge cases, AND error cases
5. Follow the Project Definition's conventions for all code examples, naming, and patterns
6. Flag anything you're unsure about as an open question rather than guessing

The specification should be detailed enough that someone could implement it without needing to ask clarifying questions.

<!-- To here -->

## Expected Output

A complete specification document following the provided template, ready for review and implementation.

## Next Steps

- Review the specification for completeness and accuracy
- Resolve any open questions the AI flagged
- Use `start-implementation.md` to begin implementing, or file the spec for later
