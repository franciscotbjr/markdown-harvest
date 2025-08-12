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
#[derive(Default)]
pub struct ContentProcessor {}

impl ContentProcessor {
    /// Creates a new ContentProcessor instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Converts HTML content to clean Markdown format.
    pub fn html_to_markdown(&self, html: &str) -> String {
        extract_and_clean_body(html)
    }
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
}
