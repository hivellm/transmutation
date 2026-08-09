# Proposal: phase1_deps-audit-update

## Why
Security audit and dependency freshness. The crate parses untrusted documents,
so known-vulnerable transitive crates are a real risk, and several direct
dependencies are several major versions behind (scraper, quick-xml, html5ever,
lopdf, pdf-extract, ndarray, zip, umya-spreadsheet). We want `cargo audit`
clean and every dependency on the newest version that still builds, tests
green, and stays within an acceptable MSRV.

## What Changes
- Run `cargo audit` against a working advisory DB; remediate any advisory
  (bump/replace the offending crate) or document an accepted risk with
  justification.
- Refresh `Cargo.lock` to latest semver-compatible versions.
- Bump direct dependencies in `Cargo.toml` to the latest versions, including
  major-version bumps, one group at a time, verifying build + clippy + tests
  after each. Back off any bump that cannot be made to build/pass without
  disproportionate churn, and record why.
- Decide and document the MSRV: several latest versions need Rust 1.87/1.88
  (current `rust-version = 1.85`). Either raise the MSRV (and CI toolchain) or
  cap those crates at the last 1.85-compatible version.

## Impact
- Affected specs: none (no spec files yet in `.rulebook/specs/`).
- Affected code: `Cargo.toml`, `Cargo.lock`, and any source touched by major-API
  changes — most likely `src/converters/html.rs` (scraper), `src/converters/odt.rs`
  and `src/converters/pptx.rs` (quick-xml), `src/engines/pdf_parser.rs` (lopdf/
  pdf-extract), `src/ml/*` (ndarray, behind `docling-ffi`), `src/converters/archive.rs`
  (zip), `src/converters/xlsx.rs` (umya-spreadsheet). CI workflows if MSRV changes.
- Breaking change: NO for the public API (internal dep/impl only); MSRV raise is
  a consumer-visible change if adopted.
- User benefit: no known-vulnerable dependencies, upstream bugfixes/perf, and a
  maintainable, current dependency tree.
