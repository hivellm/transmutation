---
name: /rulebook-task-list
id: rulebook-task-list
category: Rulebook
description: List all Rulebook tasks (active and optionally archived).
---
<!-- RULEBOOK:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `/rulebook/RULEBOOK.md` for complete task management guidelines.

**Steps**
1. **List Active Tasks**:
   ```bash
   rulebook task list
   ```
   Shows all active tasks with their status (pending, in-progress, completed, blocked).

2. **List Including Archived**:
   ```bash
   rulebook task list --archived
   ```
   Shows both active and archived tasks.

3. **Review Task Status**:
   - **pending**: Task not started
   - **in-progress**: Task being worked on
   - **completed**: Task finished (ready for archive)
   - **blocked**: Task blocked by dependency

4. **Select Task to Work On**:
   - Choose task with highest priority
   - Check task status before starting
   - Verify task is not blocked

**Reference**
- Use `rulebook task show <task-id>` to view task details
- Use `rulebook task validate <task-id>` to check task format
- See `/rulebook/RULEBOOK.md` for complete task management guidelines
<!-- RULEBOOK:END -->

