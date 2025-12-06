<!-- GIT:START -->

**AI Assistant Git Push Mode**: MANUAL

**CRITICAL**: Never execute `git push` commands automatically.
Always provide push commands for manual execution by the user.

Example:
```
✋ MANUAL ACTION REQUIRED:
Run these commands manually (SSH password may be required):
  git push origin main
  git push origin v1.0.0
```

# Git Workflow Rules

**CRITICAL**: Specific rules and patterns for Git version control workflow.

## Git Workflow Overview

This project follows a strict Git workflow to ensure code quality and proper version control.

**NEVER commit code without tests passing. NEVER create tags without full quality checks.**

## Initial Repository Setup

### New Project Initialization

**⚠️ CRITICAL**: Only run initialization commands if `.git` directory does NOT exist!

```bash
# Check if Git repository already exists
if [ -d .git ]; then
  echo "❌ Git repository already initialized. Skipping git init."
  echo "Current status:"
  git status
  git remote -v
  exit 0
fi

# If no .git directory exists, initialize:

# Initialize Git repository
git init

# Add all files
git add .

# Initial commit
git commit -m "chore: Initial project setup"

# Rename default branch to main (GitHub standard)
git branch -M main

# Add remote (if applicable)
git remote add origin <repository-url>
```

**AI Assistant Behavior:**

```
BEFORE running any Git initialization commands:

1. Check if .git directory exists
2. If exists:
   ✅ Repository already configured
   ❌ DO NOT run: git init
   ❌ DO NOT run: git branch -M main
   ✅ Check status: git status
   ✅ Show remotes: git remote -v
   
3. If not exists:
   ✅ Safe to initialize
   ✅ Run full initialization sequence
```

## AI Assistant Git Checks

**CRITICAL**: AI assistants MUST perform these checks before Git operations:

### Automatic Checks

```bash
# 1. Check if Git repository exists
if [ ! -d .git ]; then
  echo "No Git repository found."
  # Ask user if they want to initialize
fi

# 2. Check if there are unstaged changes
git status --short

# 3. Check current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "On branch: $CURRENT_BRANCH"

# 4. Check if remote exists
git remote -v

# 5. Check for unpushed commits
git log origin/main..HEAD --oneline 2>/dev/null
```

### Before Git Commands

**NEVER execute if `.git` directory exists:**
- ❌ `git init` - Repository already initialized
- ❌ `git branch -M main` - Branch may already be configured
- ❌ `git remote add origin` - Remote may already exist (check first with `git remote -v`)
- ❌ `git config user.name/email` - User configuration is personal
- ❌ Reconfiguration commands - Repository is already set up

**ALWAYS safe to execute:**
- ✅ `git status` - Check repository state
- ✅ `git add` - Stage changes
- ✅ `git commit` - Create commits (after quality checks)
- ✅ `git log` - View history
- ✅ `git diff` - View changes
- ✅ `git branch` - List branches
- ✅ `git tag` - Create tags (after quality checks)

**Execute with caution (check first):**
- ⚠️ `git push` - Follow push mode configuration
- ⚠️ `git pull` - May cause merge conflicts
- ⚠️ `git merge` - May cause conflicts
- ⚠️ `git rebase` - Can rewrite history
- ⚠️ `git reset --hard` - Destructive, only for rollback
- ⚠️ `git push --force` - NEVER on main/master

### Repository Detection

**AI Assistant MUST check:**

```bash
# Before ANY Git operation:

# 1. Does .git exist?
if [ -d .git ]; then
  echo "✅ Git repository exists"
  
  # 2. Check current state
  git status
  
  # 3. Check branch
  BRANCH=$(git branch --show-current)
  echo "On branch: $BRANCH"
  
  # 4. Check remote
  REMOTE=$(git remote -v)
  if [ -z "$REMOTE" ]; then
    echo "⚠️  No remote configured"
  else
    echo "Remote: $REMOTE"
  fi
  
  # 5. Proceed with normal Git operations
else
  echo "⚠️  No Git repository found"
  echo "Ask user if they want to initialize Git"
fi
```

## Daily Development Workflow

### 1. Before Making Changes

**CRITICAL**: Always check current state:

```bash
# Check current branch and status
git status

# Ensure you're on the correct branch
git branch

# Pull latest changes if working with team (use --ff-only for safety)
git pull --ff-only origin main
```

**Git Safety**: Use `--ff-only` to prevent unexpected merge commits and maintain linear history.

### 2. Making Changes

**CRITICAL**: Commit after every important implementation:

**⚠️ IMPORTANT: All commit messages MUST be in English**

```bash
# After implementing a feature/fix:

# 1. Run ALL quality checks FIRST
npm run lint           # or equivalent for your language
npm run type-check     # TypeScript/typed languages
npm test              # ALL tests must pass
npm run build         # Ensure build succeeds

# 2. If ALL checks pass, stage changes
git add .

# 3. Commit with conventional commit message (ENGLISH ONLY)
git commit -m "feat: Add user authentication

- Implement login/logout functionality
- Add JWT token management
- Include comprehensive tests (95%+ coverage)
- Update documentation"

# Alternative for smaller changes (ENGLISH ONLY):
git commit -m "fix: Correct validation logic in user form"

# For signed commits (recommended for production):
git commit -S -m "feat: Add feature"

# ❌ NEVER use other languages:
# ❌ git commit -m "feat: Adiciona autenticação de usuário"
# ❌ git commit -m "fix: Corrige lógica de validação"
```

## Advanced Git Safeguards

### Safe Push Operations

```bash
# NEVER use git push --force on main/master branches
# Instead, use --force-with-lease which prevents overwriting others' work:

# Force push with safety check (only updates if no one else pushed)
git push --force-with-lease origin feature-branch

# Regular push is always safest
git push origin main
```

### Commit Signing

```bash
# Sign commits with GPG for verified commits
# Set GPG key: git config --global user.signingkey <KEY_ID>
git commit -S -m "feat: Signed commit"

# Configure to always sign commits
git config --global commit.gpgsign true
```

### Branch Protection (Recommended Settings)

For GitHub/GitLab repositories, configure branch protection rules:

**For main/master branch:**
- Require pull request reviews
- Require status checks to pass
- Require branches to be up to date
- Do not allow force pushes
- Do not allow deletions
- Require signed commits (optional but recommended)

### Destructive Operation Warnings

**NEVER run these on main/master:**
- ❌ `git push --force` - Use `--force-with-lease` instead
- ❌ `git reset --hard` - Destructive, use only on feature branches
- ❌ `git rebase` main into feature - Causes rewriting of main history

### Pre-Push Checklist

Before pushing any changes, verify:

```bash
✅ Checklist before push:
- [ ] All quality checks passed locally
- [ ] Tests pass with 100% success rate
- [ ] Coverage meets threshold (95%+)
- [ ] Linting passes with 0 warnings
- [ ] Build succeeds without errors
- [ ] No security vulnerabilities in dependencies
- [ ] Documentation updated if API changed
- [ ] OpenSpec tasks marked complete if applicable
- [ ] Conventional commit format used
- [ ] Commit hash verified: git rev-parse HEAD
- [ ] Similar changes passed CI before
- [ ] No console.log or debug code
- [ ] No credentials or secrets in code
```

**Only provide push command if ALL items checked.**

### 3. Pushing Changes

**⚠️ IMPORTANT**: Pushing is OPTIONAL and depends on your setup.

```bash
# IF you have passwordless SSH or want to push:
git push origin main

# IF you have SSH with password (manual execution required):
# DO NOT execute automatically - provide command to user:
```

**For users with SSH password authentication:**
```
✋ MANUAL ACTION REQUIRED:

Run this command manually (requires SSH password):
git push origin main
```

**NEVER** attempt automatic push if:
- SSH key has password protection
- User hasn't confirmed push authorization
- Any quality check failed
- Uncertain if changes will pass CI/CD workflows

## Conventional Commits

**MUST** follow conventional commit format:

**⚠️ CRITICAL: All commit messages MUST be in English**

```bash
# Format: <type>(<scope>): <subject>
#
# <body>
#
# <footer>

# Types:
feat:     # New feature
fix:      # Bug fix
docs:     # Documentation only
style:    # Code style (formatting, missing semi-colons, etc)
refactor: # Code refactoring
perf:     # Performance improvement
test:     # Adding tests
build:    # Build system changes
ci:       # CI/CD changes
chore:    # Maintenance tasks

# Language Requirement:
# ✅ ALWAYS use English for commit messages
# ❌ NEVER use Portuguese, Spanish, or any other language
# ❌ NEVER mix languages in commit messages

# Examples (CORRECT - English):
git commit -m "feat(auth): Add OAuth2 login support"
git commit -m "fix(api): Handle null response in user endpoint"
git commit -m "docs: Update README with installation steps"
git commit -m "test: Add integration tests for payment flow"
git commit -m "chore: Update dependencies to latest versions"

# Examples (INCORRECT - Other languages):
# ❌ git commit -m "feat: Adiciona suporte de login OAuth2"
# ❌ git commit -m "fix: Corrige resposta nula no endpoint"
# ❌ git commit -m "docs: Atualiza README com passos de instalação"
```

## Version Management

### Creating New Version

**CRITICAL**: Full quality gate required before versioning!

```bash
# 1. MANDATORY: Run complete quality suite
npm run lint          # Must pass with no warnings
npm test             # Must pass 100%
npm run type-check   # Must pass (if applicable)
npm run build        # Must succeed
npx codespell        # Must pass (if configured)

# 2. Update version in package.json/Cargo.toml/etc
# Use semantic versioning:
# - MAJOR: Breaking changes (1.0.0 -> 2.0.0)
# - MINOR: New features, backwards compatible (1.0.0 -> 1.1.0)
# - PATCH: Bug fixes (1.0.0 -> 1.0.1)

# 3. Update CHANGELOG.md
# Document all changes in this version:
## [1.2.0] - 2024-01-15
### Added
- New feature X
- New feature Y

### Fixed
- Bug in component Z

### Changed
- Refactored module A

# 4. Commit version changes
git add .
git commit -m "chore: Release version 1.2.0

- Updated version to 1.2.0
- Updated CHANGELOG.md with release notes"

# 5. Create annotated tag
git tag -a v1.2.0 -m "Release version 1.2.0

Major changes:
- Feature X
- Feature Y
- Bug fix Z

All tests passing ✅
Coverage: 95%+ ✅
Linting: Clean ✅
Build: Success ✅"

# 6. OPTIONAL: Push tag (manual if SSH password)
# Only if you're CERTAIN it will pass CI/CD workflows!
```

**For users requiring manual push:**
```
✋ MANUAL ACTIONS REQUIRED:

1. Verify all quality checks passed locally
2. Push commits:
   git push origin main

3. Push tag:
   git push origin v1.2.0

Note: Tag push will trigger CI/CD workflows and may create GitHub release.
Only push if you're confident all checks will pass.
```

## Quality Gate Enforcement

**CRITICAL**: Pre-commit checks MUST match GitHub Actions workflow commands to prevent CI/CD failures.

### Language-Specific Pre-Commit Commands

**The commands you run locally MUST be identical to those in your GitHub Actions workflows.**

#### TypeScript/JavaScript Projects

```bash
# These commands MUST match .github/workflows/*.yml

# 1. Type check (matches workflow)
npm run type-check        # Must match workflow exactly

# 2. Lint (matches workflow)
npm run lint              # Must match workflow exactly

# 3. Format check (matches workflow)
npx prettier --check 'src/**/*.ts' 'tests/**/*.ts'  # Must match workflow

# 4. Tests (matches workflow)
npm test                  # Must match workflow exactly

# 5. Build (matches workflow)
npm run build             # Must match workflow exactly

# If ANY fails: ❌ DO NOT COMMIT - Fix first!
```

#### Rust Projects

```bash
# These commands MUST match .github/workflows/*.yml

# 1. Format check (matches workflow)
cargo fmt --all -- --check

# 2. Clippy (matches workflow)
cargo clippy --all-targets --all-features -- -D warnings

# 3. Tests (matches workflow)
cargo test --all-features

# 4. Build (matches workflow)
cargo build --release

# If ANY fails: ❌ DO NOT COMMIT - Fix first!
```

#### Python Projects

```bash
# These commands MUST match .github/workflows/*.yml

# 1. Format check (matches workflow)
black --check .

# 2. Lint (matches workflow)
ruff check .

# 3. Type check (matches workflow)
mypy .

# 4. Tests (matches workflow)
pytest

# If ANY fails: ❌ DO NOT COMMIT - Fix first!
```

### Before ANY Commit

**MANDATORY CHECKS**:

```bash
# Checklist - ALL must pass:
☐ Code formatted
☐ Linter passes (no warnings)
☐ Type check passes
☐ ALL tests pass (100%)
☐ Coverage meets threshold (95%+)
☐ Build succeeds
☐ No console errors/warnings

# Run quality check script:
npm run quality-check  # or equivalent

# If ANY check fails:
# ❌ DO NOT COMMIT
# ❌ FIX THE ISSUES FIRST
```

### Before Tag Creation

**MANDATORY CHECKS** (even stricter):

```bash
# Extended checklist - ALL must pass:
☐ All pre-commit checks passed
☐ Codespell passes (no typos)
☐ Security audit clean
☐ Dependencies up to date
☐ Documentation updated
☐ CHANGELOG.md updated
☐ Version bumped correctly
☐ All workflows would pass

# Run comprehensive check:
npm run lint
npm test
npm run type-check
npm run build
npx codespell
npm audit

# Only create tag if everything is green!
```

## Error Recovery & Rollback

### When Implementation Is Failing

If the AI is making repeated mistakes and user is frustrated:

```bash
# 1. Identify last stable commit
git log --oneline -10

# 2. Create backup branch of current work
git branch backup-failed-attempt

# 3. Hard reset to last stable version
git reset --hard <last-stable-commit-hash>

# 4. Verify stability
npm test
npm run build

# 5. Reimplement from scratch using DIFFERENT approach
# ⚠️ DO NOT repeat the same techniques that failed before
# ⚠️ Review AGENTS.md for alternative patterns
# ⚠️ Consider different architecture/design

# 6. After successful reimplementation
git branch -D backup-failed-attempt  # Delete backup if no longer needed
```

### Undo Last Commit (Not Pushed)

```bash
# Keep changes, undo commit
git reset --soft HEAD~1

# Discard changes completely
git reset --hard HEAD~1
```

### Revert Pushed Commit

```bash
# Create revert commit
git revert <commit-hash>

# Then push (manual if SSH password)
```

## Branch Strategy

### Feature Branches

```bash
# Create feature branch
git checkout -b feature/user-authentication

# Work on feature...
# Commit regularly with quality checks

# When feature complete and tested:
git checkout main
git merge feature/user-authentication

# Delete feature branch
git branch -d feature/user-authentication
```

### Hotfix Workflow

```bash
# Critical bug in production
git checkout -b hotfix/critical-security-fix

# Fix the bug
# MUST include tests
# MUST pass all quality checks

git commit -m "fix: Critical security vulnerability in auth

- Patch authentication bypass
- Add regression tests
- Update security documentation"

# Merge to main
git checkout main
git merge hotfix/critical-security-fix

# Tag immediately if production fix
git tag -a v1.2.1 -m "Hotfix: Security patch"

# Manual push if required
```

## CRITICAL RESTRICTIONS - HUMAN AUTHORIZATION REQUIRED

**⚠️ IMPERATIVE RULES - THESE ARE NON-NEGOTIABLE ⚠️**

### Destructive Git Operations

**ABSOLUTELY FORBIDDEN without explicit human authorization:**

```
❌ NEVER execute: git checkout
   ✋ ALWAYS ask user: "Do you want to checkout [branch/commit]? [Y/n]"
   ✅ Only execute after explicit user confirmation
   
❌ NEVER execute: git reset
   ✋ ALWAYS ask user: "Do you want to reset to [commit]? This may lose changes. [Y/n]"
   ✅ Only execute after explicit user confirmation
   ⚠️  Explain consequences before executing
```

**Rationale**: These commands can cause data loss. Human oversight is mandatory.

### Merge Conflict Resolution

**When merge conflicts occur:**

```
❌ NEVER attempt to resolve conflicts by editing files automatically
❌ NEVER commit merged files without human review
✅ ALWAYS stop and request human assistance
✅ ALWAYS provide conflict locations and context
✅ ALWAYS wait for human to resolve manually

Message to user:
"⚠️ Merge conflict detected in the following files:
- [list of conflicted files]

Please resolve these conflicts manually. I cannot auto-resolve merge conflicts.

To resolve:
1. Open the conflicted files
2. Look for conflict markers (<<<<<<<, =======, >>>>>>>)
3. Choose the correct version or merge manually
4. Remove conflict markers
5. Run: git add <resolved-files>
6. Run: git commit

Let me know when you're done, and I can help with the next steps."
```

**Rationale**: Merge conflicts require human judgment about which code to keep.

### Commit Frequency Management

**⚠️ IMPORTANT: Reduce excessive commits**

```
❌ DO NOT commit after every small change
❌ DO NOT create multiple commits for the same logical feature
✅ COMMIT only when:
   - A complete feature is implemented and tested
   - A significant bug fix is completed
   - A major refactoring is done
   - Before creating a version tag
   - User explicitly requests a commit
   
✅ GROUP related changes into meaningful commits
✅ USE conventional commit messages that describe the full scope

Example of GOOD commit frequency:
- Implement entire authentication system → 1 commit
- Add login, logout, and session management → 1 commit
- Complete feature with tests and docs → 1 commit

Example of BAD commit frequency (AVOID):
- Add login function → commit
- Add logout function → commit  
- Add session check → commit
- Fix typo → commit
- Update comment → commit
```

**Rationale**: Too many commits pollute git history and make it harder to track meaningful changes.

### Feature Branch Strategy

**BEFORE starting ANY new task or feature:**

```
✋ ALWAYS ask user FIRST:
"Should I create a separate branch for this feature/task? [Y/n]

Options:
1. Create feature branch: git checkout -b feature/[name]
2. Work directly on current branch
3. Create hotfix branch: git checkout -b hotfix/[name]

What would you prefer?"

✅ Wait for user decision
✅ Respect user's branching strategy
❌ NEVER assume to work on main without asking
❌ NEVER create branches without permission

If user says YES to branch:
  → Create branch with descriptive name
  → Work on that branch
  → Ask before merging back to main
  
If user says NO to branch:
  → Proceed on current branch
  → Be extra careful with commits
```

**Rationale**: Branching strategy varies by team and project. Always confirm with the human first.

## Critical AI Assistant Rules

### Repository Initialization

**BEFORE any `git init` or setup commands:**

```
1. Check for .git directory existence
2. If .git exists:
   - ❌ STOP - Repository already configured
   - ❌ DO NOT run git init
   - ❌ DO NOT run git config
   - ❌ DO NOT run git branch -M
   - ❌ DO NOT reconfigure anything
   - ✅ Use existing repository as-is
   
3. If .git does NOT exist:
   - ✅ Ask user if they want Git initialization
   - ✅ Run initialization sequence if approved
```

### Push Command Behavior

**Based on configured push mode:**

```
Manual Mode (DEFAULT):
  ❌ NEVER execute: git push
  ✅ ALWAYS provide: "Run manually: git push origin main"
  
Prompt Mode:
  ⚠️  ALWAYS ask first: "Ready to push. Proceed? [Y/n]"
  ✅ Execute only if user confirms
  
Auto Mode:
  ⚠️  Check quality first
  ⚠️  Only if 100% confident
  ✅ Execute if all checks passed
```

### Quality Gate Enforcement

**MANDATORY checks before commit:**

```bash
# Run in this exact order:
1. npm run lint          # or language equivalent
2. npm run type-check    # if applicable
3. npm test             # ALL tests must pass
4. npm run build        # must succeed

# If ANY fails:
❌ STOP - DO NOT commit
❌ Fix issues first
❌ Re-run all checks

# If ALL pass:
✅ Safe to commit
✅ Proceed with git add and commit
```

**MANDATORY checks before tag:**

```bash
# Extended checks for version tags:
1. All commit checks above +
2. npx codespell        # no typos
3. npm audit            # no vulnerabilities
4. CHANGELOG.md updated
5. Version bumped correctly
6. Documentation current

# If ANY fails:
❌ STOP - DO NOT create tag
❌ Fix issues
❌ Re-verify everything

# Only create tag if 100% green!
```

## Best Practices

### DO's ✅

- **ALWAYS** check if .git exists before init commands
- **ALWAYS** run tests before commit
- **ALWAYS** use conventional commit messages
- **ALWAYS** write commit messages in English (never in Portuguese, Spanish, or other languages)
- **ALWAYS** update CHANGELOG for versions
- **ALWAYS** ask before executing `git checkout`
- **ALWAYS** ask before executing `git reset`
- **ALWAYS** ask user if a feature branch should be created before starting tasks
- **ALWAYS** request human assistance when merge conflicts occur
- **COMMIT** only when complete features/fixes are done (not for every small change)
- **TAG** releases with semantic versions
- **VERIFY** quality gates before tagging
- **DOCUMENT** breaking changes clearly
- **REVERT** when implementation is failing repeatedly
- **ASK** user before automatic push
- **PROVIDE** manual commands for SSH password users
- **CHECK** repository state before operations
- **RESPECT** existing Git configuration
- **GROUP** related changes into meaningful commits

### DON'Ts ❌

- **NEVER** run `git init` if .git exists
- **NEVER** run `git config` (user-specific)
- **NEVER** run `git checkout` without explicit user authorization
- **NEVER** run `git reset` without explicit user authorization
- **NEVER** auto-resolve merge conflicts by editing files
- **NEVER** commit merged files without human review
- **NEVER** create excessive commits for small changes
- **NEVER** assume branching strategy - always ask user first
- **NEVER** reconfigure existing repository
- **NEVER** commit without passing tests
- **NEVER** commit with linting errors
- **NEVER** commit with build failures
- **NEVER** write commit messages in languages other than English
- **NEVER** mix languages in commit messages
- **NEVER** create tag without quality checks
- **NEVER** push automatically with SSH password
- **NEVER** push if uncertain about CI/CD success
- **NEVER** commit console.log/debug code
- **NEVER** commit credentials or secrets
- **NEVER** force push to main/master
- **NEVER** rewrite published history
- **NEVER** skip hooks (--no-verify)
- **NEVER** assume repository configuration

## SSH Configuration

### For Users with SSH Password

If your SSH key has password protection:

**Configuration in AGENTS.md or project settings:**

```yaml
git_workflow:
  auto_push: false
  push_mode: "manual"
  reason: "SSH key has password protection"
```

**AI Assistant Behavior:**
- ✅ Provide push commands in chat
- ✅ Wait for user manual execution
- ❌ Never attempt automatic push
- ❌ Never execute git push commands

### For Users with Passwordless SSH

```yaml
git_workflow:
  auto_push: true  # or prompt each time
  push_mode: "auto"
```

## Git Hooks

### Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/sh

echo "Running pre-commit checks..."

# Run linter
npm run lint
if [ $? -ne 0 ]; then
  echo "❌ Linting failed. Commit aborted."
  exit 1
fi

# Run tests
npm test
if [ $? -ne 0 ]; then
  echo "❌ Tests failed. Commit aborted."
  exit 1
fi

# Run type check (if applicable)
if command -v tsc &> /dev/null; then
  npm run type-check
  if [ $? -ne 0 ]; then
    echo "❌ Type check failed. Commit aborted."
    exit 1
  fi
fi

echo "✅ All pre-commit checks passed!"
exit 0
```

### Pre-push Hook

Create `.git/hooks/pre-push`:

```bash
#!/bin/sh

echo "Running pre-push checks..."

# Run full test suite
npm test
if [ $? -ne 0 ]; then
  echo "❌ Tests failed. Push aborted."
  exit 1
fi

# Run build
npm run build
if [ $? -ne 0 ]; then
  echo "❌ Build failed. Push aborted."
  exit 1
fi

echo "✅ All pre-push checks passed!"
exit 0
```

Make hooks executable:
```bash
chmod +x .git/hooks/pre-commit
chmod +x .git/hooks/pre-push
```

## CI/CD Integration

### Before Providing Push Commands

**CRITICAL**: Only suggest push if confident about CI/CD success:

```
✅ Provide push command if:
- All local tests passed
- All linting passed
- Build succeeded
- Coverage meets threshold
- No warnings or errors
- Code follows AGENTS.md standards
- Similar changes passed CI/CD before

❌ DO NOT provide push command if:
- ANY quality check failed
- Uncertain about CI/CD requirements
- Making experimental changes
- First time working with this codebase
- User seems uncertain

Instead say:
"I recommend running the full CI/CD pipeline locally first to ensure 
the changes will pass. Once confirmed, you can push manually."
```

## GitHub MCP Server Integration

**If GitHub MCP Server is available**, use it for automated workflow monitoring.

### Workflow Validation After Push

```
After every git push (manual or auto):

1. Wait 5-10 seconds for workflows to trigger

2. Check workflow status via GitHub MCP:
   - List workflow runs for latest commit
   - Check status of each workflow

3. If workflows are RUNNING:
   ⏳ Report: "CI/CD workflows in progress..."
   ✅ Continue with other tasks
   ✅ Check again in next user interaction
   
4. If workflows COMPLETED:
   - All passed: ✅ Report success
   - Some failed: ❌ Fetch errors and fix

5. If workflows FAILED:
   a. Fetch complete error logs via GitHub MCP
   b. Display errors to user
   c. Analyze against AGENTS.md standards
   d. Propose specific fixes
   e. Implement fixes
   f. Run local quality checks
   g. Commit fixes
   h. Provide push command for retry
```

### Next Interaction Check

```
On every user message after a push:

if (github_mcp_available && last_push_timestamp) {
  // Check workflow status
  const status = await checkWorkflows();
  
  if (status.running) {
    console.log('⏳ CI/CD still running, will check later');
  } else if (status.failed) {
    console.log('❌ CI/CD failures detected!');
    await analyzeAndFixErrors(status.errors);
  } else {
    console.log('✅ All CI/CD workflows passed!');
  }
}
```

### Error Analysis Flow

```
When workflow fails:

1. Fetch error via GitHub MCP:
   - Workflow name
   - Job name  
   - Failed step
   - Error output
   - Full logs

2. Categorize error:
   - Test failure → Fix test or implementation
   - Lint error → Format/fix code style
   - Build error → Fix compilation issues
   - Type error → Fix type definitions
   - Coverage error → Add more tests

3. Fix following AGENTS.md:
   - Apply correct pattern from AGENTS.md
   - Add tests if needed
   - Verify locally before committing

4. Commit fix:
   git commit -m "fix: Resolve CI/CD failure - [specific issue]"

5. Provide push command:
   "Ready to retry. Run: git push origin main"

6. After next push:
   - Monitor again
   - Verify fix worked
```

### CI/CD Confidence Check

**Before suggesting push:**

```
Assess confidence in CI/CD success:

HIGH confidence (safe to push):
✅ All local checks passed
✅ Similar changes passed CI before
✅ No experimental changes
✅ Follows AGENTS.md exactly
✅ Comprehensive tests
✅ No unusual patterns

MEDIUM confidence (verify first):
⚠️ First time with this pattern
⚠️ Modified build configuration
⚠️ Changed dependencies
⚠️ Cross-platform concerns
→ Suggest: "Let's verify locally first"

LOW confidence (don't push yet):
❌ Experimental implementation
❌ Skipped some tests
❌ Uncertain about compatibility
❌ Modified CI/CD files
→ Say: "Let's run additional checks first"
```

## Troubleshooting

### Merge Conflicts

```bash
# View conflicts
git status

# Edit conflicted files (marked with <<<<<<<, =======, >>>>>>>)

# After resolving:
git add <resolved-files>
git commit -m "fix: Resolve merge conflicts"
```

### Accidental Commit

```bash
# Undo last commit, keep changes
git reset --soft HEAD~1

# Make corrections
# Re-commit properly
```

### Lost Commits

```bash
# View all actions
git reflog

# Recover lost commit
git checkout <commit-hash>
git checkout -b recovery-branch
```

<!-- GIT:END -->