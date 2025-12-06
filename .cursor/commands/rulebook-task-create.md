---
name: /rulebook-task-create
id: rulebook-task-create
category: Rulebook
description: Create a new Rulebook task following OpenSpec-compatible format with Context7 MCP validation.
---
<!-- RULEBOOK:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `/rulebook/RULEBOOK.md` for complete task management guidelines and format requirements.
- **CRITICAL**: Context7 MCP is REQUIRED for task creation to ensure correct OpenSpec-compatible format.
- Identify any vague or ambiguous details and ask the necessary follow-up questions before editing files.

**Steps**
1. **Check Context7 MCP (MANDATORY)**: Query Context7 for OpenSpec documentation to get the official format:
   ```
   @Context7 /fission-ai/openspec task creation format spec structure
   ```
   Review official format requirements: spec delta format, requirement structure, scenario formatting, delta headers (ADDED/MODIFIED/REMOVED/RENAMED).

2. **Explore Current State**: 
   - Run `rulebook task list` to see existing tasks
   - Review related code or docs (e.g., via `rg`/`ls`) to understand current behavior
   - Note any gaps that require clarification

3. **Choose Task ID**: Use verb-led kebab-case (e.g., `add-feature`, `update-api`, `refactor-module`). Must be unique.

4. **Create Task Structure**:
   ```bash
   rulebook task create <task-id>
   ```
   This creates `/rulebook/tasks/<task-id>/` with:
   - `proposal.md` - Why and what changes
   - `tasks.md` - Implementation checklist
   - `specs/` - Directory for spec deltas

5. **Write Proposal** (`proposal.md`):
   - **Why**: Minimum 20 characters explaining why this change is needed
   - **What Changes**: Detailed description of what will change
   - **Impact**: Affected specs, code, breaking changes, user benefits

6. **Write Tasks Checklist** (`tasks.md`):
   - Organize by phases (Implementation, Testing, Documentation)
   - Use checkbox format: `- [ ] Task description`
   - Include validation steps (tests, coverage checks)

7. **Write Spec Delta** (`specs/<module>/spec.md`):
   - Use `## ADDED|MODIFIED|REMOVED|RENAMED Requirements` headers
   - Requirements MUST use `### Requirement: [Name]` with SHALL/MUST keywords
   - Scenarios MUST use `#### Scenario:` (4 hashtags, NOT 3)
   - Scenarios MUST use Given/When/Then structure
   - Example:
     ```markdown
     ## ADDED Requirements
     
     ### Requirement: Feature Name
     The system SHALL do something specific and testable.
     
     #### Scenario: Scenario Name
     Given some precondition
     When an action occurs
     Then an expected outcome happens
     ```

8. **Validate Task**:
   ```bash
   rulebook task validate <task-id>
   ```
   Fix any validation errors before proceeding.

9. **Verify Format**:
   - Purpose section: ≥20 characters ✅
   - Requirements: Contain SHALL or MUST ✅
   - Scenarios: Use `#### Scenario:` (4 hashtags) ✅
   - Scenarios: Use Given/When/Then structure ✅
   - Delta headers: Use ADDED/MODIFIED/REMOVED/RENAMED ✅

**Reference**
- See `/rulebook/RULEBOOK.md` for complete task management guidelines
- Use `rulebook task show <task-id>` to view task details
- Use `rulebook task list` to see all tasks
- Search existing requirements with `rg -n "Requirement:|Scenario:" rulebook/tasks` before writing new ones
- Explore the codebase with `rg <keyword>`, `ls`, or direct file reads so proposals align with current implementation realities

**Common Pitfalls to Avoid**
- ❌ Using 3 hashtags for scenarios (`### Scenario:`) - MUST use 4 (`#### Scenario:`)
- ❌ Missing SHALL/MUST keywords in requirements
- ❌ Using bullet points for scenarios instead of Given/When/Then
- ❌ Purpose section too short (<20 characters)
- ❌ Wrong delta headers (use ADDED/MODIFIED/REMOVED/RENAMED, not "New Requirements" or "Changes")
<!-- RULEBOOK:END -->

