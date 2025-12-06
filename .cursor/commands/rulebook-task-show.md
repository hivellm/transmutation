---
name: /rulebook-task-show
id: rulebook-task-show
category: Rulebook
description: Show detailed information about a specific Rulebook task.
---
<!-- RULEBOOK:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `/rulebook/RULEBOOK.md` for complete task management guidelines.

**Steps**
1. **Show Task Details**:
   ```bash
   rulebook task show <task-id>
   ```
   Displays:
   - Task ID and title
   - Status (pending, in-progress, completed, blocked)
   - Created and updated dates
   - Archive date (if archived)
   - Proposal summary
   - Spec files list

2. **Review Proposal**:
   - Read `proposal.md` to understand why and what changes
   - Check impact assessment
   - Verify breaking changes are documented

3. **Review Tasks Checklist**:
   - Check `tasks.md` for implementation checklist
   - Identify completed items (`[x]`)
   - Identify pending items (`[ ]`)
   - Follow priority order (most critical first)

4. **Review Spec Deltas**:
   - Check `specs/*/spec.md` files
   - Verify format compliance (SHALL/MUST, 4 hashtags for scenarios)
   - Understand requirements and scenarios

5. **Check Task Status**:
   - Verify task is not blocked
   - Check if task is ready for implementation
   - Confirm all prerequisites are met

**Reference**
- Use `rulebook task list` to see all tasks
- Use `rulebook task validate <task-id>` to check task format
- See `/rulebook/RULEBOOK.md` for complete task management guidelines
<!-- RULEBOOK:END -->

