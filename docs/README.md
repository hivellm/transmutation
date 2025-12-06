# Transmutation Documentation

**Version:** 0.1.0  
**Last Updated:** October 13, 2025

---

## Quick Links

| Document | Description | Audience |
|----------|-------------|----------|
| [SETUP.md](SETUP.md) | Installation and build instructions | Everyone |
| [CLI_GUIDE.md](CLI_GUIDE.md) | Command-line usage guide | Users |
| [MSI_BUILD.md](MSI_BUILD.md) | Windows MSI installer creation | Windows users |
| [MSI_DEPENDENCIES.md](MSI_DEPENDENCIES.md) | MSI dependency management strategies | Distributors |
| [DEPENDENCIES.md](DEPENDENCIES.md) | Dependency management guide | Developers |
| [FFI.md](FFI.md) | FFI integration (95%+ similarity) | Advanced users |
| [BENCHMARKS.md](BENCHMARKS.md) | Performance comparison | Decision makers |
| [ROADMAP.md](ROADMAP.md) | Future development plans | Contributors |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Technical design | Developers |

---

## Getting Started

### 1. Install and Build

Start with [SETUP.md](SETUP.md) for:
- System requirements
- Rust installation
- Build instructions (Pure Rust vs FFI)
- Docker builds
- Troubleshooting

**Quick start:**
```bash
cargo build --release --features pdf,cli
./target/release/transmutation convert document.pdf -o output.md
```

### 2. Learn the CLI

Read [CLI_GUIDE.md](CLI_GUIDE.md) for:
- Basic commands
- Conversion modes
- Options and flags
- Batch processing
- Examples

**Quick start:**
```bash
# Fast mode (default)
transmutation convert document.pdf -o output.md

# Precision mode (recommended)
transmutation convert document.pdf --precision -o output.md

# FFI mode (95%+ similarity)
transmutation convert document.pdf --ffi -o output.json
```

### 3. Understand Performance

Check [BENCHMARKS.md](BENCHMARKS.md) for:
- Speed comparison (98x faster than Docling)
- Quality metrics (80-95% similarity)
- Mode comparison (Fast vs Precision vs FFI)
- Resource usage
- Use case recommendations

**Summary:**
- **Fast:** 80% similarity, 250x faster
- **Precision:** 82% similarity, 94x faster ⭐ Recommended
- **FFI:** 95%+ similarity, returns JSON

---

## Advanced Topics

### FFI Integration

See [FFI.md](FFI.md) if you need:
- Maximum accuracy (95%+ similarity)
- Raw docling-parse output
- No Python dependency
- C++ library integration

**Trade-off:** FFI is slower but provides highest quality structural data.

### Architecture

Read [ARCHITECTURE.md](ARCHITECTURE.md) for:
- System design
- Code organization
- Engine architecture
- Extensibility

### Development

Check [STATUS.md](STATUS.md) and [ROADMAP.md](ROADMAP.md) for:
- What's implemented
- What's planned
- How to contribute

---

## Documentation Structure

```
docs/
├── README.md           ← You are here
├── SETUP.md            ← Installation & build
├── CLI_GUIDE.md        ← Usage guide
├── FFI.md              ← FFI integration
├── BENCHMARKS.md       ← Performance data
├── STATUS.md           ← Current status
├── ROADMAP.md          ← Future plans
└── ARCHITECTURE.md     ← Technical design
```

---

## Common Tasks

### Convert a PDF
```bash
transmutation convert document.pdf -o output.md
```
**→ See:** [CLI_GUIDE.md](CLI_GUIDE.md)

### Choose Best Mode
**→ See:** [BENCHMARKS.md](BENCHMARKS.md#use-case-recommendations)

### Build with FFI
```bash
./build_cpp.sh
cargo build --release --features pdf,cli,docling-ffi
```
**→ See:** [SETUP.md](SETUP.md#2-build-with-ffi-advanced---95-similarity)

### Troubleshoot Build Issues
**→ See:** [SETUP.md](SETUP.md#troubleshooting)

### Understand Performance
**→ See:** [BENCHMARKS.md](BENCHMARKS.md)

### Contribute
**→ See:** [STATUS.md](STATUS.md), [ROADMAP.md](ROADMAP.md)

---

## Quick Reference

### Conversion Modes

| Mode | Flag | Similarity | Speed | Output | Best For |
|------|------|------------|-------|--------|----------|
| Fast | (default) | 80% | 250x | Markdown | High-volume |
| Precision | `--precision` | 82% | 94x | Markdown | Production ⭐ |
| FFI | `--ffi` | 95%+ | ~50x | JSON | Research |

### System Requirements

- **Rust:** 1.85+ (Edition 2024)
- **OS:** Linux, macOS, Windows
- **FFI:** Linux/WSL only (optional)

### File Sizes

- **Binary:** 5MB (pure Rust)
- **FFI Library:** 7.4MB (optional)
- **Memory:** 50-100MB runtime

---

## Support

- **Issues:** https://github.com/hivellm/transmutation/issues
- **Discussions:** https://github.com/hivellm/transmutation/discussions
- **Main README:** [../README.md](../README.md)

---

**Built with ❤️ by the HiveLLM Team**

