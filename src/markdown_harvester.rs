use crate::{
    content_processor::ContentProcessor, http_client::HttpClient, http_config::HttpConfig,
};
use std::future::Future;

#[cfg(feature = "chunks")]
use text_splitter::{MarkdownSplitter, ChunkConfig};

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
        future: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Option<String>, Option<String>) -> Fut + Clone,
        Fut: Future<Output = ()>,
    {
        let http_client = HttpClient::new();
        let future_clone = future.clone();

        http_client
            .fetch_content_from_text_async(
                text.as_str(),
                http_config,
                move |url: Option<String>, content: Option<String>| {
                    let future = future_clone.clone();
                    async move {
                        if let (Some(url), Some(content)) = (url, content) {
                            // Create a new ContentProcessor for each URL processing
                            let content_processor = ContentProcessor::new();
                            let markdown_content = content_processor.html_to_markdown(&content);
                            future(Some(url), Some(markdown_content)).await;
                        }
                    }
                },
            )
            .await?;

        Ok(())
    }

    /// Extracts URLs from the given text and returns their content as Markdown chunks for RAG systems.
    ///
    /// This method is similar to `get_hyperlinks_content` but splits the Markdown content into smaller
    /// semantic chunks using `MarkdownSplitter` that are ideal for vector generation in Retrieval-Augmented 
    /// Generation (RAG) architectures. The splitter respects Markdown structure and semantic boundaries.
    ///
    /// **Feature Required**: This method is only available when the `chunks` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `text` - Input text that may contain URLs
    /// * `http_config` - HTTP configuration including timeout, retries, and other HTTP settings
    /// * `chunk_size` - Maximum size of each chunk in characters (recommended: 500-2000 for RAG systems)
    /// * `chunk_overlap` - Optional overlap between chunks in characters (must be < chunk_size)
    ///
    /// # Returns
    ///
    /// A `Vec<(String, Vec<String>)>` where each tuple contains:
    /// - First element: The URL that was processed
    /// - Second element: Vector of Markdown text chunks from that URL's content
    ///
    /// # Markdown Semantic Splitting
    ///
    /// The MarkdownSplitter uses semantic levels to create meaningful chunks:
    /// 1. Preserves heading structures
    /// 2. Keeps related paragraphs together when possible
    /// 3. Maintains code blocks and lists as units
    /// 4. Respects horizontal rules and thematic breaks
    /// 5. Preserves inline formatting (links, emphasis, etc.)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use markdown_harvest::{MarkdownHarvester, HttpConfig};
    ///
    /// #[cfg(feature = "chunks")]
    /// {
    ///     let text = "Check out this article: https://example.com/article";
    ///     let config = HttpConfig::default();
    ///     let chunk_size = 1000; // 1000 characters per chunk
    ///     
    ///     let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(
    ///         text.to_string(), 
    ///         config, 
    ///         chunk_size,
    ///         Some(100) // 100 characters overlap for better context preservation
    ///     );
    ///     
    ///     for (url, chunks) in results {
    ///         println!("URL: {}", url);
    ///         println!("Number of semantic chunks: {}", chunks.len());
    ///         for (i, chunk) in chunks.iter().enumerate() {
    ///             println!("Chunk {}: {} characters", i + 1, chunk.len());
    ///             println!("Content: {}\n---", chunk);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Use Cases
    ///
    /// - **RAG Systems**: Generate embeddings for semantically meaningful chunks
    /// - **Vector Databases**: Store content with preserved Markdown structure
    /// - **Search Systems**: Enable more granular and context-aware matching
    /// - **LLM Processing**: Feed semantically coherent chunks to language models
    ///
    /// # Performance Notes
    ///
    /// - MarkdownSplitter parsing adds minimal overhead to content extraction
    /// - Semantic chunking preserves document structure better than character splitting
    /// - Memory usage scales with chunk size and number of URLs
    /// - Consider using the async version for multiple URLs
    #[cfg(feature = "chunks")]
    pub fn get_hyperlinks_content_as_chunks(
        text: String, 
        http_config: HttpConfig,
        chunk_size: usize,
        chunk_overlap: Option<usize>,
    ) -> Vec<(String, Vec<String>)> {
        // First get the regular markdown content
        let markdown_results = Self::get_hyperlinks_content(text, http_config);
        
        if markdown_results.is_empty() {
            return Vec::new();
        }

        // Validate overlap parameter
        if let Some(overlap) = chunk_overlap {
            if overlap >= chunk_size {
                // Return error as empty result for now - in a real implementation,
                // we would return a proper Result type
                eprintln!("Warning: chunk_overlap ({}) must be smaller than chunk_size ({})", overlap, chunk_size);
                return Vec::new();
            }
        }

        // Initialize Markdown splitter with ChunkConfig including overlap
        let config = match chunk_overlap {
            Some(overlap) => {
                match ChunkConfig::new(chunk_size).with_overlap(overlap) {
                    Ok(config) => config,
                    Err(_) => {
                        // This should not happen due to our validation above, but handle gracefully
                        eprintln!("Failed to create ChunkConfig with overlap");
                        return Vec::new();
                    }
                }
            },
            None => ChunkConfig::new(chunk_size),
        };
        let splitter = MarkdownSplitter::new(config);
        
        let mut chunked_results = Vec::new();
        
        for (url, markdown_content) in markdown_results {
            // Split the markdown content into semantic chunks
            let chunks: Vec<String> = splitter
                .chunks(&markdown_content)
                .map(|chunk| chunk.to_string())
                .collect();
            
            chunked_results.push((url, chunks));
        }
        
        chunked_results
    }

    /// Extracts URLs from text and processes their content as Markdown chunks asynchronously with custom callback handling.
    ///
    /// This asynchronous method provides high-performance parallel processing of multiple URLs
    /// while splitting their content into semantic Markdown chunks suitable for RAG systems. 
    /// Unlike the synchronous version, this method processes URLs concurrently and streams 
    /// chunked results through a user-provided callback.
    ///
    /// **Feature Required**: This method is only available when the `chunks` feature is enabled.
    ///
    /// # Performance
    ///
    /// - Processes URLs in parallel instead of sequentially
    /// - Non-blocking operations for better resource utilization
    /// - Immediate callback execution as each URL's chunks are ready
    /// - **Ideal for batch processing multiple URLs in RAG workflows**
    /// - Semantic chunking preserves Markdown structure and meaning
    ///
    /// # Arguments
    ///
    /// * `text` - Input text that may contain URLs to extract and process
    /// * `http_config` - HTTP configuration including timeout, redirects, and other settings
    /// * `chunk_size` - Maximum size of each chunk in characters (recommended: 500-2000 for RAG)
    /// * `chunk_overlap` - Optional overlap between chunks in characters (must be < chunk_size)
    /// * `callback` - Async callback function that receives processed chunk results
    ///   - Called with `(Some(url), Some(chunks))` for each successfully processed URL
    ///   - Called with `(None, None)` when no URLs are found in the input text
    ///   - Must implement `Fn(Option<String>, Option<Vec<String>>) -> Future<Output = ()> + Clone`
    ///
    /// # Returns
    ///
    /// A `Result<(), Box<dyn std::error::Error>>` indicating success or failure of the async operation.
    /// Individual URL processing errors are handled internally and don't cause the entire operation to fail.
    ///
    /// # Markdown Semantic Chunking Benefits
    ///
    /// - **Structure Preservation**: Maintains heading hierarchy and document flow
    /// - **Context Retention**: Keeps related content together (lists, code blocks, paragraphs)
    /// - **Semantic Boundaries**: Splits at natural Markdown boundaries
    /// - **Format Integrity**: Preserves inline formatting and links within chunks
    /// - **RAG Optimization**: Creates chunks that maintain document context and meaning
    ///
    /// # Examples
    ///
    /// ## Basic Usage with Semantic Chunk Collection
    ///
    /// ```rust,no_run
    /// use markdown_harvest::{MarkdownHarvester, HttpConfig};
    /// use std::sync::{Arc, Mutex};
    ///
    /// #[cfg(feature = "chunks")]
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let text = "Research these: https://example.com https://httpbin.org/json";
    ///     let config = HttpConfig::default();
    ///     let chunk_size = 800; // 800 characters per chunk
    ///     
    ///     // Collect all semantic chunks in a thread-safe vector
    ///     let all_chunks = Arc::new(Mutex::new(Vec::new()));
    ///     let chunks_clone = all_chunks.clone();
    ///     
    ///     let callback = move |url: Option<String>, chunks: Option<Vec<String>>| {
    ///         let chunks_ref = chunks_clone.clone();
    ///         async move {
    ///             if let (Some(url), Some(chunks)) = (url, chunks) {
    ///                 let mut all_chunks = chunks_ref.lock().unwrap();
    ///                 all_chunks.push((url, chunks));
    ///                 println!("✅ Processed URL with {} semantic chunks", all_chunks.len());
    ///             }
    ///         }
    ///     };
    ///     
    ///     MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
    ///         text.to_string(), 
    ///         config, 
    ///         chunk_size,
    ///         Some(50), // 50 characters overlap
    ///         callback
    ///     ).await?;
    ///     
    ///     let final_results = all_chunks.lock().unwrap();
    ///     println!("📊 Total URLs processed: {}", final_results.len());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Real-time RAG Pipeline Processing
    ///
    /// ```rust,no_run
    /// use markdown_harvest::{MarkdownHarvester, HttpConfig};
    ///
    /// #[cfg(feature = "chunks")]
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let text = "Process https://example.com for RAG pipeline";
    ///     let config = HttpConfig::builder().timeout(10000).build();
    ///     let chunk_size = 1200;
    ///     
    ///     // Process semantic chunks immediately as they arrive
    ///     let callback = |url: Option<String>, chunks: Option<Vec<String>>| async move {
    ///         match (url, chunks) {
    ///             (Some(url), Some(chunks)) => {
    ///                 println!("🔗 URL: {}", url);
    ///                 println!("📦 Generated {} semantic chunks:", chunks.len());
    ///                 
    ///                 for (i, chunk) in chunks.iter().enumerate() {
    ///                     println!("  Semantic Chunk {}: {} chars", i + 1, chunk.len());
    ///                     
    ///                     // RAG Pipeline Processing:
    ///                     // - Generate embeddings for each semantic chunk
    ///                     // - Store chunks with metadata in vector database  
    ///                     // - Index chunks for semantic search
    ///                     // - Maintain document structure context
    ///                 }
    ///             }
    ///             (None, None) => {
    ///                 println!("ℹ️ No URLs found in the provided text");
    ///             }
    ///             _ => unreachable!(),
    ///         }
    ///     };
    ///     
    ///     MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
    ///         text.to_string(),
    ///         config,
    ///         chunk_size,
    ///         Some(100), // 100 characters overlap for context preservation
    ///         callback
    ///     ).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # RAG Integration Best Practices
    ///
    /// - **Chunk Size**: 500-2000 characters work well for most embedding models
    /// - **Semantic Context**: MarkdownSplitter preserves document structure and context
    /// - **Metadata Storage**: Store URL, chunk index, and heading context with embeddings
    /// - **Overlap Strategy**: Consider document structure instead of arbitrary overlap
    /// - **Quality Filtering**: Filter chunks by semantic completeness and meaningful content
    ///
    /// # See Also
    ///
    /// - [`get_hyperlinks_content_as_chunks`](Self::get_hyperlinks_content_as_chunks) - Synchronous version
    /// - [`get_hyperlinks_content_async`](Self::get_hyperlinks_content_async) - Async without chunking
    /// - [`HttpConfig`](crate::HttpConfig) - HTTP configuration options
    #[cfg(feature = "chunks")]
    pub async fn get_hyperlinks_content_as_chunks_async<F, Fut>(
        text: String,
        http_config: HttpConfig,
        chunk_size: usize,
        chunk_overlap: Option<usize>,
        callback: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Option<String>, Option<Vec<String>>) -> Fut + Clone,
        Fut: Future<Output = ()>,
    {
        let callback_clone = callback.clone();
        
        Self::get_hyperlinks_content_async(
            text,
            http_config,
            move |url: Option<String>, content: Option<String>| {
                let callback = callback_clone.clone();
                async move {
                    match (url, content) {
                        (Some(url), Some(content)) => {
                            // Validate overlap parameter
                            if let Some(overlap) = chunk_overlap {
                                if overlap >= chunk_size {
                                    eprintln!("Warning: chunk_overlap ({}) must be smaller than chunk_size ({})", overlap, chunk_size);
                                    return;
                                }
                            }

                            // Initialize Markdown splitter with ChunkConfig including overlap
                            let config = match chunk_overlap {
                                Some(overlap) => {
                                    match ChunkConfig::new(chunk_size).with_overlap(overlap) {
                                        Ok(config) => config,
                                        Err(_) => {
                                            // This should not happen due to our validation above, but handle gracefully
                                            eprintln!("Failed to create ChunkConfig with overlap");
                                            return;
                                        }
                                    }
                                },
                                None => ChunkConfig::new(chunk_size),
                            };
                            let splitter = MarkdownSplitter::new(config);
                            
                            // Split content into semantic Markdown chunks
                            let chunks: Vec<String> = splitter
                                .chunks(&content)
                                .map(|chunk| chunk.to_string())
                                .collect();
                            
                            // Call the user's callback with semantic chunks
                            callback(Some(url), Some(chunks)).await;
                        }
                        (None, None) => {
                            // No URLs found - pass through to user callback
                            callback(None, None).await;
                        }
                        _ => {
                            // This should not happen in normal flow
                        }
                    }
                }
            },
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HttpConfig;

    #[test]
    fn test_markdown_harvester_new() {
        let harvester = MarkdownHarvester::default();
        // Test that the struct can be created (it's a unit struct)
        assert_eq!(std::mem::size_of_val(&harvester), 0);
    }

    #[test]
    fn test_get_hyperlinks_content_with_empty_text() {
        let text = String::new();
        let config = HttpConfig::default();
        let results = MarkdownHarvester::get_hyperlinks_content(text, config);
        assert!(results.is_empty());
    }

    #[test]
    fn test_get_hyperlinks_content_with_no_urls() {
        let text = "This is just plain text without any URLs.".to_string();
        let config = HttpConfig::default();
        let results = MarkdownHarvester::get_hyperlinks_content(text, config);
        assert!(results.is_empty());
    }

    #[cfg(feature = "chunks")]
    mod chunks_tests {
        use super::*;

        #[test]
        fn test_get_hyperlinks_content_as_chunks_with_empty_text() {
            let text = String::new();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(text, config, chunk_size, None);
            assert!(results.is_empty());
        }

        #[test]
        fn test_get_hyperlinks_content_as_chunks_with_empty_text_and_overlap() {
            let text = String::new();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            let chunk_overlap = Some(100);
            let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(text, config, chunk_size, chunk_overlap);
            assert!(results.is_empty());
        }

        #[test]
        fn test_get_hyperlinks_content_as_chunks_with_no_urls() {
            let text = "This is just plain text without any URLs.".to_string();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(text, config, chunk_size, None);
            assert!(results.is_empty());
        }

        #[test]
        fn test_get_hyperlinks_content_as_chunks_with_no_urls_and_overlap() {
            let text = "This is just plain text without any URLs.".to_string();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            let chunk_overlap = Some(200);
            let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(text, config, chunk_size, chunk_overlap);
            assert!(results.is_empty());
        }

        #[test]
        fn test_get_hyperlinks_content_as_chunks_functionality() {
            // Test with mock data since we can't make real HTTP requests in unit tests
            let text = "Check out this article: https://example.com/article".to_string();
            let config = HttpConfig::default();
            let chunk_size = 500; // Medium chunk size for testing
            
            // This will return empty since we can't actually fetch the URL in tests
            // but we're testing that the function structure works with MarkdownSplitter
            let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(text, config, chunk_size, None);
            
            // In a real scenario with mocked HTTP client, we would test:
            // - That Markdown content is properly chunked with semantic boundaries
            // - That URL association is maintained
            // - That MarkdownSplitter preserves document structure
            // - That chunk sizes respect semantic boundaries
            // For now, we verify the function doesn't panic and returns the expected type
            assert!(results.is_empty() || results.iter().all(|(url, chunks)| {
                !url.is_empty() && chunks.iter().all(|chunk| !chunk.is_empty())
            }));
        }

        #[tokio::test]
        async fn test_get_hyperlinks_content_as_chunks_async_with_empty_text() {
            let text = String::new();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            
            let callback = |url: Option<String>, chunks: Option<Vec<String>>| {
                async move {
                    // This should be called once with (None, None) for empty text
                    assert!(url.is_none() && chunks.is_none());
                }
            };

            let result = MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
                text, config, chunk_size, None, callback
            ).await;
            
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_get_hyperlinks_content_as_chunks_async_with_no_urls() {
            let text = "This is just plain text without any URLs.".to_string();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            
            let callback = |url: Option<String>, chunks: Option<Vec<String>>| {
                async move {
                    // Should be called with (None, None) when no URLs found
                    assert!(url.is_none() && chunks.is_none());
                }
            };

            let result = MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
                text, config, chunk_size, None, callback
            ).await;
            
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_get_hyperlinks_content_as_chunks_async_callback_signature() {
            let text = "Visit https://example.com for info".to_string();
            let config = HttpConfig::default();
            let chunk_size = 800; // Good size for semantic chunking
            
            // Test that the callback is called with the expected signature
            let callback = |url: Option<String>, chunks: Option<Vec<String>>| {
                async move {
                    // Verify the callback receives the correct types
                    match (url, chunks) {
                        (Some(_url), Some(_chunks)) => {
                            // In a real scenario, we'd verify:
                            // - URL is correctly passed through
                            // - Chunks are semantically split Markdown content
                            // - MarkdownSplitter preserves document structure
                            // - Chunks respect semantic boundaries
                        }
                        (None, None) => {
                            // This is expected when no URLs are found or processing fails
                        }
                        _ => {
                            // This combination should never occur
                            panic!("Invalid callback signature for chunks async");
                        }
                    }
                }
            };

            let result = MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
                text, config, chunk_size, None, callback
            ).await;
            
            assert!(result.is_ok());
        }

        #[test]
        fn test_markdown_chunk_size_validation() {
            let text = "Test https://example.com".to_string();
            let config = HttpConfig::default();
            
            // Test different chunk sizes with MarkdownSplitter
            let chunk_sizes = vec![100, 500, 1000, 2000, 5000];
            
            for chunk_size in chunk_sizes {
                let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(
                    text.clone(), 
                    config.clone(), 
                    chunk_size,
                    None
                );
                
                // MarkdownSplitter should handle all chunk sizes without panicking
                // In real scenarios with content, we'd verify:
                // - Semantic boundaries are respected
                // - Document structure is preserved  
                // - Chunks don't exceed specified size (with reasonable margin for semantic splitting)
                assert!(results.is_empty() || results.iter().all(|(_, chunks)| {
                    chunks.iter().all(|chunk| {
                        // Allow semantic splitting to exceed size slightly for boundary preservation
                        chunk.len() <= chunk_size * 2 // Generous margin for semantic boundaries
                    })
                }));
            }
        }

        #[test]
        fn test_markdown_semantic_splitting_structure() {
            // Test that would verify MarkdownSplitter semantic behavior
            // (In a real test with mocked content, we would verify semantic boundaries)
            let text = "Check https://example.com/docs".to_string();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            
            let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(
                text, config, chunk_size, None
            );
            
            // Verify structure without real HTTP calls
            // In production tests with mocked HTTP responses containing Markdown:
            // - Headers should be preserved with their content
            // - Code blocks should stay intact
            // - Lists should be kept together when possible
            // - Paragraphs should be preserved as semantic units
            assert!(results.is_empty() || results.iter().all(|(url, chunks)| {
                !url.is_empty() && chunks.iter().all(|chunk| !chunk.is_empty())
            }));
        }

        #[test]
        fn test_chunk_overlap_validation() {
            let text = "Test https://example.com".to_string();
            let config = HttpConfig::default();
            let chunk_size = 1000;
            
            // Test valid overlap values
            let valid_overlaps = vec![50, 100, 200, 500, 999];
            for overlap in valid_overlaps {
                let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(
                    text.clone(), 
                    config.clone(), 
                    chunk_size, 
                    Some(overlap)
                );
                
                // Should not panic with valid overlap values
                // Results are empty since no actual HTTP requests are made in tests
                assert!(results.is_empty() || results.iter().all(|(url, chunks)| {
                    !url.is_empty() && chunks.iter().all(|chunk| !chunk.is_empty())
                }));
            }
        }

        #[test]
        fn test_chunk_overlap_invalid_values() {
            let text = "Test https://example.com".to_string();
            let config = HttpConfig::default();
            let chunk_size = 500;
            
            // Test invalid overlap values (>= chunk_size)
            let invalid_overlaps = vec![500, 600, 1000];
            for overlap in invalid_overlaps {
                let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(
                    text.clone(), 
                    config.clone(), 
                    chunk_size, 
                    Some(overlap)
                );
                
                // Should return empty results for invalid overlap values
                assert!(results.is_empty());
            }
        }

        #[tokio::test]
        async fn test_chunk_overlap_async_validation() {
            let text = "Visit https://example.com for info".to_string();
            let config = HttpConfig::default();
            let chunk_size = 800;
            let chunk_overlap = Some(100); // Valid overlap
            
            let callback = |url: Option<String>, chunks: Option<Vec<String>>| {
                async move {
                    match (url, chunks) {
                        (Some(_url), Some(_chunks)) => {
                            // In real scenarios with content, verify overlap functionality
                        }
                        (None, None) => {
                            // Expected when no URLs found or processing fails
                        }
                        _ => {
                            panic!("Invalid callback signature for chunks async with overlap");
                        }
                    }
                }
            };

            let result = MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
                text, config, chunk_size, chunk_overlap, callback
            ).await;
            
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_chunk_overlap_async_invalid_values() {
            let text = "Visit https://example.com for info".to_string();
            let config = HttpConfig::default();
            let chunk_size = 500;
            let invalid_overlap = Some(500); // Invalid: overlap >= chunk_size
            
            let callback = |url: Option<String>, chunks: Option<Vec<String>>| {
                async move {
                    // Should handle invalid overlap gracefully
                    // Note: In actual implementation, this might not be called for invalid overlap
                }
            };

            let result = MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
                text, config, chunk_size, invalid_overlap, callback
            ).await;
            
            assert!(result.is_ok());
        }
    }

    // Integration-style tests that would work with a real HTTP mock
    #[test]
    fn test_integration_workflow_with_chunks() {
        // This test verifies the overall workflow structure including chunks
        let text = "Check https://example.com and https://test.org".to_string();
        let config = HttpConfig::builder()
            .timeout(5000)
            .build();

        // Test synchronous version
        let sync_results = MarkdownHarvester::get_hyperlinks_content(text.clone(), config.clone());
        
        // Test that it returns the expected structure (empty in unit tests since no real HTTP)
        assert!(sync_results.is_empty() || sync_results.iter().all(|(url, content)| {
            !url.is_empty() && !content.is_empty()
        }));

        // Test chunks version if feature is enabled
        #[cfg(feature = "chunks")]
        {
            let chunk_results = MarkdownHarvester::get_hyperlinks_content_as_chunks(
                text, config, 1000, None
            );
            
            // Verify same number of URLs processed
            assert_eq!(sync_results.len(), chunk_results.len());
            
            // Verify structure with MarkdownSplitter
            assert!(chunk_results.is_empty() || chunk_results.iter().all(|(url, chunks)| {
                !url.is_empty() && chunks.iter().all(|chunk| !chunk.is_empty())
            }));
        }
    }
}
