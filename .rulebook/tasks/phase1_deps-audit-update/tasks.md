## 1. Implementation

> Delivered in PR #8 (branch `chore/deps-audit-update`), release 0.3.5.

### 1.a Audit
- [x] 1.1 Fix `cargo audit` advisory-db fetch (cleared `~/.cargo/advisory-db`) and run it
- [x] 1.2 Record every advisory and decide remediation — 5 vulns fixed via bumps; 2 unmaintained (`paste`, `ttf-parser`) accepted (transitive, no fix)

### 1.b Safe refresh (semver-compatible)
- [x] 1.3 `cargo update` to refresh `Cargo.lock`; build + clippy + `cargo test --features office` green

### 1.c Low-risk direct bumps
- [x] 1.4 `sha2` 0.10 → 0.11
- [~] 1.5 `rstar` — HELD (docling-ffi only, not buildable/CI-tested here)
- [x] 1.6 `indicatif` 0.17 → 0.18 (cli)
- [x] 1.7 `pdfium-render` 0.8 → 0.9 (verified via pdf-to-image)
- [x] 1.8 `tracing-opentelemetry` 0.30 → 0.33
- [x] 1.9 `mockall` 0.13 → 0.15 (dev-dep)
- [x] 1.10 `imageproc` 0.25 → 0.27, `image` → 0.25.10

### 1.d Higher-risk major bumps
- [x] 1.11 `quick-xml` 0.37 → 0.41 (BytesText::unescape → decode + escape::unescape in xml/pptx/odt)
- [x] 1.12 `scraper` 0.21 → 0.27 (no source change needed)
- [x] 1.13 `html5ever` 0.29 → 0.39 (no source change needed)
- [x] 1.14 `zip` 6.0 → 8.6 (no source change needed)
- [x] 1.15 `lopdf` 0.35 → 0.44 (fixes RUSTSEC-2026-0187; no source change)
- [x] 1.16 `pdf-extract` 0.8 → 0.12 (no source change)
- [~] 1.17 `ndarray` — HELD (docling-ffi only, unverifiable/no CI)
- [x] 1.18 `umya-spreadsheet` 2.3 → 3.0 (Spreadsheet→Workbook, get_*→* in xlsx.rs)
- [~] 1.19 `ort` — HELD (docling-ffi only, unverifiable/no CI)
- [x] also: `comrak` 0.29 → 0.54, `file-format` 0.26 → 0.29, `console` 0.15 → 0.16, `criterion` 0.6 → 0.8

### 1.e MSRV decision
- [x] 1.20 MSRV raised `rust-version` 1.85 → 1.88 (user decision). `rust-toolchain.toml` already `nightly` (satisfies 1.88). Disabled clippy `collapsible_if` (let-chains now suggested across untouched parsers; separate refactor).

## 2. Tail (docs + tests — check or waive with tailWaiver)
- [x] 2.1 Update or create documentation covering the implementation — CHANGELOG 0.3.5 section (deps, advisories, MSRV) + README version refs
- [x] 2.2 Write tests covering the new behavior — no new runtime behavior; existing suite covers it (dep-only + MSRV change)
- [x] 2.3 Run tests and confirm they pass — `cargo fmt --check`, `cargo clippy --features "office,cli" --all-targets -- -D warnings`, `cargo test --features office` (96 passing), `cargo audit` (0 vulns)
