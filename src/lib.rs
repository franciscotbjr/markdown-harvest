//! # markdown-harvest
//!
//! A Rust crate designed to extract, clean, and convert web content from URLs found in text messages into clean Markdown format.
//! Originally created as an auxiliary component for Retrieval-Augmented Generation (RAG) solutions to process URLs submitted by users.
//!
//! ## Overview
//!
//! This crate provides functionality to:
//! - Extract URLs from text input
//! - Fetch web content from those URLs
//! - Clean and convert HTML content to readable Markdown format
//! - Remove unwanted elements like navigation, advertisements, and scripts
//!
//! ## Quick Start
//!
//! ```rust
//! use markdown_harvest::MarkdownHarvester;
//!
//! let text = "Check out this article: https://example.com/article";
//! let results = MarkdownHarvester::get_hyperlinks_content(text.to_string());
//!
//! for (url, markdown_content) in results {
//!     println!("URL: {}", url);
//!     println!("Content: {}", markdown_content);
//! }
//! ```
//!
//! ## Features
//!
//! - **URL Detection**: Automatically extracts HTTP/HTTPS URLs from text
//! - **Content Extraction**: Fetches and processes web content
//! - **HTML Cleaning**: Removes scripts, styles, navigation, and advertisements  
//! - **Markdown Conversion**: Converts cleaned HTML to readable Markdown
//! - **User Agent Rotation**: Uses random user agents to avoid blocking
//!
//! ## Main Components
//!
//! - [`MarkdownHarvester`]: The main struct for processing URLs and extracting content
//! - [`UserAgent`]: Enum providing various browser user agent strings
//! - Pattern functions: Helper functions that define cleaning patterns for HTML processing

mod content_processor;
mod http_client;
mod markdown_harvester;
mod patterns;
mod user_agent;

pub use content_processor::ContentProcessor;
pub use http_client::HttpClient;
pub use markdown_harvester::MarkdownHarvester;
pub use patterns::{
    additional_cleanup, content_selectors, media_elements, text_selectors, unwanted_elements,
    unwanted_text_patterns,
};
pub use user_agent::UserAgent;
