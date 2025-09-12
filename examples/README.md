# Markdown Harvest Examples

This directory contains executable examples demonstrating all API functions of the Markdown Harvest crate.

## Available Examples

### 1. Synchronous Processing
**File:** `sync_example.rs`  
**Function:** `get_hyperlinks_content`  
**Run with:** `cargo run --example sync_example`

Demonstrates basic synchronous URL processing where URLs are processed sequentially and all results are returned together.

### 2. Asynchronous Processing
**File:** `async_example.rs`  
**Function:** `get_hyperlinks_content_async`  
**Run with:** `cargo run --example async_example`

Demonstrates asynchronous URL processing with parallel execution and real-time result streaming.

### 3. Synchronous Chunking (RAG)
**File:** `sync_chunks_example.rs`  
**Function:** `get_hyperlinks_content_as_chunks`  
**Run with:** `cargo run --example sync_chunks_example --features chunks`

Demonstrates synchronous processing with semantic chunking for RAG systems, including configurable chunk sizes and overlap.

### 4. Asynchronous Chunking (RAG)
**File:** `async_chunks_example.rs`  
**Function:** `get_hyperlinks_content_as_chunks_async`  
**Run with:** `cargo run --example async_chunks_example --features chunks`

Demonstrates asynchronous processing with semantic chunking, combining parallel execution with real-time chunk streaming.

## Requirements

- For basic examples: No additional features required
- For chunking examples: Requires `--features chunks` flag
- All examples require an internet connection to fetch URL content

## Usage Tips

1. **Test URLs:** Try URLs like:
   - `https://example.com`
   - `https://httpbin.org/json`
   - `https://www.rust-lang.org`

2. **Input Format:** Enter text containing URLs, such as:
   ```
   Check out https://example.com and https://httpbin.org/json
   ```

3. **Chunk Configuration:** 
   - Recommended chunk sizes: 1000-2000 characters
   - Overlap options: 10-20% for better context preservation
   - Adjust based on your embedding model requirements

4. **Performance Comparison:** Run both sync and async examples with the same URLs to compare performance differences.

## Educational Purpose

These examples are designed for:
- Learning how to use each API function
- Understanding the differences between sync/async processing
- Exploring chunking options for RAG systems
- Performance testing and comparison

All examples are excluded from the main crate build and are intended for educational and testing purposes only.