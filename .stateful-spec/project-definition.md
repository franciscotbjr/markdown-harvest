# Project Definition — markdown-harvest

---

## Project Identity

- **Project Name:** markdown-harvest
- **Description:** A Rust crate to extract, clean, and convert web content from URLs found in text messages into clean Markdown format, designed for RAG (Retrieval-Augmented Generation) systems.
- **Project Type:** library (with binary CLI entrypoint)
- **Repository URL:** https://github.com/franciscotbjr/markdown-harvest
- **License:** MIT

## Technology Stack

### Language(s)

| Language | Version | Role |
|----------|---------|------|
| Rust | Edition 2024 | Primary |

### Framework(s)

| Framework | Version | Purpose |
|-----------|---------|---------|
| tokio | 1.49.0 | Async runtime (full features) |

### Key Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| reqwest | 0.13.1 | HTTP client (blocking + async, native-tls, http2) |
| scraper | 0.25.0 | HTML parsing with CSS selectors |
| html2md | 0.2.15 | HTML to Markdown conversion |
| regex | 1.12.2 | URL detection via regex |
| rand | 0.9.2 | User-agent rotation |
| once_cell | 1.21.3 | Lazy-static pre-compiled regex |
| futures | 0.3.31 | Async utilities |
| text-splitter | 0.29.3 | Optional: semantic Markdown chunking (`chunks` feature) |

### Build System & Package Manager

- **Package Manager:** cargo
- **Build Tool:** cargo
- **Test Runner:** cargo test (also cargo-nextest in CI)

## Repository Structure

```
project/
├── src/
│   ├── lib.rs                    # Crate root: module declarations + re-exports
│   ├── main.rs                   # Binary entrypoint: interactive CLI
│   ├── markdown_harvester.rs     # Main Facade struct: orchestration layer
│   ├── http_client.rs            # HTTP requests + URL extraction
│   ├── http_config.rs            # HttpConfig + HttpConfigBuilder (Builder pattern)
│   ├── http_regex.rs             # Pre-compiled URL regex (once_cell::sync::Lazy)
│   ├── content_processor.rs      # HTML cleaning + Markdown conversion
│   ├── patterns.rs               # Regex/CSS selector patterns for cleaning
│   └── user_agent.rs             # UserAgent enum with 12 platform variants
├── examples/
│   ├── sync_example.rs
│   ├── async_example.rs
│   ├── sync_chunks_example.rs
│   └── async_chunks_example.rs
├── Cargo.toml
├── Cargo.lock
├── README.md
├── CHANGELOG.md
├── DEV_NOTES.md
└── LICENSE
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| src/ | Library + binary source code (9 files) |
| examples/ | Usage examples (4 Rust files) |
| reps/ | Research and implementation planning documents |

## Code Conventions

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Files | snake_case | user_agent.rs |
| Functions/Methods | snake_case | get_hyperlinks_content |
| Types/Structs/Enums | PascalCase | MarkdownHarvester |
| Constants | SCREAMING_SNAKE_CASE | (not commonly used) |
| Modules | snake_case | content_processor |

### Code Style

- **Formatter:** rustfmt (default settings, no custom config)
- **Linter:** clippy (default settings, no custom config)
- **No rustfmt.toml, clippy.toml, or .editorconfig present**

### Patterns & Conventions

- **Architecture:** Facade pattern — MarkdownHarvester orchestrates HttpClient + ContentProcessor
- **API Design:** Constructor with required fields + fluent Builder pattern for HttpConfig
- **Async/Sync:** Both async and sync variants provided for all public APIs
- **Content Processing:** 3-priority semantic extraction (article/main tags > content selectors > body)
- **Testing:** Inline `#[cfg(test)] mod tests` in each source file, no separate tests/ directory
- **Error Handling:** Custom error types with manual From impls (not #[from])

## Testing

### Strategy

- **Unit Tests:** Co-located with source in `#[cfg(test)] mod tests` blocks
- **Integration Tests:** None in tests/ directory; integration tested via examples/
- **Test Framework:** cargo test (built-in `#[test]`)
- **Test Count:** 48 base tests, 62 with `chunks` feature
- **Note:** One test (`test_corrode_dev_article_extraction`) marked `#[ignore]` — requires real HTTP

### Test Naming Convention

`test_{what_is_being_tested}_{scenario}` — e.g., `test_get_hyperlinks_empty_text`, `test_builder_new`

## Quality Gates

```bash
# Tests (all features)
cargo test --all-features

# Build (all features)
cargo build --all-features

# Lint (no custom config)
cargo clippy --all-features -- -D warnings

# Format check
cargo fmt --check
```

## CI/CD

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| build.yml | Push to feature/bug/release branches, PR to main | Build + test |
| publish.yml | Push of v* tags | Audit + test (nextest) + publish to crates.io |
| binaries.yml | Manual dispatch (workflow_dispatch) | Cross-platform release binaries |

### Key CI Details

- Tests run with `cargo test --verbose --all-features` (build.yml) and `cargo nextest run --workspace --all-features` (publish.yml)
- No code coverage tool configured
- No linting/clippy step in CI pipelines (run locally before commit)

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Project overview, usage, feature flags, API reference |
| CHANGELOG.md | Version history (Keep a Changelog format) |
| DEV_NOTES.md | Development progress and notes |

### Documentation Style

- **Code Comments:** Rustdoc (`///` for public, `//` for internal)
- **No `no_run` attribute convention observed**

## Deployment

- **Target Environment:** crates.io
- **CI/CD:** GitHub Actions
- **Branch Strategy:** main + feature/bug/release/issue branches

## Constraints & Non-Negotiables

- No unsafe code without justification and documentation
- All public items must have rustdoc documentation
- Feature flags for optional functionality (`chunks` gating `text-splitter`)
- Dependencies must build on Linux, macOS, and Windows (cross-platform support required)
- No introducing dependencies, patterns, or tools not in this document without discussing first
