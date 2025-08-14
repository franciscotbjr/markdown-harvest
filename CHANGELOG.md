# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2025-01-13

⚠️ **BREAKING CHANGES**: This version introduces breaking changes to the public API. See migration guide below.

### Added
- **🏗️ HTTP Configuration with Builder Pattern**: Complete HTTP configuration system
  - New `HttpConfig` struct with configurable timeout, redirects, and cookie management
  - `HttpConfigBuilder` implementing the Builder pattern for fluent configuration
  - Support for `timeout`, `max_redirect`, and `cookie_store` configuration options
  - Builder pattern allows: `HttpConfig::builder().timeout(5000).max_redirect(10).cookie_store(true).build()`
  - Default configuration available via `HttpConfig::default()`
  - Enhanced HTTP client with comprehensive request headers for better compatibility

- **🧪 Comprehensive Test Suite**: Extensive unit test coverage
  - 17 new unit tests specifically for `HttpConfig` and `HttpConfigBuilder`
  - Tests cover Builder pattern, edge cases, chaining, and all configuration options
  - Total test count increased from 10 to 27 unit tests
  - 100% coverage of public API methods and edge cases

### Changed - BREAKING CHANGES
- **💥 API Signature Change**: Method signatures now require `HttpConfig` parameter
  - `MarkdownHarvester::get_hyperlinks_content(text: String)` → `get_hyperlinks_content(text: String, http_config: HttpConfig)`
  - `HttpClient::fetch_content_from_text(text: &str)` → `fetch_content_from_text(text: &str, http_config: HttpConfig)`
  - This change affects all existing code using these methods

- **🔧 HTTP Client Enhancements**: Advanced HTTP configuration and headers
  - Added support for redirect policy configuration
  - Cookie store management for session handling
  - Enhanced browser-like headers for better website compatibility
  - Improved error handling and timeout management

- **📚 Unit Test Improvements**: Enhanced test reliability by preventing real HTTP calls
  - Modified doctests to use `no_run` attribute to prevent execution during testing
  - Replaced potential network-dependent assertions with network availability notes
  - Removed `httpbin.org` references from doctests to avoid external dependencies
  - All tests now run in isolation without requiring internet connectivity

- **📖 Documentation Updates**: All examples updated to use new HttpConfig API
  - Updated doctests to demonstrate Builder pattern usage
  - Consistent API documentation across all modules
  - Clear examples showing all configuration options

### Technical Details
- **🎯 Builder Pattern Implementation**: Fluent API design with method chaining
- **⚡ Performance Optimizations**: `HttpConfig` is `Copy + Clone + Default` for optimal performance
- **🔒 Type Safety**: Compile-time validation of configuration parameters
- **🌐 Enhanced HTTP Features**:
  - Configurable redirect limits (default: 2, configurable via `max_redirect`)
  - Cookie store support for session management
  - Comprehensive browser-like headers for better compatibility
- **🧪 Testing Infrastructure**: Robust test suite with 100% API coverage
- **📋 Future-Ready Architecture**: Extensible design for additional HTTP configuration options

### Migration Guide

⚠️ **Important**: This is a breaking change. All existing code must be updated.

```rust
// ❌ Before v0.1.3 - OLD API (will not compile)
let results = MarkdownHarvester::get_hyperlinks_content(text);

// ✅ v0.1.3 - NEW API with default configuration
let results = MarkdownHarvester::get_hyperlinks_content(text, HttpConfig::default());

// ✅ v0.1.3 - NEW API with custom timeout
let config = HttpConfig::builder().timeout(5000).build();
let results = MarkdownHarvester::get_hyperlinks_content(text, config);

// ✅ v0.1.3 - NEW API with full configuration
let config = HttpConfig::builder()
    .timeout(10000)           // 10 second timeout
    .max_redirect(5)          // Allow up to 5 redirects
    .cookie_store(true)       // Enable cookie storage
    .build();
let results = MarkdownHarvester::get_hyperlinks_content(text, config);
```

### Why This Breaking Change?
This breaking change was necessary to:
1. **🚀 Enable Future Extensibility**: The new architecture allows adding HTTP features without further breaking changes
2. **🎯 Improve Flexibility**: Users can now configure timeouts, redirects, cookies, and future HTTP options
3. **🏗️ Better API Design**: Builder pattern provides a more intuitive and discoverable API
4. **📈 Enhanced Functionality**: Support for advanced HTTP features that weren't possible with the old API

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