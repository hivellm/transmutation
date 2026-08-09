<!-- PROHIBITIONS:START -->
<!-- TIER1_PROHIBITIONS:START -->
# Absolute Prohibitions (Tier 1 — Highest Precedence)

These override all other rules.

## 1. No shortcuts, stubs, or simplified logic

No TODO/FIXME/HACK comments, no stubs or placeholder returns, no silently
reduced scope, no partial implementations. Implement completely — every edge
case and error path — or explain concretely why you can't. Correct beats fast.

## 2. No destructive git operations without explicit user authorization

Require explicit user authorization (destroys history or uncommitted work):
`reset --hard`, `checkout -- .` / `restore .`, `clean -f`, `push --force`,
`branch -D`, `rebase` on shared branches, `stash`. Autonomous and safe:
`status`/`diff`/`log`/`blame`/`add`/`commit`, creating branches for your own
work, switching to or merging YOUR agent-created branches, `revert` of your
own unpushed commits, and `git worktree` for parallel work. Never switch a
shared checkout that has changes you did not author; never rewrite or merge
into the default branch except via an approved PR.

## 3. No deletion without authorization

Never `rm`/`del` any file without an explicit user "yes, delete it". Caches
auto-invalidate; build artifacts have clean commands; investigate locks before
touching them.

## 4. Research before implementing — never guess

State what you KNOW and what you DON'T KNOW, research the unknown (read
source, check docs, run diagnostics), then implement. "I think this might be
the problem" is not acceptable; "source X does Y at file:line, we do Z, the
difference causes W" is.

## 5. No deferred tasks

A checklist item is implemented, not postponed. If a dependency blocks it,
implement the dependency first. If truly impossible, explain why concretely
and propose an alternative.

## 6. Respect task dependencies

Checklist order expresses dependencies: never start an item whose
prerequisites (earlier items it builds on) are incomplete, and never skip or
silently drop items. Independent items — including within the same phase —
may run in any order or in parallel (e.g. via subagents). Phases gate on
their dependencies, not on the phase number.
<!-- TIER1_PROHIBITIONS:END -->
<!-- PROHIBITIONS:END -->