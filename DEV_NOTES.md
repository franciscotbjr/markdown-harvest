# Development Notes

## Version 0.1.6 - In Progress (2026-01-03)

### Dependency Updates - Phase 1 ✅ COMPLETED

Successfully updated core dependencies to latest stable versions:

#### Updated Dependencies
- **reqwest**: `0.12.23` → `0.13.1`
- **scraper**: `0.23.1` → `0.25.0`
- **text-splitter**: `0.28` → `0.29.3`

#### Technical Implementation Details

**reqwest Configuration**
The update to reqwest 0.13.1 introduced `aws-lc-rs` as the default TLS backend, which requires cmake and NASM build tools. To maintain build simplicity and cross-platform compatibility, the configuration was changed to use `native-tls`:

```toml
reqwest = {
    version = "0.13.1",
    default-features = false,
    features = ["blocking", "json", "cookies", "native-tls", "http2"]
}
```

**Benefits of this approach:**
- ✅ No cmake or NASM build dependencies required
- ✅ Uses OS-native TLS implementation (SChannel on Windows, SecureTransport on macOS, OpenSSL on Linux)
- ✅ Lighter build process
- ✅ Maintains all existing functionality
- ✅ Better compatibility across different development environments

**Test Results:**
- Without chunks feature: **41/41 tests PASSED** ✅
- With chunks feature: **55/55 tests PASSED** ✅
- No breaking changes
- Full backward compatibility maintained

### Next Phase - Issue #40 Fix

The next phase will implement the fix for Issue #40, which involves improving content extraction for articles nested inside `<article>` tags using the `readability` library.

**Planned Implementation:**
- Add `readability` dependency to Cargo.toml
- Modify ContentProcessor to use readability for smart content extraction
- Implement fallback to current body extraction method
- Add tests for the specific use case (corrode.dev blog article)

---

## Previous Development Notes

### Main.rs Runtime Issue - FIXED (2025-09-11)

**Problem:**
When running the synchronous processing option in main.rs, the application would panic with:
```
Cannot drop a runtime in a context where blocking is not allowed.
```

**Root Cause:**
The synchronous example was inadvertently creating a Tokio runtime in a blocking context, causing a panic when the runtime was dropped.

**Solution:**
Fixed the runtime management in main.rs to properly handle both synchronous and asynchronous execution contexts.

**Status:** ✅ RESOLVED
