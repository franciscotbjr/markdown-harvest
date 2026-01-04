use crate::patterns::{
    additional_cleanup, content_selectors, media_elements, text_selectors, unwanted_elements,
    unwanted_text_patterns,
};

use regex::Regex;
use scraper::{Html, Selector};

/// Component responsible for HTML cleaning and Markdown conversion.
///
/// `ContentProcessor` handles all aspects of content processing including HTML parsing,
/// content extraction, cleaning unwanted elements, and converting to Markdown format.
/// This component reuses the original functions from MarkdownHarvester to maintain
/// compatibility and behavior.
#[derive(Default, Clone)]
pub struct ContentProcessor {}

impl ContentProcessor {
    /// Creates a new ContentProcessor instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Converts HTML content to clean Markdown format.
    pub fn html_to_markdown(&self, html: &str) -> String {
        extract_and_clean_content(html)
    }
}

/// Extracts the main content from HTML using a priority-based strategy.
///
/// Priority order:
/// 1. Semantic HTML5 tags (article, main, [role='main'])
/// 2. Content-specific class selectors (.content, .article, .post, .entry)
/// 3. Fallback to body tag
///
/// # Arguments
/// * `document` - Parsed HTML document
///
/// # Returns
/// * String containing the extracted HTML content
fn extract_main_content(document: &Html) -> String {
    // Priority 1: Semantic HTML5 tags
    if let Some(content) = try_semantic_tags(document) {
        return content;
    }

    // Priority 2: Content selectors
    if let Some(content) = try_content_selectors_direct(document) {
        return content;
    }

    // Priority 3: Fallback to body
    fallback_to_body_tag(document)
}

/// Attempts to extract content from semantic HTML5 tags.
///
/// Tries in order: <article>, <main>, [role='main']
/// Returns the first match found.
///
/// # Arguments
/// * `document` - Parsed HTML document
///
/// # Returns
/// * Some(String) if a semantic tag is found, None otherwise
fn try_semantic_tags(document: &Html) -> Option<String> {
    // Define semantic selectors in priority order
    let semantic_selectors = ["article", "main", "[role='main']"];

    for selector_str in semantic_selectors.iter() {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                // Found semantic tag, return its HTML
                return Some(element.html());
            }
        }
    }

    None // No semantic tags found
}

/// Attempts to extract content using content-specific class selectors.
///
/// Tries: .content, .article, .post, .entry
/// Returns the first match found.
///
/// # Arguments
/// * `document` - Parsed HTML document
///
/// # Returns
/// * Some(String) if a content selector matches, None otherwise
fn try_content_selectors_direct(document: &Html) -> Option<String> {
    let class_selectors = [".content", ".article", ".post", ".entry"];

    for selector_str in class_selectors.iter() {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                return Some(element.html());
            }
        }
    }

    None
}

/// Fallback extraction using the body tag.
///
/// Returns the entire body content as last resort.
///
/// # Arguments
/// * `document` - Parsed HTML document
///
/// # Returns
/// * String containing body HTML, or empty string if no body tag found
fn fallback_to_body_tag(document: &Html) -> String {
    let body_selector = Selector::parse("body").unwrap();

    match document.select(&body_selector).next() {
        Some(body_element) => body_element.html(),
        None => String::new(), // No body tag found
    }
}

fn extract_and_clean_content(html: &str) -> String {
    // Step 1: Parse document
    let document = Html::parse_document(html);

    // Step 2: Smart content extraction (NEW!)
    let extracted_html = extract_main_content(&document);

    // Check if extraction was successful
    if extracted_html.is_empty() {
        return String::new();
    }

    // Step 3: Clean the extracted content
    let relevant_html = clear_content(extracted_html);

    // Step 4: Convert to Markdown
    let markdown_content = html2md::parse_html(&relevant_html);

    // Step 5: Final cleanup
    final_clean_from_markdown(markdown_content)
}

// DEPRECATED: Kept for backwards compatibility during transition
// Will be removed in future version
#[allow(dead_code)]
fn extract_and_clean_body(html: &str) -> String {
    // Step 1: Extract only the body content from the HTML
    let document = Html::parse_document(html);
    let body_selector = Selector::parse("body").unwrap();

    let body_html = match document.select(&body_selector).next() {
        Some(body_element) => body_element.html(),
        None => return String::new(), // Return empty if no body found
    };

    // Step 2: Clean the body content by removing unwanted elements
    let relevant_html = clear_content(body_html);

    // Step 3: Convert the cleaned HTML to Markdown
    let markdown_content = html2md::parse_html(&relevant_html);

    // Step 4: Final cleanup
    // Remove unwanted elements while preserving Markdown structure
    final_clean_from_markdown(markdown_content)
}

fn clear_content(content_html: String) -> String {
    let mut cleaned_body = content_html;

    // Remove script blocks
    let script_regex = Regex::new(r"(?i)<script[^>]*>[\s\S]*?</script>").unwrap();
    cleaned_body = script_regex.replace_all(&cleaned_body, "").to_string();

    // Remove style blocks
    let style_regex = Regex::new(r"(?i)<style[^>]*>[\s\S]*?</style>").unwrap();
    cleaned_body = style_regex.replace_all(&cleaned_body, "").to_string();

    // Remove images, iframes, and other non-textual elements
    for pattern in media_elements().iter() {
        let regex = Regex::new(pattern).unwrap();
        cleaned_body = regex.replace_all(&cleaned_body, "").to_string();
    }

    // Remove navigation, header, footer, sidebar and advertising elements
    for pattern in unwanted_elements().iter() {
        let regex = Regex::new(pattern).unwrap();
        cleaned_body = regex.replace_all(&cleaned_body, "").to_string();
    }

    // Parse the cleaned body HTML and use scraper to extract only text content elements
    let cleaned_document =
        Html::parse_document(&format!("<html><body>{}</body></html>", cleaned_body));

    // Select only content-relevant elements and extract their inner HTML

    let mut relevant_html = String::new();
    let mut found_main_content = false;

    // First try to find main content containers
    for selector_str in content_selectors().iter() {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in cleaned_document.select(&selector) {
                relevant_html.push_str(&element.html());
                relevant_html.push('\n');
                found_main_content = true;
            }
        }
    }

    // If no main content containers found, extract individual text elements
    if !found_main_content {
        for selector_str in text_selectors().iter() {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in cleaned_document.select(&selector) {
                    relevant_html.push_str(&element.html());
                    relevant_html.push('\n');
                }
            }
        }
    }

    // If still no content found, fallback to the entire cleaned body
    if relevant_html.trim().is_empty() {
        relevant_html = cleaned_body;
    }

    // Additional cleanup before markdown conversion - remove remaining unwanted elements
    for pattern in additional_cleanup().iter() {
        let regex = Regex::new(pattern).unwrap();
        relevant_html = regex.replace_all(&relevant_html, "").to_string();
    }

    return relevant_html;
}

fn final_clean_from_markdown(markdown_content: String) -> String {
    let mut result = markdown_content;

    // Remove any remaining HTML tags that might have been missed
    let html_tag_regex = Regex::new(r"<[^>]+>").unwrap();
    result = html_tag_regex.replace_all(&result, "").to_string();

    // Remove Markdown links [text](url) and keep only the text part
    let link_regex = Regex::new(r"\[([^\]]+)\]\([^)]+\)").unwrap();
    result = link_regex.replace_all(&result, "$1").to_string();

    // Remove standalone URLs that might remain
    let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
    result = url_regex.replace_all(&result, "").to_string();

    // Keep Markdown formatting but clean up problematic patterns
    // Remove code blocks (usually not relevant content)
    let code_block_regex = Regex::new(r"```[\s\S]*?```").unwrap();
    result = code_block_regex.replace_all(&result, "").to_string();

    // Remove excessive whitespace and normalize line breaks
    let space_regex = Regex::new(r"[ \t]+").unwrap();
    result = space_regex.replace_all(&result, " ").to_string();

    let newline_regex = Regex::new(r"\n{3,}").unwrap();
    result = newline_regex.replace_all(&result, "\n\n").to_string();

    // Remove common advertising/navigation text patterns but preserve line structure
    for pattern in unwanted_text_patterns().iter() {
        let regex = Regex::new(pattern).unwrap();
        result = regex.replace_all(&result, "").to_string();
    }

    // Clean up empty lines and extra spacing
    let cleanup_regex = Regex::new(r"\n\s*\n\s*\n").unwrap();
    result = cleanup_regex.replace_all(&result, "\n\n").to_string();

    // Remove lines that are likely metadata or navigation while preserving markdown structure
    result = remove_lines_metadata_or_navigation(result.lines().collect()).join("\n");

    // Clean up excessive empty lines but preserve paragraph structure
    let excessive_newlines_regex = Regex::new(r"\n{4,}").unwrap();
    result = excessive_newlines_regex
        .replace_all(&result, "\n\n\n")
        .to_string();

    result.trim().to_string()
}

fn remove_lines_metadata_or_navigation(lines: Vec<&str>) -> Vec<&str> {
    lines
        .into_iter()
        .filter(|line| {
            let trimmed = line.trim();

            // Always keep lines that start with markdown headers
            if trimmed.starts_with('#') || trimmed.starts_with("##") {
                return true;
            }

            // Filter out very short lines that aren't meaningful
            if trimmed.is_empty() || trimmed.len() < 5 {
                return trimmed.is_empty(); // Keep empty lines for spacing
            }

            // Keep lines with meaningful content (relaxed filtering for single words)
            let lower = trimmed.to_lowercase();

            // Only filter out single words if they are likely navigation/metadata terms
            if !trimmed.contains(' ') {
                let navigation_terms = [
                    "home",
                    "about",
                    "contact",
                    "menu",
                    "search",
                    "login",
                    "register",
                    "subscribe",
                    "share",
                    "follow",
                    "back",
                    "next",
                    "prev",
                    "more",
                    "advertisement",
                    "ads",
                    "sponsored",
                    "cookie",
                    "privacy",
                    "terms",
                ];
                if navigation_terms.iter().any(|&term| lower == term) {
                    return false;
                }
            }

            // Skip obvious metadata/navigation patterns
            if lower.starts_with("http")
                || lower.contains("@")
                || lower == "menu"
                || lower == "navigation"
                || lower == "nav"
                || lower == "footer"
                || lower == "header"
                || lower == "sidebar"
            {
                return false;
            }

            // Filter out extremely short lines (less than 2 characters) that aren't meaningful
            if trimmed.len() < 2 {
                return false;
            }

            // Keep everything else, including single words that could be content
            true
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let processor = ContentProcessor::new();
        assert_eq!(std::mem::size_of_val(&processor), 0);
    }

    #[test]
    fn test_extract_and_clean_body_with_empty_html() {
        let empty_html = "";
        let result = extract_and_clean_body(empty_html);
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_and_clean_body_with_no_body() {
        let html_without_body = "<html><head><title>Test</title></head></html>";
        let result = extract_and_clean_body(html_without_body);
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_and_clean_body_with_simple_content() {
        let simple_html =
            "<html><body><h1>Test Title</h1><p>Test paragraph content.</p></body></html>";
        let result = extract_and_clean_body(simple_html);

        // Should contain the content without HTML tags
        assert!(result.contains("Test Title"));
        assert!(result.contains("Test paragraph content"));
        assert!(!result.contains("<h1>"));
        assert!(!result.contains("<p>"));
    }

    #[test]
    fn test_html_to_markdown() {
        let processor = ContentProcessor::new();
        let html = "<html><body><h1>Title</h1><p>Content</p></body></html>";
        let result = processor.html_to_markdown(html);

        assert!(result.contains("Title"));
        assert!(result.contains("Content"));
        assert!(!result.contains("<html>"));
        assert!(!result.contains("<body>"));
    }

    // ============================================================================
    // NEW TESTS FOR SEMANTIC HTML5 TAG EXTRACTION (Issue #40)
    // ============================================================================
    // These tests validate the smart article extraction algorithm that prioritizes
    // semantic HTML5 tags (article, main) over body tag extraction.
    // Expected to FAIL until implementation is complete.

    #[test]
    fn test_extract_article_tag_priority() {
        let html = r#"
            <html>
            <body>
                <nav>Navigation menu</nav>
                <article>
                    <h1>Article Title</h1>
                    <p>Article main content here.</p>
                </article>
                <footer>Footer content</footer>
            </body>
            </html>
        "#;

        let result = extract_and_clean_body(html);

        // Should extract article content
        assert!(
            result.contains("Article Title"),
            "Expected to find 'Article Title' in extracted content"
        );
        assert!(
            result.contains("Article main content"),
            "Expected to find 'Article main content' in extracted content"
        );

        // Should NOT contain navigation or footer
        assert!(
            !result.contains("Navigation menu"),
            "Should not contain navigation content"
        );
        assert!(
            !result.contains("Footer content"),
            "Should not contain footer content"
        );
    }

    #[test]
    fn test_extract_main_tag() {
        let html = r#"
            <html>
            <body>
                <header>Site header</header>
                <main>
                    <h1>Main Content Title</h1>
                    <p>This is the main content area.</p>
                </main>
                <aside>Sidebar content</aside>
            </body>
            </html>
        "#;

        let result = extract_and_clean_body(html);

        // Should extract main content
        assert!(
            result.contains("Main Content Title"),
            "Expected to find 'Main Content Title' in extracted content"
        );
        assert!(
            result.contains("main content area"),
            "Expected to find main content text"
        );

        // Should NOT contain header or sidebar
        assert!(
            !result.contains("Site header"),
            "Should not contain header content"
        );
        assert!(
            !result.contains("Sidebar content"),
            "Should not contain sidebar content"
        );
    }

    #[test]
    fn test_fallback_to_body_when_no_semantic_tags() {
        let html = r#"
            <html>
            <body>
                <div class="wrapper">
                    <h1>Legacy Page Title</h1>
                    <p>Content without semantic tags.</p>
                </div>
            </body>
            </html>
        "#;

        let result = extract_and_clean_body(html);

        // Should fallback to body extraction and still work
        assert!(
            result.contains("Legacy Page Title"),
            "Expected to find title in extracted content"
        );
        assert!(
            result.contains("Content without semantic tags"),
            "Expected to find content text"
        );
    }

    #[test]
    fn test_article_takes_priority_over_body_clutter() {
        let html = r#"
            <html>
            <body>
                <header>
                    <nav>
                        <a href="/">Home</a>
                        <a href="/about">About</a>
                    </nav>
                </header>
                <div class="sidebar">
                    <h3>Related Links</h3>
                    <ul>
                        <li><a href="/link1">Link 1</a></li>
                        <li><a href="/link2">Link 2</a></li>
                    </ul>
                </div>
                <article>
                    <h1>Patterns for Defensive Programming in Rust</h1>
                    <p>This article explains defensive programming techniques.</p>
                    <h2>Introduction</h2>
                    <p>Defensive programming is essential for building robust systems.</p>
                </article>
                <footer>
                    <p>Copyright 2024</p>
                </footer>
            </body>
            </html>
        "#;

        let result = extract_and_clean_body(html);

        // Should extract article content
        assert!(
            result.contains("Patterns for Defensive Programming"),
            "Expected to find article title"
        );
        assert!(
            result.contains("defensive programming techniques"),
            "Expected to find article content"
        );
        assert!(
            result.contains("Introduction"),
            "Expected to find article section heading"
        );

        // Should NOT contain navigation, sidebar, or footer
        assert!(
            !result.contains("Home") || !result.contains("About"),
            "Should not contain navigation links"
        );
        assert!(
            !result.contains("Related Links"),
            "Should not contain sidebar content"
        );
        assert!(
            !result.contains("Copyright"),
            "Should not contain footer content"
        );
    }

    #[test]
    fn test_multiple_articles_extracts_first() {
        let html = r#"
            <html>
            <body>
                <article>
                    <h1>First Article</h1>
                    <p>First article content.</p>
                </article>
                <article>
                    <h1>Second Article</h1>
                    <p>Second article content.</p>
                </article>
            </body>
            </html>
        "#;

        let result = extract_and_clean_body(html);

        // Should extract only the first article (or both, depending on implementation)
        assert!(
            result.contains("First Article"),
            "Expected to find first article"
        );
        // Note: Implementation may choose to extract all articles or just the first one
        // This test validates that at least the first article is extracted
    }

    #[test]
    fn test_role_main_attribute() {
        let html = r#"
            <html>
            <body>
                <nav>Navigation</nav>
                <div role="main">
                    <h1>Main Content via Role</h1>
                    <p>Content identified by role attribute.</p>
                </div>
                <aside>Sidebar</aside>
            </body>
            </html>
        "#;

        let result = extract_and_clean_body(html);

        // Should extract content with role="main"
        assert!(
            result.contains("Main Content via Role"),
            "Expected to find content with role='main'"
        );
        assert!(
            result.contains("role attribute"),
            "Expected to find main content text"
        );

        // Should NOT contain navigation or sidebar
        assert!(
            !result.contains("Navigation"),
            "Should not contain navigation"
        );
        assert!(
            !result.contains("Sidebar"),
            "Should not contain sidebar"
        );
    }

    #[test]
    #[ignore] // This test requires real HTTP fetch, run manually with: cargo test -- --ignored
    fn test_corrode_dev_article_extraction() {
        // This test validates the specific URL from Issue #40
        // URL: https://corrode.dev/blog/defensive-programming/
        //
        // This test is ignored by default because it requires:
        // 1. Network access to fetch the URL
        // 2. The website to be available
        // 3. The website structure to remain consistent
        //
        // To run this test manually:
        // cargo test test_corrode_dev_article_extraction -- --ignored
        //
        // Expected behavior after implementation:
        // - Should extract the main article content
        // - Should contain the article title "Patterns for Defensive Programming in Rust"
        // - Should have substantial content (> 1000 characters)
        // - Should NOT be empty

        use crate::http_client::HttpClient;
        use crate::http_config::HttpConfig;

        let text_with_url = "Check this article: https://corrode.dev/blog/defensive-programming/";

        // Fetch HTML using the existing API
        let http_config = HttpConfig::default();
        let http_client = HttpClient::new();

        let results = http_client.fetch_content_from_text(text_with_url, http_config);

        if results.is_empty() {
            eprintln!("Failed to fetch URL - network issue or URL unavailable");
            eprintln!("Skipping test");
            return;
        }

        // Get the HTML content
        let (_url, html) = &results[0];

        // Process content
        let processor = ContentProcessor::new();
        let result = processor.html_to_markdown(html);

        // Validate extraction
        assert!(
            !result.is_empty(),
            "Extracted content should not be empty"
        );

        assert!(
            result.len() > 1000,
            "Article should have substantial content (got {} characters)",
            result.len()
        );

        assert!(
            result.contains("Defensive Programming") || result.contains("defensive programming"),
            "Should contain article title or main topic"
        );

        // Print result for manual inspection
        println!("\n=== Extracted Content (first 500 chars) ===");
        println!("{}", &result.chars().take(500).collect::<String>());
        println!("\n=== Total length: {} characters ===", result.len());
    }
}
