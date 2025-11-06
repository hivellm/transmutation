<!-- CONTEXT7:START -->
# Context7 Instructions

**CRITICAL**: Use MCP Context7 to access up-to-date library documentation before adding dependencies.

## Core Functions

### 1. resolve-library-id
Resolve a package name to Context7-compatible library ID:
```
resolve-library-id("tokio") → "/tokio-rs/tokio"
resolve-library-id("react") → "/facebook/react"
```

### 2. get-library-docs
Fetch documentation with optional topic filter:
```
get-library-docs("/tokio-rs/tokio", topic="async")
```

## Workflow

**Before adding ANY dependency:**
```
1. resolve-library-id(library_name)
2. get-library-docs(library_id) 
3. Check latest version and security
4. Add dependency with correct version
5. Document choice in commit
```

**Before updating major version:**
```
1. get-library-docs for current version
2. get-library-docs for target version
3. Review breaking changes
4. Plan migration
```

## Best Practices

✅ **DO:**
- Always check Context7 before adding dependencies
- Use topic parameter for specific info
- Document version choices
- Review security advisories

❌ **DON'T:**
- Add dependencies without checking latest version
- Skip migration guides on major updates
- Ignore security warnings

<!-- CONTEXT7:END -->