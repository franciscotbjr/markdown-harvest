# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2025-01-13

### Changed
- **Unit Test Improvements**: Enhanced test reliability by preventing real HTTP calls
  - Modified doctests to use `no_run` attribute to prevent execution during testing
  - Replaced potential network-dependent assertions with network availability notes
  - Removed `httpbin.org` references from doctests to avoid external dependencies
  - All tests now run in isolation without requiring internet connectivity

### Technical Details
- Doctests in `lib.rs` and `markdown_harvester.rs` now use `rust,no_run` to prevent HTTP execution
- Improved test performance by eliminating network timeouts
- Enhanced test reliability for CI/CD environments without internet access

## [0.1.2] - 2025-01-11

### Added
- **HttpClient** component: Responsible for HTTP requests and URL processing
  - Extracts URLs from text using regex patterns
  - Handles HTTP requests with random user agent rotation
  - Can be used independently for HTTP-based content retrieval
- **ContentProcessor** component: Responsible for HTML cleaning and Markdown conversion
  - Parses HTML and extracts content from body elements
  - Removes unwanted elements (scripts, ads, navigation)
  - Converts cleaned HTML to Markdown format
  - Can be used independently for HTML-to-Markdown conversion
- **MarkdownHarvester** component: Provides unified interface combining both components
  - Encapsulates the complexity of coordinating HTTP and content processing
  - Implements the Facade pattern for clean API access
- Comprehensive unit tests for all new components
- Individual component testing for isolated functionality
- Integration tests for component interaction

### Changed
- Separated responsibilities into independent components
  - `MarkdownHarvester` now acts as a facade to maintain backward compatibility
  - Internal implementation now uses `HttpClient` and `ContentProcessor` components
  - All existing public APIs remain unchanged for backward compatibility

### Technical Details
- Implemented separation of concerns following the Facade pattern
- HTTP logic isolated in `HttpClient` component
- Content processing logic isolated in `ContentProcessor` component

### Migration
No migration required. This version maintains full backward compatibility with v0.1.1.
The `MarkdownHarvester::get_hyperlinks_content()` function continues to work exactly as before.

For new projects, you can now use the individual components:
```rust
use markdown_harvest::{HttpClient, ContentProcessor, MarkdownHarvester};

// Use components individually
let http_client = HttpClient::new();
let content_processor = ContentProcessor::new();

// Or use the facade
let results = MarkdownHarvester::get_hyperlinks_content(text);
```

## [0.1.1] - Previous Release
- Initial functionality for URL extraction and content processing
- HTML cleaning and Markdown conversion
- Multi-platform user agent support
- Interactive CLI mode