---
name: /rulebook-task-archive
id: rulebook-task-archive
category: Rulebook
description: Archive a completed Rulebook task and apply spec deltas to main specifications.
---
<!-- RULEBOOK:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `/rulebook/RULEBOOK.md` for complete task management guidelines.

**Steps**
1. **Verify Task Completion**:
   - All items in `tasks.md` must be marked as `[x]`
   - All tests must pass
   - Code review complete (if applicable)
   - Documentation updated (README, CHANGELOG, specs)

2. **Run Quality Checks**:
   ```bash
   npm test
   npm run lint
   npm run type-check
   npm run build
   ```
   Ensure all checks pass before archiving.

3. **Validate Task Format**:
   ```bash
   rulebook task validate <task-id>
   ```
   Must pass all format checks.

4. **Archive Task**:
   ```bash
   rulebook task archive <task-id>
   ```
   Or without prompts:
   ```bash
   rulebook task archive <task-id> --skip-validation
   ```
   (Only use `--skip-validation` if you're certain the task is valid)

5. **Archive Process**:
   - Validates task format (unless skipped)
   - Checks task completion status
   - Applies spec deltas to main specifications
   - Moves task to `/rulebook/tasks/archive/YYYY-MM-DD-<task-id>/`
   - Updates related specifications

6. **Verify Archive**:
   ```bash
   rulebook task list --archived
   ```
   Task should appear in archived list.

7. **Post-Archive Actions**:
   - Ensure spec deltas are applied to main specifications
   - Update CHANGELOG.md with the change
   - Document any breaking changes
   - Create migration guides (if needed)
   - Unblock related tasks (if any)

**Reference**
- Use `rulebook task list --archived` to see archived tasks
- Use `rulebook task show <task-id>` to view task details
- See `/rulebook/RULEBOOK.md` for complete task management guidelines
<!-- RULEBOOK:END -->

