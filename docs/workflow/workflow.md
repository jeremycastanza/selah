# Workflow

_Last updated: YYYY-MM-DD_

This file documents how you collaborate with AI agents in this project.

## Before Each Session

First message:

```
Read docs/workflow/context.md and docs/workflow/scope.md.
Propose a plan. List what we will NOT build.
```

## Build Loop

### Phase 1: Framing

1. Update `docs/workflow/scope.md` with objective, success criteria, constraints
2. Update `docs/workflow/context.md` if structure changed

### Phase 2: Building

1. Prompt AI with atomic task
2. Guidelines:
   - Ask for patch diff, not prose
   - Keep changes surgical

### Phase 3: Testing

1. Run tests
2. Trim scope if needed

### Phase 4: Commit and Cleanup

1. Commit with git
2. Append to `docs/decisions.md` if decision occurred
3. Update `docs/tasks.md` accordingly
4. Update `docs/workflow/context.md` (keep under 200 lines)

## Prompting Pattern

_Add your preferred prompting patterns here._

## Notes

_Customize this file to match your workflow._
