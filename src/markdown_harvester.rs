use crate::{
    content_processor::ContentProcessor, http_client::HttpClient, http_config::HttpConfig,
};

/// Main struct for extracting and converting web content from URLs to Markdown.
///
/// `MarkdownHarvester` provides functionality to detect URLs in text, fetch their content,
/// clean the HTML, and convert it to readable Markdown format. It's designed to be used
/// in Retrieval-Augmented Generation (RAG) systems where clean text content is needed
/// from web URLs.
///
/// # Examples
///
/// ```rust,no_run
/// use markdown_harvest::{MarkdownHarvester, HttpConfig};
///
/// let text = "Check out this article: https://example.com/news and https://example.com/blog";
/// let config = HttpConfig::default();
/// let results = MarkdownHarvester::get_hyperlinks_content(text.to_string(), config);
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
    /// Extracts URLs from the given text and fetches their content as Markdown with custom HTTP configuration.
    ///
    /// This method allows specifying custom HTTP configuration including timeout, retries, and other
    /// HTTP-related settings. This is useful when you need to control how HTTP requests are made,
    /// such as timeout duration, number of retries, or other connection parameters.
    ///
    /// # Arguments
    ///
    /// * `text` - Input text that may contain URLs
    /// * `http_config` - HTTP configuration including timeout, retries, and other HTTP settings
    ///
    /// # Returns
    ///
    /// A `Vec<(String, String)>` where each tuple contains:
    /// - First element: The URL that was processed
    /// - Second element: The cleaned Markdown content from that URL
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use markdown_harvest::{MarkdownHarvester, HttpConfig};
    ///
    /// // Use custom HTTP configuration with 5 seconds timeout
    /// let text = "Visit https://example.com for more info";
    /// let config = HttpConfig::builder().timeout(5000).build();
    /// let results = MarkdownHarvester::get_hyperlinks_content(text.to_string(), config);
    /// // Note: results may be empty due to network availability
    ///
    /// // Use default HTTP configuration
    /// let results = MarkdownHarvester::get_hyperlinks_content(text.to_string(), HttpConfig::default());
    /// ```
    pub fn get_hyperlinks_content(text: String, http_config: HttpConfig) -> Vec<(String, String)> {
        let http_client = HttpClient::new();
        let content_processor = ContentProcessor::new();

        // Step 1: Extract URLs and fetch HTML content
        let html_results = http_client.fetch_content_from_text(text.as_str(), http_config);

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
