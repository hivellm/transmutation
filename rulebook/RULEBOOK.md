<!-- RULEBOOK:START -->
# Rulebook Task Management

**CRITICAL**: Use Rulebook's built-in task management system for spec-driven development of new features and breaking changes.

## When to Use

Create tasks for:
- ✅ New features/capabilities
- ✅ Breaking changes
- ✅ Architecture changes  
- ✅ Performance/security work

Skip for:
- ❌ Bug fixes (restore intended behavior)
- ❌ Typos, formatting, comments
- ❌ Dependency updates (non-breaking)

## ⚠️ CRITICAL: Task Creation is MANDATORY Before Implementation

**ABSOLUTE RULE**: You MUST create a task BEFORE implementing ANY feature.

### Why This Matters

**Without task registration:**
- ❌ Tasks can be lost in context
- ❌ No tracking of implementation progress
- ❌ No record of what was done and why
- ❌ Difficult to resume work after context loss
- ❌ No validation of completion criteria

**With task registration:**
- ✅ All features are tracked and documented
- ✅ Progress is visible and measurable
- ✅ Implementation history is preserved
- ✅ Easy to resume work from any point
- ✅ Clear completion criteria

### MANDATORY Workflow

**NEVER start implementation without creating a task first:**

```bash
# ❌ WRONG: Starting implementation directly
# ... writing code without task ...

# ✅ CORRECT: Create task first
rulebook task create <task-id>
# Write proposal.md
# Write tasks.md
# Write spec deltas
rulebook task validate <task-id>
# NOW you can start implementation
```

### Task Creation Before Any Feature Request

**When a feature is requested:**

1. **STOP** - Do not start coding
2. **Create task** - `rulebook task create <task-id>`
3. **Plan** - Write proposal.md and tasks.md
4. **Spec** - Write spec deltas
5. **Validate** - `rulebook task validate <task-id>`
6. **THEN** - Start implementation

**Example:**
```
User: "Add user authentication feature"

❌ WRONG: Start coding immediately
✅ CORRECT:
  1. rulebook task create add-user-authentication
  2. Write proposal.md explaining why and what
  3. Write tasks.md with implementation checklist
  4. Write specs/core/spec.md with requirements
  5. rulebook task validate add-user-authentication
  6. NOW start implementing
```

## CRITICAL: Task Creation Workflow

**MANDATORY STEPS** - Follow in this exact order:

### Step 1: Check Context7 MCP (MANDATORY)

**BEFORE creating ANY task, you MUST:**

1. **Query Context7 for OpenSpec documentation** (Rulebook uses OpenSpec-compatible format):
   ```
   @Context7 /fission-ai/openspec task creation format spec structure
   ```

2. **Review official format requirements**:
   - Spec delta file format
   - Requirement structure
   - Scenario formatting
   - Delta headers (ADDED/MODIFIED/REMOVED/RENAMED)

3. **Verify format requirements**:
   - Scenario MUST use `#### Scenario:` (4 hashtags, NOT 3, NOT bullets)
   - Requirements MUST use `### Requirement: [Name]`
   - MUST include SHALL/MUST statement after requirement name
   - MUST include at least one scenario per requirement
   - Purpose section MUST have minimum 20 characters

**Why This Matters:**
Most AI assistants create tasks with incorrect formats (wrong scenario headers, missing SHALL statements, incomplete deltas). Context7 provides the official format documentation that prevents validation failures.

### Step 2: Explore Current State

```bash
# List existing tasks
rulebook task list

# List active changes
rulebook task list --active

# View task details
rulebook task show <task-id>
```

### Step 3: Choose Task ID

- Use **verb-led** kebab-case: `add-auth`, `update-api`, `remove-feature`, `refactor-module`
- Must be unique (check existing tasks)
- Descriptive and focused (one capability per task)

### Step 4: Create Task Structure

```bash
# Create new task
rulebook task create <task-id>

# This creates:
# /rulebook/tasks/<task-id>/
#   ├── proposal.md       # Why and what changes
#   ├── tasks.md          # Implementation checklist
#   ├── design.md         # Technical decisions (optional)
#   └── specs/
#       └── <module>/
#           └── spec.md   # Delta showing additions/modifications
```

### Step 5: Write Proposal

**File**: `/rulebook/tasks/<task-id>/proposal.md`

```markdown
# Proposal: Task Name

## Why
Minimum 20 characters explaining why this change is needed.
Provide context, motivation, and business/technical rationale.

## What Changes
Detailed description of what will change:
- Specific components affected
- New features or capabilities
- Breaking changes (if any)
- Migration path (if applicable)

## Impact
- Affected specs: list spec names
- Affected code: list files/modules
- Breaking change: YES/NO
- User benefit: describe benefits
```

### Step 6: Write Tasks Checklist

**File**: `/rulebook/tasks/<task-id>/tasks.md`

```markdown
## 1. Implementation Phase
- [ ] 1.1 First task item
- [ ] 1.2 Second task item

## 2. Testing Phase
- [ ] 2.1 Write unit tests
- [ ] 2.2 Write integration tests

## 3. Documentation Phase
- [ ] 3.1 Update README
- [ ] 3.2 Update CHANGELOG
```

### Step 7: Write Spec Delta

**File**: `/rulebook/tasks/<task-id>/specs/<module>/spec.md`

**CRITICAL FORMAT REQUIREMENTS:**

```markdown
# Specification Name

## ADDED Requirements

### Requirement: Feature Name
The system SHALL/MUST do something specific and testable.
Every requirement needs SHALL or MUST keyword.

#### Scenario: Scenario Name
Given some precondition
When an action occurs
Then an expected outcome happens

## MODIFIED Requirements

### Requirement: Existing Feature
The system SHALL/MUST do something modified.

#### Scenario: Modified scenario
Given updated precondition
When action occurs
Then new expected outcome

## REMOVED Requirements

### Requirement: Deprecated Feature
[Description of what is being removed]

## RENAMED Requirements
- FROM: `### Requirement: Old Name`
- TO: `### Requirement: New Name`
```

**Format Rules:**
- ✅ Purpose section: Minimum 20 characters
- ✅ Requirements: Must contain SHALL or MUST
- ✅ Scenarios: Use `#### Scenario:` (4 hashtags)
- ✅ Scenarios: Use Given/When/Then structure
- ✅ Deltas: Use ADDED/MODIFIED/REMOVED/RENAMED headers
- ❌ NEVER use 3 hashtags for scenarios
- ❌ NEVER use bullet points for scenarios
- ❌ NEVER omit SHALL/MUST from requirements

### Step 8: Validate Task

```bash
# Validate task format
rulebook task validate <task-id>

# Validate with strict mode (recommended)
rulebook task validate <task-id> --strict

# Validate all tasks
rulebook task validate --all
```

**Validation checks:**
- Purpose section length (≥20 chars)
- Requirement keywords (SHALL/MUST)
- Scenario format (4 hashtags)
- Given/When/Then structure
- Delta headers format

### Step 9: Update Task Status

```bash
# Mark task as in progress
rulebook task update <task-id> --status in-progress

# Update task progress
rulebook task update <task-id> --progress 50

# Mark task as completed
rulebook task update <task-id> --status completed
```

### Step 10: Archive Completed Task

```bash
# Archive completed task
rulebook task archive <task-id>

# Archive without prompts
rulebook task archive <task-id> --yes
```

**Archive process:**
1. Validates task format
2. Checks task completion status
3. Applies spec deltas to main specifications
4. Moves task to `/rulebook/tasks/archive/YYYY-MM-DD-<task-id>/`
5. Updates related specifications

## Task Format Examples

### Correct Format ✅

```markdown
# Auth Specification

## ADDED Requirements

### Requirement: Two-Factor Authentication
The system MUST require a second factor during login for enhanced security.

#### Scenario: OTP required
Given a user submits valid credentials
When authentication starts
Then an OTP challenge is required

#### Scenario: OTP verification
Given a user receives an OTP code
When they submit the correct OTP
Then they are authenticated successfully
```

### Incorrect Format ❌

```markdown
# Auth Specification

## Requirements

### Requirement: Two-Factor Authentication
The system requires a second factor.  # ❌ Missing SHALL/MUST

#### Scenario: OTP required  # ❌ Only 3 hashtags
- WHEN user submits credentials  # ❌ Using bullets instead of Given/When/Then
- THEN OTP challenge is required
```

## Common Pitfalls & How to Avoid Them

### Top 5 Mistakes AI Assistants Make

1. **Wrong Scenario Headers**
   - ❌ `### Scenario:` (3 hashtags)
   - ✅ `#### Scenario:` (4 hashtags)

2. **Missing SHALL/MUST Keywords**
   - ❌ "The system provides authentication"
   - ✅ "The system SHALL provide authentication"

3. **Using Bullets for Scenarios**
   - ❌ `- WHEN user does X THEN Y happens`
   - ✅ `Given X\nWhen Y\nThen Z`

4. **Incomplete Purpose Section**
   - ❌ "Auth system" (too short)
   - ✅ "Authentication system for secure user access with JWT tokens and session management" (≥20 chars)

5. **Wrong Delta Headers**
   - ❌ `## New Requirements` or `## Changes`
   - ✅ `## ADDED Requirements`, `## MODIFIED Requirements`, etc.

## Integration with AGENT_AUTOMATION

**CRITICAL**: After implementing a task, follow AGENT_AUTOMATION workflow:

1. Run quality checks (lint, test, type-check, build)
2. Update task status in `tasks.md`
3. Update documentation (ROADMAP, CHANGELOG, specs)
4. Commit with conventional commit format
5. Archive task when complete

## ⚠️ CRITICAL: Git Hooks Will Block Commits with Problems

**ABSOLUTE RULE**: Pre-commit and pre-push hooks will **BLOCK** any commit attempt if there are:
- ❌ Lint errors or warnings
- ❌ Test failures
- ❌ Type check errors
- ❌ Formatting issues
- ❌ Coverage below thresholds

### Why This Matters

**DO NOT attempt to commit code with problems:**
- ❌ `git commit` will **FAIL** if lint has errors
- ❌ `git commit` will **FAIL** if tests are failing
- ❌ `git push` will **FAIL** if pre-push checks fail
- ❌ You will waste time trying to commit broken code
- ❌ The hooks will reject your commit automatically

**ALWAYS fix problems BEFORE attempting to commit:**
- ✅ Run `npm run lint` and fix ALL errors/warnings first
- ✅ Run `npm test` and ensure ALL tests pass
- ✅ Run `npm run type-check` and fix ALL type errors
- ✅ Run `npm run format` if formatting is required
- ✅ Run `npm test -- --coverage` and ensure coverage thresholds are met
- ✅ **ONLY THEN** attempt `git commit`

### Mandatory Pre-Commit Workflow

**BEFORE every commit, you MUST:**

```bash
# 1. Fix lint errors FIRST (highest priority)
npm run lint
# Fix ALL errors and warnings
# If lint fails, commit will be blocked

# 2. Fix test failures SECOND
npm test
# Fix ALL failing tests
# If tests fail, commit will be blocked

# 3. Fix type errors THIRD
npm run type-check
# Fix ALL type errors
# If type check fails, commit will be blocked

# 4. Fix formatting (if required)
npm run format
# Apply formatting fixes

# 5. Verify coverage (if required by hooks)
npm test -- --coverage
# Ensure coverage thresholds are met

# 6. ONLY AFTER all checks pass, attempt commit
git add .
git commit -m "feat: your commit message"
# This will now succeed because all checks passed
```

### What Happens If You Try to Commit with Problems

**Example of blocked commit:**

```bash
$ git commit -m "feat: add new feature"

🔍 Running TypeScript/JavaScript pre-commit checks...
  → Type checking...
  → Linting...

/mnt/f/project/src/feature.ts
   42:19  error  Unexpected any. Specify a different type  @typescript-eslint/no-explicit-any

✖ 1 problem (1 error, 0 warnings)

❌ Commit blocked: Lint errors found
```

**You MUST fix the error before committing:**

```bash
# Fix the lint error
# ... edit code to fix the issue ...

# Run lint again to verify
npm run lint
# ✅ All checks pass

# NOW commit will succeed
git commit -m "feat: add new feature"
# ✅ Commit successful
```

### ⚠️ CRITICAL: NEVER Use --no-verify to Bypass Hooks

**ABSOLUTE PROHIBITION**: You MUST NEVER use `--no-verify` or `--no-gpg-sign` flags to bypass git hooks.

**FORBIDDEN COMMANDS:**
- ❌ `git commit --no-verify` - **NEVER USE THIS**
- ❌ `git commit -n` - **NEVER USE THIS** (short form of --no-verify)
- ❌ `git push --no-verify` - **NEVER USE THIS**
- ❌ Any flag that skips pre-commit or pre-push hooks

### Why This Is Prohibited

**Using `--no-verify` defeats the entire purpose of quality gates:**
- ❌ Allows broken code to be committed
- ❌ Bypasses all quality checks (lint, test, type-check)
- ❌ Introduces technical debt and bugs
- ❌ Violates project quality standards
- ❌ Can break the build for other developers
- ❌ Makes code review harder (reviewers see broken code)

**The hooks exist for a reason:**
- ✅ They protect code quality
- ✅ They prevent bugs from entering the codebase
- ✅ They ensure consistency across the project
- ✅ They catch errors before they reach production

### What to Do Instead

**If you're tempted to use `--no-verify`, it means:**
1. **You have problems that need fixing** - Fix them first
2. **You're trying to commit too early** - Complete the work properly
3. **You're rushing** - Slow down and do it right

**Correct approach:**

```bash
# ❌ WRONG: Trying to bypass hooks
git commit --no-verify -m "feat: add feature"
# This is FORBIDDEN - never do this

# ✅ CORRECT: Fix problems first, then commit
npm run lint
# Fix all errors...

npm test
# Fix all failing tests...

npm run type-check
# Fix all type errors...

# NOW commit (hooks will pass)
git commit -m "feat: add feature"
# ✅ Commit successful - all checks passed
```

### Summary

**CRITICAL RULES:**
- ⚠️ **NEVER** attempt to commit code with lint errors - hooks will block it
- ⚠️ **NEVER** attempt to commit code with test failures - hooks will block it
- ⚠️ **NEVER** attempt to commit code with type errors - hooks will block it
- ⚠️ **NEVER** use `--no-verify` or any flag to bypass hooks - **ABSOLUTELY FORBIDDEN**
- ⚠️ **ALWAYS** fix ALL problems BEFORE attempting to commit
- ⚠️ **ALWAYS** run quality checks manually before `git commit`
- ⚠️ **ALWAYS** ensure all checks pass before committing

**The hooks are there to protect code quality - they will NOT let broken code through. Always resolve problems first, then commit. Bypassing hooks is strictly prohibited and defeats the purpose of quality gates.**

## MANDATORY: Task List Updates During Implementation

**CRITICAL RULE**: You MUST update the task list (`tasks.md`) immediately after completing and testing each implementation step.

### When to Update Task List

**ALWAYS update `tasks.md` when:**
- ✅ You complete a task item (mark as `[x]`)
- ✅ You finish implementing a feature and it's tested
- ✅ You complete a test suite
- ✅ You finish documentation updates
- ✅ You verify the implementation works correctly

**NEVER commit without updating `tasks.md` if you've made progress on a task.**

### How to Update Task List

1. **After Implementation**:
   ```markdown
   ## 1. Implementation Phase
   - [x] 1.1 Create task manager module  # ✅ Mark as done
   - [x] 1.2 Add validation logic        # ✅ Mark as done
   - [ ] 1.3 Add archive functionality   # Still pending
   ```

2. **After Testing**:
   ```markdown
   ## 2. Testing Phase
   - [x] 2.1 Write unit tests            # ✅ Tests written and passing
   - [x] 2.2 Write integration tests     # ✅ Tests written and passing
   - [ ] 2.3 Add E2E tests                # Still pending
   ```

3. **After Documentation**:
   ```markdown
   ## 3. Documentation Phase
   - [x] 3.1 Update README                # ✅ Updated
   - [x] 3.2 Update CHANGELOG             # ✅ Updated
   - [ ] 3.3 Update API docs              # Still pending
   ```

### Workflow: Implement → Test → Verify Coverage → Update Tasks → Commit → Next Task

**MANDATORY SEQUENCE** for every implementation:

```bash
# 1. Implement the feature
# ... write code ...

# 2. Test the implementation
npm test
npm run lint
npm run type-check

# 3. Verify test coverage (CRITICAL)
npm test -- --coverage
# Check coverage thresholds are met
# Fix any coverage gaps before proceeding

# 4. Update tasks.md IMMEDIATELY after successful tests and coverage check
# Mark completed items as [x] in tasks.md
# Update task status if needed

# 5. Verify task status is updated before moving to next task
rulebook task show <task-id>
# Confirm status reflects current progress

# 6. Commit locally (BACKUP - do this frequently)
git add .
git commit -m "feat: implement task manager validation

- Complete task 1.2: Add validation logic
- All tests passing
- Coverage verified: 95%
- Updated tasks.md"

# 7. Keep remote repository updated (if configured)
# Check if remote is configured
git remote -v

# If remote exists, push regularly
git push origin <branch-name>

# If no remote configured, see setup instructions below

# 8. Only then proceed to next task
# Follow priority order (most critical first)
```

## ⚠️ CRITICAL: Frequent Local Commits for Backup

**ABSOLUTE RULE**: Commit locally frequently, even if just for backup purposes.

### Why Frequent Commits Matter

**Without frequent commits:**
- ❌ Risk of losing work if system crashes
- ❌ No recovery point if something goes wrong
- ❌ Difficult to revert to previous working state
- ❌ Lost context if session is interrupted

**With frequent commits:**
- ✅ Work is backed up locally
- ✅ Easy to recover from mistakes
- ✅ Can revert to any previous state
- ✅ Progress is preserved

### When to Commit Locally

**Commit locally whenever you:**
- ✅ Complete a task item (even if not fully tested)
- ✅ Finish implementing a feature (before full testing)
- ✅ Fix a bug or issue
- ✅ Update documentation
- ✅ Make significant progress
- ✅ Feel the need for a backup point
- ✅ Are about to try something risky
- ✅ Are switching to a different task

**Commit frequency:**
- **Minimum**: After completing each task item
- **Recommended**: Every 15-30 minutes of active work
- **Maximum**: As often as you feel necessary for safety

### Local Commit Workflow

```bash
# Quick local commit (backup)
git add .
git commit -m "wip: progress on task 1.2

- Implemented validation logic
- Still testing
- Backup commit"

# Or more descriptive
git add .
git commit -m "feat: add validation logic (WIP)

- Task 1.2 in progress
- Core validation implemented
- Tests pending
- Backup before continuing"
```

### Commit Message Format for Backup Commits

**For work-in-progress commits:**
```bash
git commit -m "wip: <brief description>

- What was done
- Current status
- Next steps"
```

**For completed task items:**
```bash
git commit -m "feat: <feature description>

- Complete task X.Y: <task name>
- All tests passing
- Coverage verified
- Updated tasks.md"
```

## ⚠️ CRITICAL: Keep Remote Repository Updated

**MANDATORY**: Keep your remote repository synchronized with local work.

### Check Remote Configuration

**First, check if remote is configured:**
```bash
git remote -v
```

**If you see output like:**
```
origin  https://github.com/user/repo.git (fetch)
origin  https://github.com/user/repo.git (push)
```
✅ Remote is configured - proceed to push regularly

**If you see no output or error:**
❌ No remote configured - see setup instructions below

### Push to Remote Regularly

**After local commits, push to remote:**
```bash
# Push current branch
git push origin <branch-name>

# Or push current branch (if tracking is set)
git push

# Push with tags
git push --tags
```

**Recommended push frequency:**
- **Minimum**: After completing a task
- **Recommended**: After every 2-3 local commits
- **Maximum**: After every local commit (if working solo)

### Remote Repository Setup

**If no remote repository is configured:**

#### Option 1: GitHub (Recommended)

1. **Create repository on GitHub:**
   - Go to https://github.com/new
   - Create a new repository
   - **DO NOT** initialize with README, .gitignore, or license (if you already have local repo)

2. **Add remote and push:**
   ```bash
   # Add remote (replace with your repository URL)
   git remote add origin https://github.com/username/repo-name.git
   
   # Or using SSH
   git remote add origin git@github.com:username/repo-name.git
   
   # Push to remote
   git push -u origin main
   # Or 'master' if that's your default branch
   ```

3. **Verify:**
   ```bash
   git remote -v
   git push
   ```

**GitHub Setup Guide:**
- **Official Guide**: https://docs.github.com/en/get-started/quickstart/create-a-repo
- **Adding Remote**: https://docs.github.com/en/get-started/getting-started-with-git/managing-remote-repositories

#### Option 2: GitLab

1. **Create repository on GitLab:**
   - Go to https://gitlab.com/projects/new
   - Create a new project
   - **DO NOT** initialize with README (if you already have local repo)

2. **Add remote and push:**
   ```bash
   git remote add origin https://gitlab.com/username/repo-name.git
   git push -u origin main
   ```

**GitLab Setup Guide:**
- **Official Guide**: https://docs.gitlab.com/ee/gitlab-basics/create-project.html

#### Option 3: Bitbucket

1. **Create repository on Bitbucket:**
   - Go to https://bitbucket.org/repo/create
   - Create a new repository

2. **Add remote and push:**
   ```bash
   git remote add origin https://bitbucket.org/username/repo-name.git
   git push -u origin main
   ```

**Bitbucket Setup Guide:**
- **Official Guide**: https://support.atlassian.com/bitbucket-cloud/docs/create-a-git-repository/

#### Option 4: Self-Hosted Git Server

**If using self-hosted Git server:**
```bash
# Add remote
git remote add origin <your-git-server-url>

# Push
git push -u origin main
```

### Verify Remote is Working

**After setting up remote:**
```bash
# Check remote configuration
git remote -v

# Test push
git push origin main

# If successful, you'll see:
# "Enumerating objects: X, done."
# "Writing objects: 100% (X/X), done."
```

### Troubleshooting Remote Issues

**Error: "remote origin already exists"**
```bash
# Remove existing remote
git remote remove origin

# Add new remote
git remote add origin <new-url>
```

**Error: "authentication failed"**
- Check your credentials
- Use SSH keys for better security
- See: https://docs.github.com/en/authentication/connecting-to-github-with-ssh

**Error: "repository not found"**
- Verify repository URL is correct
- Check you have access to the repository
- Ensure repository exists on remote server

### Best Practices for Remote Sync

**DO's ✅:**
- ✅ Push to remote after completing tasks
- ✅ Push before switching branches
- ✅ Push before trying risky changes
- ✅ Push at end of work session
- ✅ Use descriptive commit messages
- ✅ Keep commits atomic (one logical change per commit)

**DON'Ts ❌:**
- ❌ Don't push broken code (test first)
- ❌ Don't push sensitive information (API keys, passwords)
- ❌ Don't force push to shared branches
- ❌ Don't skip pushing for extended periods
- ❌ Don't commit without meaningful messages

### Automated Backup Reminder

**Set up reminders to push regularly:**
```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
alias git-backup='git add . && git commit -m "backup: $(date +%Y-%m-%d\ %H:%M:%S)" && git push'

# Use: git-backup (quick backup and push)
```

### Summary: Backup and Remote Sync Workflow

**Complete workflow:**
1. **Work locally** - Make changes
2. **Test changes** - Ensure they work
3. **Commit locally** - `git commit` (backup)
4. **Update tasks.md** - Mark progress
5. **Push to remote** - `git push` (if remote configured)
6. **Continue work** - Next task

**If no remote:**
1. **Set up remote** - Follow instructions above
2. **Push initial code** - `git push -u origin main`
3. **Continue regular pushes** - After each commit or task

### Priority Order: Most Critical First

**ALWAYS follow this priority order when continuing implementation:**

1. **Tests** (HIGHEST PRIORITY)
   - Write tests for the feature
   - Ensure all tests pass
   - Verify test coverage meets thresholds

2. **Coverage Verification** (CRITICAL)
   - Run coverage check: `npm test -- --coverage`
   - Fix any coverage gaps
   - Ensure coverage thresholds are met

3. **Update Task Status** (MANDATORY)
   - Mark completed items as `[x]` in `tasks.md`
   - Update task status if needed
   - Document what was completed

4. **Next Task** (Only after above steps)
   - Move to next most critical task
   - Follow same sequence

**Example Priority Order:**

```markdown
## Priority Order (Most Critical First)

### 1. Testing (CRITICAL - Do First)
- [ ] 1.1 Write unit tests for core functionality
- [ ] 1.2 Write integration tests
- [ ] 1.3 Verify test coverage ≥ 95%

### 2. Coverage Verification (CRITICAL - Do Second)
- [ ] 2.1 Run coverage check
- [ ] 2.2 Fix coverage gaps
- [ ] 2.3 Verify thresholds met

### 3. Task Status Update (MANDATORY - Do Third)
- [ ] 3.1 Update tasks.md with completed items
- [ ] 3.2 Update task status
- [ ] 3.3 Document completion

### 4. Next Implementation (Only After Above)
- [ ] 4.1 Move to next critical task
- [ ] 4.2 Follow same sequence
```

### Never Skip Steps

**CRITICAL RULES:**
- ❌ NEVER proceed to next task without updating current task status
- ❌ NEVER skip test coverage verification
- ❌ NEVER mark tasks complete without tests passing
- ❌ NEVER implement without creating task first
- ✅ ALWAYS update task status before moving to next task
- ✅ ALWAYS verify coverage before marking task complete
- ✅ ALWAYS follow priority order (most critical first)

### Task Status Tracking

**Track progress in `tasks.md`:**

```markdown
## Progress Summary
- Total tasks: 15
- Completed: 8
- In progress: 2
- Pending: 5
- Blocked: 0

## Current Status
- ✅ Implementation Phase: 80% complete (4/5 tasks)
- ⏳ Testing Phase: 50% complete (2/4 tasks)
- ⏸️ Documentation Phase: 0% complete (0/3 tasks)
```

### Validation Before Committing

**BEFORE every commit, verify:**
- [ ] All completed tasks are marked as `[x]` in `tasks.md`
- [ ] Task status reflects current progress
- [ ] No tasks are marked complete without implementation
- [ ] All tests pass for completed tasks
- [ ] Test coverage meets thresholds (run `npm test -- --coverage`)
- [ ] Task status updated before moving to next task
- [ ] Documentation is updated for completed features

### Task Status Update Before Next Task

**CRITICAL RULE**: You MUST update task status in `tasks.md` BEFORE moving to the next task.

**Why:**
- Prevents loss of progress tracking
- Ensures context is preserved
- Makes it easy to resume work
- Provides clear progress visibility

**Workflow:**
```bash
# 1. Complete current task item
# ... implementation ...

# 2. Test and verify coverage
npm test
npm test -- --coverage

# 3. Update tasks.md IMMEDIATELY
# Mark as [x] and add status comment

# 4. Verify update
rulebook task show <task-id>
# Confirm status is updated

# 5. ONLY THEN proceed to next task
# Follow priority order (most critical first)
```

**Example:**
```markdown
## 1. Implementation Phase
- [x] 1.1 Create task manager module <!-- tested, coverage: 95% -->
- [x] 1.2 Add validation logic <!-- tested, coverage: 92%, status: complete -->
- [ ] 1.3 Add archive functionality <!-- next: will start after status update -->
```

## Task Archiving Workflow

**CRITICAL**: Archive tasks ONLY after full completion and validation.

### When to Archive

**Archive a task when:**
- ✅ All items in `tasks.md` are marked as `[x]`
- ✅ All tests pass (unit, integration, E2E)
- ✅ Code review is complete (if applicable)
- ✅ Documentation is updated (README, CHANGELOG, specs)
- ✅ Task format is validated (`rulebook task validate <task-id>`)
- ✅ Spec deltas have been applied to main specifications

**NEVER archive a task that is:**
- ❌ Partially complete
- ❌ Missing tests
- ❌ Failing validation
- ❌ Missing documentation

### Archive Process

**Step-by-step archive workflow:**

```bash
# 1. Verify all tasks are complete
rulebook task show <task-id>
# Check that all items in tasks.md are [x]

# 2. Run all quality checks
npm test
npm run lint
npm run type-check
npm run build

# 3. Validate task format
rulebook task validate <task-id>

# 4. Update final documentation
# - Update CHANGELOG.md
# - Update README.md if needed
# - Update any affected documentation

# 5. Archive the task
rulebook task archive <task-id>

# 6. Verify archive
rulebook task list --archived
# Task should appear in archived list
```

### Post-Archive Actions

**After archiving, ensure:**
- ✅ Spec deltas are applied to main specifications
- ✅ CHANGELOG.md is updated with the change
- ✅ Any breaking changes are documented
- ✅ Migration guides are created (if needed)
- ✅ Related tasks are unblocked (if any)

### Archive Location

**Archived tasks are moved to:**
```
/rulebook/tasks/archive/YYYY-MM-DD-<task-id>/
```

**Structure:**
```
/rulebook/tasks/archive/2025-11-13-add-auth/
├── proposal.md
├── tasks.md          # All items marked [x]
├── design.md
└── specs/
    └── core/
        └── spec.md
```

## Task Creation Best Practices

### Task ID Naming

**Use verb-led kebab-case:**
- ✅ `add-user-authentication`
- ✅ `refactor-task-manager`
- ✅ `update-api-validation`
- ✅ `remove-legacy-code`
- ❌ `user-auth` (not descriptive)
- ❌ `task_manager` (use kebab-case)
- ❌ `new-feature` (too generic)

### Task Scope

**One capability per task:**
- ✅ Good: `add-email-notifications`
- ❌ Bad: `add-email-notifications-and-sms-and-push` (too broad)

**Break large tasks into smaller ones:**
- ✅ `add-email-notifications`
- ✅ `add-sms-notifications`
- ✅ `add-push-notifications`

### Task Checklist Structure

**Organize tasks by phase:**

```markdown
## 1. Planning & Design
- [ ] 1.1 Research existing solutions
- [ ] 1.2 Design architecture
- [ ] 1.3 Create technical spec

## 2. Implementation
- [ ] 2.1 Create core module
- [ ] 2.2 Add validation logic
- [ ] 2.3 Integrate with existing system

## 3. Testing
- [ ] 3.1 Write unit tests
- [ ] 3.2 Write integration tests
- [ ] 3.3 Test edge cases

## 4. Documentation
- [ ] 4.1 Update README
- [ ] 4.2 Update CHANGELOG
- [ ] 4.3 Add code comments

## 5. Cleanup
- [ ] 5.1 Remove debug code
- [ ] 5.2 Remove unused imports
- [ ] 5.3 Final code review
```

## Continuous Task Updates

**CRITICAL**: Update `tasks.md` continuously, not just at the end.

### Real-Time Updates

**Update as you work:**
1. **Start task**: Mark as `[ ]` (if not already)
2. **Begin implementation**: Add comment `<!-- in progress -->`
3. **Complete implementation**: Mark as `[x]`
4. **Test passes**: Add comment `<!-- tested -->`
5. **Ready for review**: Add comment `<!-- ready for review -->`

**Example:**
```markdown
## 1. Implementation
- [x] 1.1 Create task manager module <!-- tested -->
- [x] 1.2 Add validation logic <!-- tested, ready for review -->
- [ ] 1.3 Add archive functionality <!-- in progress -->
```

### Progress Tracking

**Add progress indicators:**
```markdown
## Progress: 60% (9/15 tasks complete)

## 1. Implementation Phase: 100% ✅
- [x] 1.1 Task 1
- [x] 1.2 Task 2
- [x] 1.3 Task 3

## 2. Testing Phase: 50% ⏳
- [x] 2.1 Unit tests
- [x] 2.2 Integration tests
- [ ] 2.3 E2E tests

## 3. Documentation Phase: 0% ⏸️
- [ ] 3.1 README
- [ ] 3.2 CHANGELOG
- [ ] 3.3 API docs
```

## Task Validation Before Archive

**MANDATORY checks before archiving:**

```bash
# 1. Format validation
rulebook task validate <task-id>
# Must pass all format checks

# 2. Completion check
# All items in tasks.md must be [x]

# 3. Test coverage
npm test -- --coverage
# Must meet coverage thresholds

# 4. Code quality
npm run lint
npm run type-check
# Must pass all checks

# 5. Build verification
npm run build
# Must build successfully
```

## Summary: Task Lifecycle

**Complete task lifecycle:**

1. **Create** (MANDATORY FIRST STEP): `rulebook task create <task-id>`
   - ⚠️ NEVER start implementation without creating task first
   - ⚠️ Tasks without registration can be lost in context

2. **Plan**: Write proposal.md and tasks.md
   - Define why, what, and how
   - Create implementation checklist

3. **Design**: Write design.md (if needed)
   - Technical decisions
   - Architecture choices

4. **Spec**: Write spec deltas in specs/
   - OpenSpec-compatible format
   - Requirements with SHALL/MUST

5. **Validate**: `rulebook task validate <task-id>`
   - Format validation
   - Structure verification

6. **Implement**: Write code, following priority order
   - Most critical tasks first
   - Update tasks.md as you go

7. **Test** (HIGHEST PRIORITY): Write tests, verify coverage
   - All tests must pass
   - Coverage must meet thresholds
   - Mark tested items in tasks.md

8. **Update Status** (MANDATORY): Update task status before next task
   - Mark completed items as `[x]`
   - Update status in tasks.md
   - Verify status update

9. **Document**: Update docs, mark in tasks.md
   - README, CHANGELOG, specs

10. **Validate**: Final validation before archive
    - All checks pass
    - Coverage verified

11. **Archive**: `rulebook task archive <task-id>`
    - Move to archive
    - Apply spec deltas

**CRITICAL REMINDERS:**
- ⚠️ **ALWAYS create task BEFORE implementation** - without registration, tasks can be lost
- ⚠️ **ALWAYS follow priority order** - most critical first (tests, coverage, status update)
- ⚠️ **ALWAYS update task status before next task** - prevents context loss
- ⚠️ **ALWAYS verify coverage** - run `npm test -- --coverage` before marking complete
- ⚠️ **ALWAYS commit locally frequently** - even for backup, prevents work loss
- ⚠️ **ALWAYS keep remote repository updated** - push regularly if remote is configured
- ⚠️ **ALWAYS update `tasks.md` at EVERY step**, not just at the end!

## Best Practices

### DO's ✅

- **ALWAYS** create task BEFORE implementing any feature
- **ALWAYS** check Context7 MCP before creating tasks
- **ALWAYS** validate task format before committing
- **ALWAYS** use SHALL/MUST in requirements
- **ALWAYS** use 4 hashtags for scenarios
- **ALWAYS** use Given/When/Then structure
- **ALWAYS** follow priority order (most critical first)
- **ALWAYS** write tests first (highest priority)
- **ALWAYS** verify test coverage before marking complete
- **ALWAYS** commit locally frequently (even for backup)
- **ALWAYS** keep remote repository updated (push regularly)
- **ALWAYS** update task status before moving to next task
- **ALWAYS** update task status during implementation
- **ALWAYS** archive completed tasks
- **ALWAYS** document breaking changes in proposal

### DON'Ts ❌

- **NEVER** start implementation without creating task first
- **NEVER** skip task registration (tasks can be lost in context)
- **NEVER** proceed to next task without updating current task status
- **NEVER** skip test coverage verification
- **NEVER** mark tasks complete without tests passing
- **NEVER** skip local commits (commit frequently for backup)
- **NEVER** let remote repository get out of sync (push regularly)
- **NEVER** commit sensitive information (API keys, passwords)
- **NEVER** force push to shared branches
- **NEVER** create tasks without checking Context7 format
- **NEVER** use 3 hashtags for scenarios
- **NEVER** omit SHALL/MUST from requirements
- **NEVER** use bullet points for scenarios
- **NEVER** skip validation
- **NEVER** leave tasks unarchived after completion
- **NEVER** mix formats (stick to OpenSpec-compatible format)
- **NEVER** ignore priority order (always do most critical first)

## CLI Commands Reference

### Task Management Commands

#### `rulebook task create <task-id>`

Create a new Rulebook task with OpenSpec-compatible format.

**Usage:**
```bash
rulebook task create add-user-authentication
```

**What it does:**
- Creates `/rulebook/tasks/<task-id>/` directory
- Generates `proposal.md` template
- Generates `tasks.md` template
- Creates `specs/` directory for spec deltas

**Requirements:**
- Task ID must be unique (verb-led kebab-case)
- Context7 MCP must be available (for format validation)

**Example:**
```bash
$ rulebook task create add-email-notifications
✅ Task add-email-notifications created successfully
Location: rulebook/tasks/add-email-notifications/

⚠️  Remember to:
  1. Check Context7 MCP for OpenSpec format requirements
  2. Fill in proposal.md (minimum 20 characters in "Why" section)
  3. Add tasks to tasks.md
  4. Create spec deltas in specs/*/spec.md
  5. Validate with: rulebook task validate add-email-notifications
```

**Error Handling:**
- `Task <task-id> already exists`: Choose a different task ID or archive existing task

---

#### `rulebook task list [--archived]`

List all Rulebook tasks (active and optionally archived).

**Usage:**
```bash
# List active tasks only
rulebook task list

# List including archived tasks
rulebook task list --archived
```

**Output:**
- Active tasks with status (pending, in-progress, completed, blocked)
- Archived tasks with archive date (if --archived flag is used)

**Example:**
```bash
$ rulebook task list

📋 Rulebook Tasks

Active Tasks:
  pending      add-user-authentication - Add user authentication feature
  in-progress  refactor-api-validation - Refactor API validation logic
  completed    update-documentation - Update project documentation

$ rulebook task list --archived

📋 Rulebook Tasks

Active Tasks:
  pending      add-user-authentication - Add user authentication feature

Archived Tasks:
  archived     2025-01-15-add-email-notifications - Add email notifications (2025-01-15)
```

**Task Status Values:**
- `pending`: Task not started
- `in-progress`: Task being worked on
- `completed`: Task finished (ready for archive)
- `blocked`: Task blocked by dependency

---

#### `rulebook task show <task-id>`

Show detailed information about a specific task.

**Usage:**
```bash
rulebook task show add-user-authentication
```

**Output:**
- Task ID and title
- Status (pending, in-progress, completed, blocked)
- Created and updated dates
- Archive date (if archived)
- Proposal summary (first 500 characters)
- Spec files list

**Example:**
```bash
$ rulebook task show add-user-authentication

📋 Task: add-user-authentication

Title: add-user-authentication
Status: pending
Created: 2025-01-15T10:30:00.000Z
Updated: 2025-01-15T10:30:00.000Z

Proposal:
# Proposal: Add User Authentication

## Why
We need to implement secure user authentication to protect user accounts and enable personalized features. This will include JWT token-based authentication with refresh tokens and password hashing using bcrypt...

Specs:
  core/spec.md (1234 chars)
```

**Error Handling:**
- `Task <task-id> not found`: Verify task ID exists with `rulebook task list`

---

#### `rulebook task validate <task-id>`

Validate task format against OpenSpec-compatible requirements.

**Usage:**
```bash
rulebook task validate add-user-authentication
```

**Validation Checks:**
- Purpose section length (≥20 characters)
- Requirement keywords (SHALL/MUST)
- Scenario format (4 hashtags, not 3)
- Given/When/Then structure
- Delta headers format (ADDED/MODIFIED/REMOVED/RENAMED)

**Example:**
```bash
$ rulebook task validate add-user-authentication
✅ Task add-user-authentication is valid

⚠️  Warnings:
  - Scenario in core/spec.md should use Given/When/Then structure
```

**Error Example:**
```bash
$ rulebook task validate invalid-task
❌ Task invalid-task validation failed

Errors:
  - Scenarios in core/spec.md must use 4 hashtags (####), not 3 (###)
  - Requirement in core/spec.md missing SHALL or MUST keyword: ### Requirement: Auth
  - Purpose section (## Why) must have at least 20 characters
```

**Error Handling:**
- Fix all errors before proceeding
- Warnings are informational but don't block archiving

---

#### `rulebook task archive <task-id> [--skip-validation]`

Archive a completed task and apply spec deltas to main specifications.

**Usage:**
```bash
# Archive with validation (recommended)
rulebook task archive add-user-authentication

# Archive without validation (use with caution)
rulebook task archive add-user-authentication --skip-validation
```

**Archive Process:**
1. Validates task format (unless `--skip-validation` is used)
2. Checks task completion status
3. Applies spec deltas to main specifications
4. Moves task to `/rulebook/tasks/archive/YYYY-MM-DD-<task-id>/`
5. Updates related specifications

**Example:**
```bash
$ rulebook task archive add-user-authentication
✅ Task add-user-authentication archived successfully
```

**Error Handling:**
- `Task validation failed`: Fix validation errors before archiving
- `Task <task-id> not found`: Verify task ID exists
- `Archive <archive-name> already exists`: Archive with that date already exists

**Important:**
- Only archive tasks that are fully completed
- All items in `tasks.md` should be marked as `[x]`
- All tests should pass
- Documentation should be updated

---

### Core Rulebook Commands

#### `rulebook init [--minimal] [--light] [--yes]`

Initialize Rulebook for current project.

**Usage:**
```bash
# Interactive mode
rulebook init

# Minimal setup (essentials only)
rulebook init --minimal

# Light mode (no quality enforcement)
rulebook init --light

# Skip prompts, use defaults
rulebook init --yes
```

**What it does:**
- Detects languages, frameworks, and MCP modules
- Generates AGENTS.md with AI assistant rules
- Creates `/rulebook/` directory with templates
- Creates/updates `.gitignore` automatically
- Optionally installs Git hooks
- Generates Cursor commands (if Cursor is selected IDE)

---

#### `rulebook update [--yes] [--minimal] [--light]`

Update AGENTS.md and .rulebook to latest version.

**Usage:**
```bash
# Interactive mode
rulebook update

# Skip confirmation
rulebook update --yes

# Minimal mode
rulebook update --minimal

# Light mode
rulebook update --light
```

**What it does:**
- Migrates OpenSpec tasks to Rulebook format (if OpenSpec exists)
- Migrates OpenSpec archives to Rulebook format
- Removes OpenSpec commands from `.cursor/commands/`
- Updates AGENTS.md with latest templates
- Merges templates while preserving customizations
- Updates Cursor commands (if Cursor is selected IDE)

---

#### `rulebook validate`

Validate project structure against Rulebook standards.

**Usage:**
```bash
rulebook validate
```

**Validation Checks:**
- AGENTS.md presence and format
- Rulebook directory structure
- Documentation structure
- Tests directory
- Score calculation (0-100)

---

#### `rulebook health`

Check project health score.

**Usage:**
```bash
rulebook health
```

**Categories Scored:**
- Quality (linting, formatting, code quality)
- Testing (test coverage, test quality)
- Security (vulnerabilities, secrets)
- Documentation (README, docs/, comments)

**Score Range:** 0-100

---

#### `rulebook workflows`

Generate GitHub Actions workflows for detected languages.

**Usage:**
```bash
rulebook workflows
```

**What it does:**
- Creates `.github/workflows/` directory
- Generates language-specific workflows (test, lint, publish)
- Adds codespell workflow for spelling checks

---

#### `rulebook check-deps`

Check for outdated and vulnerable dependencies.

**Usage:**
```bash
rulebook check-deps
```

**Supported Package Managers:**
- npm (package.json)
- Cargo (Cargo.toml)
- pip (requirements.txt, pyproject.toml)
- Go modules (go.mod)

---

#### `rulebook check-coverage [-t <threshold>]`

Check test coverage against threshold.

**Usage:**
```bash
# Default threshold (95%)
rulebook check-coverage

# Custom threshold
rulebook check-coverage -t 80
```

---

#### `rulebook generate-docs [--yes]`

Generate documentation structure and standard files.

**Usage:**
```bash
# Interactive mode
rulebook generate-docs

# Skip prompts
rulebook generate-docs --yes
```

---

#### `rulebook version <major|minor|patch>`

Bump project version (semantic versioning).

**Usage:**
```bash
rulebook version major  # 1.0.0 -> 2.0.0
rulebook version minor  # 1.0.0 -> 1.1.0
rulebook version patch  # 1.0.0 -> 1.0.1
```

---

#### `rulebook changelog [-v <version>]`

Generate changelog from git commits.

**Usage:**
```bash
# Auto-detect version
rulebook changelog

# Specify version
rulebook changelog -v 1.0.0
```

---

#### `rulebook fix`

Auto-fix common project issues.

**Usage:**
```bash
rulebook fix
```

---

### Advanced Commands (Beta)

#### `rulebook watcher`

Start modern full-screen console watcher for task progress.

**Usage:**
```bash
rulebook watcher
```

**Features:**
- Live task progress tracking
- Activity log with timestamps
- System status monitoring
- Auto-refresh every 2 seconds

---

#### `rulebook agent [--dry-run] [--tool <name>] [--iterations <n>] [--watch]`

Start autonomous agent for managing AI CLI workflows.

**Usage:**
```bash
# Dry run (simulate without changes)
rulebook agent --dry-run

# Specify CLI tool
rulebook agent --tool cursor-agent

# Set max iterations
rulebook agent --iterations 10

# Enable watcher mode
rulebook agent --watch
```

---

#### `rulebook config [--show] [--set <key=value>] [--feature <name> --enable|--disable]`

Manage Rulebook configuration.

**Usage:**
```bash
# Show current config
rulebook config --show

# Set config value
rulebook config --set rulebookDir=custom-rulebook

# Enable feature
rulebook config --feature watcher --enable

# Disable feature
rulebook config --feature agent --disable
```

## Migration from OpenSpec

If your project previously used OpenSpec:

1. **Automatic Migration**: Run `rulebook update` to automatically migrate OpenSpec tasks to Rulebook format
2. **Manual Migration**: Tasks in `/openspec/changes/` will be moved to `/rulebook/tasks/`
3. **Format Compatibility**: Rulebook uses OpenSpec-compatible format, so existing tasks remain valid

## Context7 MCP Requirement

**CRITICAL**: Context7 MCP is REQUIRED for task creation.

**Why**: 
- Ensures correct format by fetching official OpenSpec documentation
- Prevents common format errors made by AI assistants
- Provides up-to-date format requirements

**If Context7 MCP is not available:**
- Task creation will fail with clear error message
- You must configure Context7 MCP before creating tasks
- See `/rulebook/CONTEXT7.md` for setup instructions

## Troubleshooting

### Validation Errors

**Error**: "Requirement must contain SHALL or MUST keyword"
- **Fix**: Add SHALL or MUST to requirement text
- **Example**: Change "The system provides authentication" to "The system SHALL provide authentication"

**Error**: "Scenario must use 4 hashtags"
- **Fix**: Change `### Scenario:` to `#### Scenario:` (at start of line)
- **Note**: Validation only checks headers at start of line, not in text content

**Error**: "Purpose section too short"
- **Fix**: Expand "Why" section in proposal.md to at least 20 characters
- **Example**: "Auth system" → "Authentication system for secure user access with JWT tokens and session management"

**Error**: "Scenario must use Given/When/Then structure"
- **Fix**: Replace bullet points with Given/When/Then format
- **Example**: 
  ```markdown
  #### Scenario: User login
  Given a user has valid credentials
  When they submit the login form
  Then they are authenticated successfully
  ```

### Task Creation Errors

**Error**: "Context7 MCP not available"
- **Fix**: Configure Context7 MCP in your MCP configuration file
- **See**: `/rulebook/CONTEXT7.md` for setup instructions

**Error**: "Task ID already exists"
- **Fix**: Choose a different task ID or archive existing task
- **Check**: Use `rulebook task list` to see existing tasks

### Task Archive Errors

**Error**: "Task validation failed"
- **Fix**: Run `rulebook task validate <task-id>` to see all errors
- **Fix**: Address all validation errors before archiving
- **Option**: Use `--skip-validation` flag only if you're certain the task is valid

**Error**: "Archive <archive-name> already exists"
- **Fix**: Archive with that date already exists
- **Check**: Use `rulebook task list --archived` to see archived tasks

### Command Errors

**Error**: "Task <task-id> not found"
- **Fix**: Verify task ID exists with `rulebook task list`
- **Check**: Ensure you're in the correct project directory

**Error**: "No tasks found"
- **Fix**: Create a task first with `rulebook task create <task-id>`
- **Check**: Verify `/rulebook/tasks/` directory exists

### Migration Errors

**Error**: "Failed to migrate task"
- **Fix**: Check error message for specific issue
- **Check**: Verify OpenSpec task structure is correct
- **Fix**: Manually migrate if automatic migration fails

**Error**: "Failed to read OpenSpec changes directory"
- **Fix**: Verify `/openspec/changes/` directory exists
- **Check**: Ensure you have read permissions

## Examples

See `/rulebook/tasks/` directory for examples of correctly formatted tasks.

<!-- RULEBOOK:END -->