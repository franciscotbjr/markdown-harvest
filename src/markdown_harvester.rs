use crate::{content_processor::ContentProcessor, http_client::HttpClient};

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
        let http_client = HttpClient::new();
        let content_processor = ContentProcessor::new();

        // Step 1: Extract URLs and fetch HTML content
        let html_results = http_client.fetch_content_from_text(text.as_str());

        if html_results.is_empty() {
            return Vec::new();
        }

        // Step 2: Process HTML content to Markdown
        let mut markdown_results = Vec::new();

        for (url, html_content) in html_results {
            let markdown_content = content_processor.html_to_markdown(&html_content);
            println!(" Cleaned content from URL '{}':", url);
            println!("{}", markdown_content);
            markdown_results.push((url, markdown_content));
        }

        markdown_results
    }


}

