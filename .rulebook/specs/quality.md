<!-- QUALITY:START -->
<!-- QUALITY_ENFORCEMENT:START -->
# Quality Enforcement Rules

Non-negotiable. A violation means the implementation is rejected.

## Forbidden

- Bypassing tests: `.skip()` / `.only()` / `.todo()`, commenting out failing
  tests, assertion-free boilerplate tests, mocking everything just to pass.
- Hiding errors: `@ts-ignore` / `@ts-expect-error` (or language equivalents)
  without a one-line justification.
- Bypassing hooks: `--no-verify` on commit or push, disabling pre-commit or
  pre-push hooks. Fix what the hook is flagging.
- Workarounds instead of root causes: no creative shortcuts that compromise
  quality — fix the actual problem.
- Temp-file litter: temporary scripts/files live in `/scripts` only and are
  deleted immediately after use; never leave test artifacts, logs, or debug
  files anywhere in the repo.

## Required

- Fix root causes, not symptoms.
- Write meaningful tests that verify real behavior.
- Per commit: type-check + lint + the tests covering the change. Per
  push/PR/task archive: the full suite (type-check → lint → all tests).
<!-- QUALITY_ENFORCEMENT:END -->
<!-- QUALITY:END -->