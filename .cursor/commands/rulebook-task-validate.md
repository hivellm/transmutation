---
name: /rulebook-task-validate
id: rulebook-task-validate
category: Rulebook
description: Validate Rulebook task format against OpenSpec-compatible requirements.
---
<!-- RULEBOOK:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `/rulebook/RULEBOOK.md` for complete task management guidelines and format requirements.

**Steps**
1. **Validate Task Format**:
   ```bash
   rulebook task validate <task-id>
   ```
   Checks:
   - Purpose section length (≥20 characters)
   - Requirement keywords (SHALL/MUST)
   - Scenario format (4 hashtags, not 3)
   - Given/When/Then structure
   - Delta headers format (ADDED/MODIFIED/REMOVED/RENAMED)

2. **Review Validation Results**:
   - **Errors**: Must be fixed before proceeding
   - **Warnings**: Should be addressed but not blocking

3. **Fix Format Issues**:
   - **Purpose too short**: Expand "Why" section to ≥20 characters
   - **Missing SHALL/MUST**: Add SHALL or MUST keyword to requirements
   - **Wrong scenario format**: Change `### Scenario:` to `#### Scenario:` (4 hashtags)
   - **Missing Given/When/Then**: Replace bullet points with Given/When/Then structure
   - **Wrong delta headers**: Use ADDED/MODIFIED/REMOVED/RENAMED, not "New Requirements"

4. **Re-validate After Fixes**:
   ```bash
   rulebook task validate <task-id>
   ```
   Ensure all errors are resolved.

5. **Common Format Errors**:
   - ❌ `### Scenario:` (3 hashtags) → ✅ `#### Scenario:` (4 hashtags)
   - ❌ "The system provides X" → ✅ "The system SHALL provide X"
   - ❌ `- WHEN user does X THEN Y` → ✅ `Given X\nWhen Y\nThen Z`
   - ❌ `## New Requirements` → ✅ `## ADDED Requirements`

**Reference**
- See `/rulebook/RULEBOOK.md` for complete format requirements
- Use Context7 MCP to get official OpenSpec format documentation
- Use `rulebook task show <task-id>` to view task details
<!-- RULEBOOK:END -->

