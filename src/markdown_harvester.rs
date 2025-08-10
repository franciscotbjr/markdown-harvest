use crate::{
    patterns::{
        additional_cleanup, content_selectors, media_elements, text_selectors, unwanted_elements,
        unwanted_text_patterns,
    },
    user_agent::UserAgent,
};

use regex::Regex;
use reqwest::blocking::Client;
use scraper::{Html, Selector};

/// Main struct for extracting and converting web content from URLs to Markdown.
///
/// `MarkdownHarvester` provides functionality to detect URLs in text, fetch their content,
/// clean the HTML, and convert it to readable Markdown format. It's designed to be used
/// in Retrieval-Augmented Generation (RAG) systems where clean text content is needed
/// from web URLs.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::MarkdownHarvester;
///
/// let text = "Check out this article: https://example.com/news and https://example.com/blog";
/// let results = MarkdownHarvester::get_hyperlinks_content(text.to_string());
///
/// println!("Found {} URLs with content", results.len());
/// for (url, content) in results {
///     println!("URL: {}", url);
///     println!("Content preview: {}...", &content[..content.len().min(100)]);
/// }
/// ```
#[derive(Default)]
pub struct MarkdownHarvester {}

impl MarkdownHarvester {
    /// Extracts URLs from the given text and fetches their content as Markdown.
    ///
    /// This method performs the following operations:
    /// 1. Scans the input text for HTTP/HTTPS URLs using regex
    /// 2. Cleans URLs by removing trailing punctuation
    /// 3. Fetches content from each URL using HTTP requests
    /// 4. Extracts and cleans HTML content from the response
    /// 5. Converts cleaned HTML to Markdown format
    /// 6. Returns a vector of tuples containing (URL, Markdown content)
    ///
    /// # Arguments
    ///
    /// * `text` - Input text that may contain URLs
    ///
    /// # Returns
    ///
    /// A `Vec<(String, String)>` where each tuple contains:
    /// - First element: The URL that was processed
    /// - Second element: The cleaned Markdown content from that URL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_harvest::MarkdownHarvester;
    ///
    /// // Single URL
    /// let text = "Visit https://example.com for more info";
    /// let results = MarkdownHarvester::get_hyperlinks_content(text.to_string());
    /// assert!(!results.is_empty());
    ///
    /// // Multiple URLs (note: some URLs may fail due to network issues)
    /// let text = "Check https://example.com and https://httpbin.org/html";
    /// let results = MarkdownHarvester::get_hyperlinks_content(text.to_string());
    /// assert!(!results.is_empty()); // At least one should succeed
    ///
    /// // No URLs
    /// let text = "This text has no URLs";
    /// let results = MarkdownHarvester::get_hyperlinks_content(text.to_string());
    /// assert!(results.is_empty());
    /// ```
    ///
    /// # Errors
    ///
    /// This method handles network errors gracefully by logging them to stderr
    /// and skipping the problematic URLs. URLs that fail to fetch will not
    /// appear in the returned results.
    ///
    /// # Panics
    ///
    /// This method will panic if the internal URL regex compilation fails,
    /// which should never happen under normal circumstances as the regex
    /// pattern is hardcoded and valid.
    pub fn get_hyperlinks_content(text: String) -> Vec<(String, String)> {
        // Regular expression to identify HTTP/HTTPS URLs
        let url_regex = Regex::new(r"https?://[a-zA-Z0-9._/%+-]+(?:/[a-zA-Z0-9._/%+-]*)*").unwrap();

        // Capture all URLs present in the text
        let urls: Vec<String> = url_regex
            .find_iter(&text)
            .map(|m| clean_url(m.as_str()))
            .collect();

        if urls.is_empty() {
            return Vec::new();
        }

        handles_http_requests_results(urls)
    }
}

fn handles_http_requests_results(urls: Vec<String>) -> Vec<(String, String)> {
    let client = Client::new();
    let mut results = Vec::new();

    for url in &urls {
        let user_agent = UserAgent::random();
        match client
            .get(url)
            .header("User-Agent", user_agent.to_string())
            .send()
        {
            Ok(response) => match response.text() {
                Ok(html_content) => {
                    let cleaned_content = extract_and_clean_body(&html_content);
                    results.push((url.to_string(), cleaned_content.clone()));
                    println!(" Cleaned content from URL '{}':", url);
                    println!("{}", cleaned_content);
                }
                Err(e) => {
                    eprintln!("Error reading content from {}: {}", url, e);
                }
            },
            Err(e) => {
                eprintln!("Error accessing {}: {}", url, e);
            }
        }
    }
    return results;
}

fn clean_url(url: &str) -> String {
    // Remove common punctuation at the end of URLs
    url.trim_end_matches(&['.', ',', ';', '!', '?', ')', ']', '}'][..])
        .to_string()
}

fn extract_and_clean_body(html: &str) -> String {
    // Step 1: Extract only the body content from the HTML
    let document = Html::parse_document(html);
    let body_selector = Selector::parse("body").unwrap();

    let body_html = match document.select(&body_selector).next() {
        Some(body_element) => body_element.html(),
        None => return String::new(), // Return empty if no body found
    };

    // Step 2: Clean the body content by removing unwanted elements
    let relevant_html = clear_body(body_html);

    // Step 3: Convert the cleaned HTML to Markdown
    let markdown_content = html2md::parse_html(&relevant_html);

    // Step 4: Final cleanup
    // Remove unwanted elements while preserving Markdown structure
    final_clean_from_markdown(markdown_content)
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

fn clear_body(body_html: String) -> String {
    let mut cleaned_body = body_html;

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

            // Skip lines that look like metadata or navigation
            let lower = trimmed.to_lowercase();
            if lower.starts_with("http")
                || lower.contains("@")
                || (trimmed
                    .chars()
                    .all(|c| c.is_alphabetic() || c.is_whitespace())
                    && !trimmed.contains(' '))
            {
                // Single words without spaces
                return false;
            }

            true
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ======= Integration Tests (existing scenarios) =======

    #[test]
    fn test_w3_org_html_specs_page_extraction() {
        let input_text = "Learn more about HTML specs: https://www.w3.org/wiki/HTML/Specifications";
        let results = MarkdownHarvester::get_hyperlinks_content(input_text.to_string());

        // Should have exactly one result
        assert_eq!(results.len(), 1);

        let (url, content) = &results[0];

        // Validate URL
        assert_eq!(url, "https://www.w3.org/wiki/HTML/Specifications");

        // Test basic functionality - the extraction should at least attempt to process
        println!("✅ W3C HTML Specifications page extraction test completed!");
        println!("🌐 URL processed: {}", url);
        println!("📄 Content length: {} characters", content.len());

        if !content.is_empty() {
            println!(
                "📝 Content preview: {}",
                &content[..std::cmp::min(200, content.len())]
            );

            // If we got content, validate it doesn't contain actual HTML tags or URLs
            // Note: escaped characters like \< are acceptable
            assert!(
                !content.contains("<html") && !content.contains("<div") && !content.contains("<p>"),
                "Content should not contain actual HTML tags"
            );
            assert!(
                !content.contains("https://") && !content.contains("http://"),
                "Content should not contain standalone URLs"
            );
        } else {
            println!(
                "ℹ️  W3C HTML Specifications page returned empty content (complex page structure)"
            );
        }
    }

    #[test]
    fn test_url_detection_and_cleanup() {
        let input_text = "Check out these sites: https://example.com and https://www.w3.org/wiki/HTML/Specifications";
        let results = MarkdownHarvester::get_hyperlinks_content(input_text.to_string());

        // Should detect both URLs
        assert_eq!(results.len(), 2);

        // Verify URLs are cleaned properly
        for (url, _) in &results {
            assert!(
                url == "https://example.com"
                    || url == "https://www.w3.org/wiki/HTML/Specifications"
            );
        }
    }

    #[test]
    fn test_empty_input() {
        let input_text = "";
        let results = MarkdownHarvester::get_hyperlinks_content(input_text.to_string());

        // Should return empty results
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_no_urls_in_text() {
        let input_text = "This text has no URLs at all, just plain text content.";
        let results = MarkdownHarvester::get_hyperlinks_content(input_text.to_string());

        // Should return empty results
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_markdown_formatting_preservation() {
        let input_text = "Test formatting: https://www.w3.org/wiki/HTML/Specifications";
        let results = MarkdownHarvester::get_hyperlinks_content(input_text.to_string());

        if !results.is_empty() {
            let (_, content) = &results[0];

            // Test that content doesn't contain actual HTML tags or URLs
            // Note: escaped characters like \< are acceptable
            assert!(
                !content.contains("<html") && !content.contains("<div") && !content.contains("<p>"),
                "Content should not contain actual HTML tags"
            );
            assert!(
                !content.contains("https://") && !content.contains("http://"),
                "Content should not contain standalone URLs"
            );

            // Content should be properly formatted text
            assert!(!content.trim().is_empty(), "Content should not be empty");

            println!(
                "✅ Markdown formatting test completed with content: {}",
                &content[..std::cmp::min(100, content.len())]
            );
        } else {
            println!("ℹ️  No content extracted for markdown formatting test");
        }
    }

    // ======= Unit Tests for Individual Functions =======

    #[test]
    fn test_clean_url_function() {
        // Test URL cleaning with various punctuation
        assert_eq!(clean_url("https://example.com."), "https://example.com");
        assert_eq!(clean_url("https://example.com,"), "https://example.com");
        assert_eq!(clean_url("https://example.com;"), "https://example.com");
        assert_eq!(clean_url("https://example.com!"), "https://example.com");
        assert_eq!(clean_url("https://example.com?"), "https://example.com");
        assert_eq!(clean_url("https://example.com)"), "https://example.com");
        assert_eq!(clean_url("https://example.com]"), "https://example.com");
        assert_eq!(clean_url("https://example.com}"), "https://example.com");

        // Test URL without punctuation (should remain unchanged)
        assert_eq!(clean_url("https://example.com"), "https://example.com");

        // Test multiple punctuation at end
        assert_eq!(clean_url("https://example.com.,;"), "https://example.com");
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
    fn test_clear_body_removes_scripts_and_styles() {
        let html_with_scripts = r#"
            <div>Content before</div>
            <script>alert('test');</script>
            <style>.test { color: red; }</style>
            <div>Content after</div>
        "#
        .to_string();

        let result = clear_body(html_with_scripts);

        // Should remove scripts and styles
        assert!(!result.contains("<script>"));
        assert!(!result.contains("alert('test')"));
        assert!(!result.contains("<style>"));
        assert!(!result.contains(".test { color: red; }"));

        // Should keep other content
        assert!(result.contains("Content before"));
        assert!(result.contains("Content after"));
    }

    #[test]
    fn test_final_clean_from_markdown_removes_html_tags() {
        let markdown_with_html = "# Title\n\n<div>Some content</div>\n\n**Bold text**".to_string();
        let result = final_clean_from_markdown(markdown_with_html);

        // Should remove HTML tags
        assert!(!result.contains("<div>"));
        assert!(!result.contains("</div>"));

        // Should preserve markdown formatting
        assert!(result.contains("# Title"));
        assert!(result.contains("**Bold text**"));
        assert!(result.contains("Some content"));
    }

    #[test]
    fn test_final_clean_from_markdown_removes_links() {
        let markdown_with_links =
            "Check out [this link](https://example.com) for more info.".to_string();
        let result = final_clean_from_markdown(markdown_with_links);

        // Should remove the URL but keep the text
        assert!(result.contains("this link"));
        assert!(!result.contains("https://example.com"));
        assert!(!result.contains("["));
        assert!(!result.contains("]"));
    }

    #[test]
    fn test_final_clean_from_markdown_removes_standalone_urls() {
        let markdown_with_urls =
            "Visit https://example.com and http://test.org for more.".to_string();
        let result = final_clean_from_markdown(markdown_with_urls);

        // Should remove standalone URLs
        assert!(!result.contains("https://example.com"));
        assert!(!result.contains("http://test.org"));
        assert!(result.contains("Visit") && result.contains("and") && result.contains("for more"));
    }

    #[test]
    fn test_final_clean_from_markdown_removes_code_blocks() {
        let markdown_with_code =
            "Some text\n\n```python\nprint('hello')\n```\n\nMore text.".to_string();
        let result = final_clean_from_markdown(markdown_with_code);

        // Should remove code blocks
        assert!(!result.contains("```"));
        assert!(!result.contains("print('hello')"));

        // Should keep other text
        assert!(result.contains("Some text"));
        assert!(result.contains("More text"));
    }

    #[test]
    fn test_final_clean_from_markdown_normalizes_whitespace() {
        let markdown_with_spaces =
            "Text   with    multiple   spaces\n\n\n\nAnd multiple newlines.".to_string();
        let result = final_clean_from_markdown(markdown_with_spaces);

        // Should normalize multiple spaces to single spaces
        assert!(!result.contains("   "));
        assert!(result.contains("Text with multiple spaces"));

        // Should limit consecutive newlines
        assert!(!result.contains("\n\n\n\n"));
    }

    #[test]
    fn test_remove_lines_metadata_or_navigation() {
        let lines = vec![
            "# Main Title",
            "",
            "This is good content.",
            "http://example.com",
            "user@email.com",
            "VeryLongSingleWordWithoutSpaces",
            "Another good paragraph.",
            "",
            "## Subtitle",
            "More content here.",
        ];

        let result = remove_lines_metadata_or_navigation(lines);

        // Should keep headers and good content
        assert!(result.contains(&"# Main Title"));
        assert!(result.contains(&"This is good content."));
        assert!(result.contains(&"## Subtitle"));
        assert!(result.contains(&"More content here."));

        // Should remove URLs, emails, and single words
        assert!(!result.contains(&"http://example.com"));
        assert!(!result.contains(&"user@email.com"));
        assert!(!result.contains(&"VeryLongSingleWordWithoutSpaces"));

        // Should preserve empty lines
        assert!(result.contains(&""));
    }

    #[test]
    fn test_handles_http_requests_results_with_empty_urls() {
        let empty_urls: Vec<String> = vec![];
        let result = handles_http_requests_results(empty_urls);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_patterns_integration() {
        use crate::patterns::{
            additional_cleanup, content_selectors, media_elements, text_selectors,
            unwanted_elements, unwanted_text_patterns,
        };

        // Test that Patterns methods return expected arrays
        assert_eq!(media_elements().len(), 9);
        assert_eq!(unwanted_elements().len(), 7);
        assert_eq!(content_selectors().len(), 7);
        assert_eq!(text_selectors().len(), 6);
        assert_eq!(additional_cleanup().len(), 3);
        assert_eq!(unwanted_text_patterns().len(), 7);

        // Test some specific patterns
        assert!(media_elements().contains(&r"(?i)<img[^>]*>"));
        assert!(unwanted_elements().contains(&r"(?i)<nav[^>]*>[\s\S]*?</nav>"));
        assert!(content_selectors().contains(&"article"));
        assert!(text_selectors().contains(&"h1, h2, h3, h4, h5, h6"));
    }
}
