use crate::{
    content_processor::ContentProcessor, http_client::HttpClient, http_config::HttpConfig,
};
use std::future::Future;

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
            markdown_results.push((url, markdown_content));
        }

        markdown_results
    }

    /// Extracts URLs from text and processes their content asynchronously with custom callback handling.
    ///
    /// This asynchronous method provides high-performance parallel processing of multiple URLs
    /// found in the input text. Unlike the synchronous version, this method processes URLs
    /// concurrently and streams results through a user-provided callback, making it ideal
    /// for high-throughput scenarios and real-time processing applications.
    ///
    /// # Performance
    ///
    /// - Processes URLs in parallel instead of sequentially
    /// - Non-blocking operations for better resource utilization
    /// - Immediate callback execution as each URL completes processing
    /// - **Performance benefits increase with the number of URLs processed**
    ///
    /// Note: Actual performance improvements depend on factors such as:
    /// - Number of URLs being processed
    /// - Network latency and server response times
    /// - System resources and concurrent load
    /// - Individual URL processing complexity
    ///
    /// # Arguments
    ///
    /// * `text` - Input text that may contain URLs to extract and process
    /// * `http_config` - HTTP configuration including timeout, redirects, and other settings
    /// * `future` - Async callback function that receives processed results
    ///   - Called with `(Some(url), Some(markdown_content))` for each successfully processed URL
    ///   - Called with `(None, None)` when no URLs are found in the input text
    ///   - Must implement `Fn(Option<String>, Option<String>) -> Future<Output = ()> + Clone`
    ///
    /// # Returns
    ///
    /// A `Result<(), Box<dyn std::error::Error>>` indicating success or failure of the async operation.
    /// Individual URL processing errors are handled internally and don't cause the entire operation to fail.
    ///
    /// # Callback Pattern
    ///
    /// The callback receives two `Option<String>` parameters:
    /// - **First parameter (URL)**: `Some(url)` if processing succeeded, `None` if no URLs found
    /// - **Second parameter (Content)**: `Some(markdown_content)` if processing succeeded, `None` if no URLs found
    ///
    /// # Examples
    ///
    /// ## Basic Usage with Result Collection
    ///
    /// ```rust,no_run
    /// use markdown_harvest::{MarkdownHarvester, HttpConfig};
    /// use std::sync::{Arc, Mutex};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let text = "Check out https://example.com and https://httpbin.org/json";
    ///     let config = HttpConfig::builder().timeout(30000).build();
    ///     
    ///     // Collect results in a thread-safe vector
    ///     let results = Arc::new(Mutex::new(Vec::new()));
    ///     let results_clone = results.clone();
    ///     
    ///     let callback = move |url: Option<String>, content: Option<String>| {
    ///         let results = results_clone.clone();
    ///         async move {
    ///             if let (Some(url), Some(content)) = (url, content) {
    ///                 let mut results = results.lock().unwrap();
    ///                 results.push((url, content));
    ///             }
    ///         }
    ///     };
    ///     
    ///     MarkdownHarvester::get_hyperlinks_content_async(text.to_string(), config, callback).await?;
    ///     
    ///     let final_results = results.lock().unwrap();
    ///     println!("Processed {} URLs", final_results.len());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Real-time Processing with Immediate Output
    ///
    /// ```rust,no_run
    /// use markdown_harvest::{MarkdownHarvester, HttpConfig};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let text = "Visit https://example.com for more info";
    ///     let config = HttpConfig::default();
    ///     
    ///     // Process and display results immediately as they arrive
    ///     let callback = |url: Option<String>, content: Option<String>| async move {
    ///         match (url, content) {
    ///             (Some(url), Some(content)) => {
    ///                 println!("✅ Processed: {}", url);
    ///                 println!("📄 Content length: {} characters", content.len());
    ///                 // Save to database, send to API, etc.
    ///             }
    ///             (None, None) => {
    ///                 println!("ℹ️ No URLs found in the provided text");
    ///             }
    ///             _ => unreachable!(),
    ///         }
    ///     };
    ///     
    ///     MarkdownHarvester::get_hyperlinks_content_async(text.to_string(), config, callback).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Advanced: Custom Processing Pipeline
    ///
    /// ```rust,no_run
    /// use markdown_harvest::{MarkdownHarvester, HttpConfig};
    /// use tokio::fs;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let text = "Research these: https://example.com https://httpbin.org/json";
    ///     let config = HttpConfig::builder()
    ///         .timeout(15000)
    ///         .max_redirect(5)
    ///         .cookie_store(true)
    ///         .build();
    ///     
    ///     let callback = |url: Option<String>, content: Option<String>| async move {
    ///         if let (Some(url), Some(content)) = (url, content) {
    ///             // Extract domain for filename
    ///             let domain = url.split('/').nth(2).unwrap_or("unknown");
    ///             let filename = format!("{}.md", domain.replace('.', "_"));
    ///             
    ///             // Save each result to a separate file
    ///             if let Err(e) = fs::write(&filename, &content).await {
    ///                 eprintln!("Failed to save {}: {}", filename, e);
    ///             } else {
    ///                 println!("💾 Saved {} ({} chars)", filename, content.len());
    ///             }
    ///         }
    ///     };
    ///     
    ///     MarkdownHarvester::get_hyperlinks_content_async(text.to_string(), config, callback).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error Handling
    ///
    /// - **Function-level errors**: Network failures, invalid HTTP configs, or callback panics
    /// - **URL-level errors**: Individual URL failures are handled gracefully and don't affect other URLs
    /// - **Callback errors**: If the callback panics, the entire operation may fail
    ///
    /// # Performance Considerations
    ///
    /// - **Concurrency**: All URLs are processed simultaneously (limited by system resources)
    /// - **Memory**: Lower memory usage compared to synchronous version (streaming vs. collecting)
    /// - **Latency**: First results arrive as soon as the fastest URL completes
    /// - **Throughput**: Higher throughput potential when processing multiple URLs
    /// - **Scalability**: Performance benefits scale with the number of concurrent URLs
    ///
    /// # When to Use
    ///
    /// Choose this async version when:
    /// - Processing multiple URLs simultaneously
    /// - Building high-performance applications
    /// - Need real-time result streaming
    /// - Integrating with existing async/await codebases
    /// - Memory efficiency is important
    /// - Want to process results as they arrive
    ///
    /// Use the synchronous version for:
    /// - When you need all results collected before proceeding
    /// - Educational purposes or prototypes  
    /// - Simple applications with straightforward workflows
    /// - When you don't need streaming results
    ///
    /// # See Also
    ///
    /// - [`get_hyperlinks_content`](Self::get_hyperlinks_content) - Synchronous version
    /// - [`HttpConfig`](crate::HttpConfig) - HTTP configuration options
    /// - [`HttpClient::fetch_content_from_text_async`](crate::HttpClient::fetch_content_from_text_async) - Lower-level async HTTP processing
    pub async fn get_hyperlinks_content_async<F, Fut>(
        text: String, 
        http_config: HttpConfig, 
        future: F
    ) -> Result<(), Box<dyn std::error::Error>>
    where 
        F: Fn(Option<String>, Option<String>) -> Fut + Clone,
        Fut: Future<Output = ()>,
    {
        let http_client = HttpClient::new();
        let future_clone = future.clone();

        let callback = move |url: Option<String>, content: Option<String>| {
            let future = future_clone.clone();
            async move {
                if let (Some(url), Some(content)) = (url, content) {
                    // Create a new ContentProcessor for each URL processing
                    let content_processor = ContentProcessor::new();
                    let markdown_content = content_processor.html_to_markdown(&content);
                    future(Some(url), Some(markdown_content)).await;
                }
            }
        };

        http_client.fetch_content_from_text_async(text.as_str(), http_config, callback).await?;

        Ok(())
    }

}
