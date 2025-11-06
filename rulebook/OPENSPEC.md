<!-- OPENSPEC:START -->
# OpenSpec Instructions

**CRITICAL**: Use OpenSpec for spec-driven development of new features and breaking changes.

## When to Use

Create proposal for:
- ✅ New features/capabilities
- ✅ Breaking changes
- ✅ Architecture changes  
- ✅ Performance/security work

Skip for:
- ❌ Bug fixes (restore intended behavior)
- ❌ Typos, formatting, comments
- ❌ Dependency updates (non-breaking)

## CRITICAL: Task Creation Workflow

**MANDATORY STEPS** - Follow in this exact order:

### Step 1: Check Context7 MCP (MANDATORY)

**BEFORE creating ANY OpenSpec task, you MUST:**

1. **Query Context7 for OpenSpec documentation**:
   ```
   @Context7 /fission-ai/openspec task creation format spec structure
   ```

2. **Review official examples**:
   - Spec delta file format
   - Requirement structure
   - Scenario formatting
   - Delta headers (ADDED/MODIFIED/REMOVED/RENAMED)

3. **Verify format requirements**:
   - Scenario MUST use `#### Scenario:` (4 hashtags, NOT 3, NOT bullets)
   - Requirements MUST use `### Requirement: [Name]`
   - MUST include SHALL/MUST statement after requirement name
   - MUST include at least one scenario per requirement

**Why This Matters:**
Most AI assistants create OpenSpec tasks with incorrect formats (wrong scenario headers, missing SHALL statements, incomplete deltas). Context7 provides the official format documentation that prevents validation failures.

### Step 2: Explore Current State

```bash
# List existing specs
openspec spec list --long

# List active changes
openspec list

# Optional: Full-text search
rg -n "Requirement:|^#" openspec/specs
rg -n "^#|Requirement:" openspec/changes
```

### Step 3: Choose Change ID

- Use **verb-led** kebab-case: `add-auth`, `update-api`, `remove-feature`, `refactor-module`
- Must be unique (check existing changes)
- Descriptive and focused (one capability per change)

### Step 4: Scaffold Change Structure

```bash
CHANGE=add-your-feature
mkdir -p openspec/changes/$CHANGE/specs/capability-name

# Create required files:
# - proposal.md (why, what, impact)
# - tasks.md (implementation checklist)
# - specs/capability-name/spec.md (deltas)
# - design.md (optional - only if needed)
```

### Step 5: Write Spec Deltas (CRITICAL FORMAT)

**Directory Structure:**
```
openspec/changes/$CHANGE/
├── proposal.md
├── tasks.md
├── design.md (optional)
└── specs/
    └── [capability-name]/
        └── spec.md  # Delta file with ADDED/MODIFIED/REMOVED/RENAMED
```

**Spec Delta File Format (`specs/[capability]/spec.md`):**

```markdown
## ADDED Requirements

### Requirement: Feature Name
The system SHALL provide [specific functionality/behavior].

#### Scenario: Descriptive scenario name
- **GIVEN** [optional initial state]
- **WHEN** [condition or trigger]
- **THEN** [expected outcome]
- **AND** [additional outcomes or conditions]

#### Scenario: Another scenario name
- **WHEN** [different condition]
- **THEN** [different outcome]

## MODIFIED Requirements

### Requirement: Existing Feature Name
[Complete modified requirement text - include full requirement, not just diff]

#### Scenario: Updated scenario
- **WHEN** [updated condition]
- **THEN** [updated outcome]

## REMOVED Requirements

### Requirement: Deprecated Feature
**Reason**: [Why removing]
**Migration**: [How to handle existing usage]

## RENAMED Requirements

- FROM: `### Requirement: Old Name`
- TO: `### Requirement: New Name`
```

**CRITICAL FORMATTING RULES:**

1. **Scenario Headers** - MUST use exactly 4 hashtags:
   ```markdown
   #### Scenario: Name   ✅ CORRECT
   ### Scenario: Name    ❌ WRONG (3 hashtags)
   - **Scenario: Name**   ❌ WRONG (bullet)
   **Scenario**: Name    ❌ WRONG (bold)
   ```

2. **Keywords** - MUST be bold with proper capitalization:
   ```markdown
   - **GIVEN** [optional]
   - **WHEN** [required]
   - **THEN** [required]
   - **AND** [optional continuation]
   ```

3. **Requirement Format**:
   ```markdown
   ### Requirement: Descriptive Name (under 50 chars)
   [SHALL/MUST statement describing core behavior]
   
   #### Scenario: [at least one scenario required]
   ```

4. **Delta Headers** - MUST match exactly (whitespace ignored):
   ```markdown
   ## ADDED Requirements
   ## MODIFIED Requirements
   ## REMOVED Requirements
   ## RENAMED Requirements
   ```

### Step 6: Write Proposal (`proposal.md`)

```markdown
## Why
[1-2 sentences on problem/opportunity]

## What Changes
- [Bullet list of changes]
- [Mark breaking changes with **BREAKING**]

## Impact
- Affected specs: [list capabilities]
- Affected code: [key files/systems]
- Breaking change: [yes/no]
```

### Step 7: Write Tasks (`tasks.md`)

```markdown
## 1. Implementation
- [ ] 1.1 Task description
- [ ] 1.2 Another task

## 2. Testing
- [ ] 2.1 Unit tests
- [ ] 2.2 Integration tests

## 3. Documentation
- [ ] 3.1 Update README
```

### Step 8: Validate (MANDATORY)

```bash
# ALWAYS use --strict flag for comprehensive validation
openspec validate $CHANGE --strict

# Debug parsing if validation fails:
openspec change show $CHANGE --json --deltas-only
```

**If validation fails:**
- Check scenario format (must be `#### Scenario:`)
- Verify each requirement has at least one scenario
- Ensure delta headers match exactly
- Check for SHALL/MUST statements in requirements
- Review JSON output to see what parser found

### Step 9: Get Approval

**DO NOT start implementation until proposal is approved.**

## Three-Stage Workflow

### Stage 1: Create
1. ✅ Check Context7 MCP for format
2. ✅ Explore current state
3. ✅ Choose verb-led `change-id`
4. ✅ Scaffold structure
5. ✅ Write spec deltas (following format exactly)
6. ✅ Write proposal.md and tasks.md
7. ✅ Validate with `--strict`
8. ✅ Get approval

### Stage 2: Implement  
1. Read `proposal.md`, `tasks.md`
2. Implement tasks
3. Run AGENT_AUTOMATION workflow
4. Update tasks as complete (mark `[x]`)
5. Document commit hash in tasks.md

### Stage 3: Archive
After deployment:
```bash
openspec archive [change] --yes
```

## Commands Reference

```bash
# List and explore
openspec spec list --long        # All capabilities
openspec list                     # Active changes
openspec show [item]             # View details
openspec show [item] --json      # JSON output

# Validation
openspec validate [change] --strict  # ALWAYS use --strict
openspec change show [id] --json --deltas-only  # Debug parsing

# Archive
openspec archive [change] --yes
```

## Common Mistakes to Avoid

❌ **WRONG Format Examples:**

```markdown
# Missing scenario header
### Requirement: Login
- **WHEN** user logs in
- **THEN** token returned

# Wrong scenario header
### Scenario: Login
- **WHEN** user logs in

# Using bullet for scenario
- **Scenario: Login**
  - **WHEN** user logs in

# Missing SHALL/MUST statement
### Requirement: Login
#### Scenario: Success
- **WHEN** user logs in
```

✅ **CORRECT Format:**

```markdown
### Requirement: User Authentication
The system SHALL authenticate users with valid credentials.

#### Scenario: Successful login
- **WHEN** user provides valid email and password
- **THEN** system returns JWT token
- **AND** token expires in 24 hours

#### Scenario: Invalid credentials
- **WHEN** user provides incorrect password
- **THEN** system returns 401 error
- **AND** error message does not reveal if email exists
```

## Best Practices

✅ **DO:**
- **ALWAYS check Context7 MCP first** (`@Context7 /fission-ai/openspec`)
- Use exactly `#### Scenario:` (4 hashtags)
- Include SHALL/MUST statement after requirement name
- Write at least one scenario per requirement
- Use GIVEN/WHEN/THEN/AND keywords in bold
- Validate with `--strict` before committing
- Keep changes focused (one capability per change)
- Use verb-led change IDs

❌ **DON'T:**
- Create tasks without checking Context7 MCP first
- Use 3 hashtags (`### Scenario:`) or bullets for scenarios
- Skip scenario definitions
- Mix multiple features in one change
- Start implementation before approval
- Use partial diffs in MODIFIED requirements (include full requirement)

## Integration with AGENT_AUTOMATION

OpenSpec drives implementation. AGENT_AUTOMATION enforces quality:

```
1. Create spec → Validate → Approve
2. Implement → Run AGENT_AUTOMATION
3. Update tasks.md with commit hash
4. Archive when deployed
```

## Complete Example

**Change ID:** `add-user-authentication`

**File: `openspec/changes/add-user-authentication/specs/auth/spec.md`**

```markdown
## ADDED Requirements

### Requirement: User Login
The system SHALL authenticate users with email and password credentials.

#### Scenario: Successful login
- **WHEN** user provides valid email and password
- **THEN** system returns JWT token
- **AND** token expires in 24 hours

#### Scenario: Invalid credentials
- **WHEN** user provides incorrect password
- **THEN** system returns 401 error
- **AND** error message does not reveal if email exists

### Requirement: Password Security
The system SHALL hash passwords using bcrypt with minimum 10 rounds.

#### Scenario: Password storage
- **WHEN** user creates account
- **THEN** password is hashed before storage
- **AND** plaintext password is never stored
```

**Validation:**
```bash
openspec validate add-user-authentication --strict
```

<!-- OPENSPEC:END -->