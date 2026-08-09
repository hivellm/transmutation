<!-- RULEBOOK:START v7.0.0 — DO NOT EDIT BY HAND. Regenerated on `rulebook update`.
     Put project-specific content in AGENTS.override.md or CLAUDE.local.md.
     Anything outside the RULEBOOK:START/END sentinels is preserved across updates. -->

# CLAUDE.md

Managed by [@hivehub/rulebook](https://github.com/hivellm/rulebook) — few rules, all deliberate.

## Project-specific overrides (user-owned, survives `rulebook update`, wins on conflict)
<!-- @AGENTS.override.md (skipped — target file not present) -->

## Commands
- Before each commit: type-check + lint + the tests covering what changed.
- Before push / PR / task archive: the FULL quality gate (type-check → lint →
  full test suite), all green. Never bypass hooks — what the project wired
  into pre-commit/pre-push is the floor.
- Diagnostic-first: run the type-checker before the test suite; it is the faster signal.

## Values
1. Complete implementations — no stubs, no TODO markers left behind; finish, or say concretely why you can't.
2. Root causes, not workarounds — diagnose before changing code; never guess at bug causes.
3. Surgical diffs — touch only what the task needs; match existing style.
4. Simplicity first — the least code that solves the problem; no unrequested abstractions or features.
5. Fix forward — never discard uncommitted work.
6. State assumptions — if interpretations diverge, say so instead of picking silently.

## Git safety (requires explicit user authorization)
`reset --hard` · `checkout -- .` / `restore .` · `clean -f` · `push --force` ·
`rebase` on shared branches · `stash` · `branch -D` · switching a shared checkout
with changes you did not author. Yours autonomously: status/diff/log/add/commit,
branches you create (create/switch/merge), `git worktree`, PRs via `gh`.

## Orchestration
Subagents, parallel dispatch, and teams are your call — fan out freely when work is
parallel or context-heavy; work directly when it isn't. Rulebook never blocks or
mandates orchestration.

## Rulebook (on demand — no ceremony for small fixes)
- Multi-session or multi-phase work: track via the `rulebook` MCP (`rulebook_task`).
  Checklist order = dependencies; independent items may run in parallel.
- Optional session context: `rulebook_session`. Learned something non-obvious?
  `rulebook_memory`.
- Project specs live in `.rulebook/specs/` — read a spec when the work touches its area.
- Analyses live in `docs/analysis/<slug>/` — numbered files, one theme per file.
- Long session? `/compact <focus>` at a task boundary (~60% context). After
  `rulebook_task {action:"archive"}`, `/clear` is free — state lives in `.rulebook/`.

<!-- RULEBOOK:END -->
