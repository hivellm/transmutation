<!-- QUALITY_ENFORCEMENT:START -->
# Quality Enforcement Rules

**CRITICAL**: These rules are NON-NEGOTIABLE and MUST be followed without exception.

## Absolute Prohibitions

### Test Bypassing - STRICTLY FORBIDDEN
- NEVER use .skip(), .only(), or .todo() to bypass failing tests
- NEVER comment out failing tests
- NEVER use @ts-ignore, @ts-expect-error, or similar to hide test errors
- NEVER mock/stub functionality just to make tests pass without fixing root cause
- FIX the actual problem causing test failures

### Git Hook Bypassing - STRICTLY FORBIDDEN  
- NEVER use --no-verify flag on git commit
- NEVER use --no-verify flag on git push
- NEVER disable or skip pre-commit hooks
- NEVER disable or skip pre-push hooks
- FIX the issues that hooks are detecting

### Test Implementation - STRICTLY FORBIDDEN
- NEVER create boilerplate tests that don't actually test behavior
- NEVER write tests that always pass regardless of implementation
- NEVER write tests without assertions
- NEVER mock everything to avoid testing real behavior
- WRITE meaningful tests that verify actual functionality

### Problem Solving Approach - REQUIRED
- DO NOT seek the simplest bypass or workaround
- DO NOT be creative with shortcuts that compromise quality
- DO solve problems properly following best practices
- DO use proven, established solutions from decades of experience
- DO fix root causes, not symptoms

## Enforcement

These rules apply to ALL implementations:
- Bug fixes
- New features  
- Refactoring
- Documentation changes
- Any code modifications

**Violation = Implementation Rejected**

<!-- QUALITY_ENFORCEMENT:END -->