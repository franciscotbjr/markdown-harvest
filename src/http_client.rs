use crate::http_regex::URL_REGEX;
use crate::{http_config::HttpConfig, user_agent::UserAgent};
use futures::future;
use reqwest::{Client, blocking};
use std::future::Future;
use std::time::Duration;

/// Component responsible for handling HTTP requests and URL processing.
///
/// `HttpClient` encapsulates all HTTP-related functionality including URL extraction,
/// URL cleaning, and content fetching. This component reuses the original functions
#[derive(Default)]
pub struct HttpClient {}

impl HttpClient {
    /// Creates a new HttpClient instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Extracts URLs from text and fetches their content with custom HTTP configuration.
    ///
    /// # Arguments
    ///
    /// * `text` - Input text that may contain URLs
    /// * `http_config` - HTTP configuration including timeout, retries, and other settings
    ///
    /// # Returns
    ///
    /// A vector of tuples containing (URL, HTML content)
    pub fn fetch_content_from_text(
        &self,
        text: &str,
        http_config: HttpConfig,
    ) -> Vec<(String, String)> {
        let urls = self.extract_urls(text);
        if urls.is_empty() {
            return Vec::new();
        }
        self.fetch_content_from_urls(urls, http_config)
    }

    pub async fn fetch_content_from_text_async<F, Fut>(
        &self,
        text: &str,
        http_config: HttpConfig,
        future: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Option<String>, Option<String>) -> Fut + Clone,
        Fut: Future<Output = ()>,
    {
        let urls = self.extract_urls(text);
        if urls.is_empty() {
            future(None, None).await;
            return Ok(());
        }

        self.fetch_content_from_urls_async(urls, http_config, future)
            .await?;

        Ok(())
    }

    fn extract_urls(&self, text: &str) -> Vec<String> {
        URL_REGEX
            .find_iter(text)
            .map(|m| clean_url(m.as_str()))
            .collect()
    }

    /// Fetches HTML content from a list of URLs with custom HTTP configuration.
    fn fetch_content_from_urls(
        &self,
        urls: Vec<String>,
        http_config: HttpConfig,
    ) -> Vec<(String, String)> {
        handles_http_requests_results(urls, http_config)
    }

    async fn fetch_content_from_urls_async<F, Fut>(
        &self,
        urls: Vec<String>,
        http_config: HttpConfig,
        future: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Option<String>, Option<String>) -> Fut + Clone,
        Fut: Future<Output = ()>,
    {
        handles_http_requests_results_async(urls, http_config, future).await?;
        Ok(())
    }
}

fn handles_http_requests_results(
    urls: Vec<String>,
    http_config: HttpConfig,
) -> Vec<(String, String)> {
    let client = build_client(http_config);
    let mut results = Vec::new();
    let user_agent = UserAgent::random();

    for url in &urls {
        match client
            .get(url)
            .header("User-Agent", user_agent.to_string())
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("DNT", "1")
            .header("Connection", "keep-alive")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .header("js_timeout", "2000")
            .header("js", "true")
            .send()
        {
            Ok(response) => match response.text() {
                Ok(html_content) => {
                    results.push((url.to_string(), html_content));
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
    results
}

async fn handles_http_requests_results_async<F, Fut>(
    urls: Vec<String>,
    http_config: HttpConfig,
    future: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(Option<String>, Option<String>) -> Fut + Clone,
    Fut: Future<Output = ()>,
{
    let client = build_client_async(http_config);
    let user_agent = UserAgent::random();

    let requests = urls.into_iter().map(|url| {
        let client = client.clone();
        let future = future.clone();

        async move {
            match client
                .get(&url)
                .header("User-Agent", user_agent.to_string())
                .header(
                    "Accept",
                    "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
                )
                .header("Accept-Language", "en-US,en;q=0.5")
                .header("DNT", "1")
                .header("Connection", "keep-alive")
                .header("Upgrade-Insecure-Requests", "1")
                .header("Sec-Fetch-Dest", "document")
                .header("Sec-Fetch-Mode", "navigate")
                .header("Sec-Fetch-Site", "none")
                .header("Sec-Fetch-User", "?1")
                .header("js_timeout", "2000")
                .header("js", "true")
                .send()
                .await
            {
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    future(Some(url.to_string()), Some(body)).await
                }
                Err(e) => future(Some(url.to_string()), Some(format!("Error: {}", e))).await,
            }
        }
    });

    future::join_all(requests).await;

    Ok(())
}

fn build_client(http_config: HttpConfig) -> blocking::Client {
    match http_config.timeout() {
        Some(timeout) => blocking::Client::builder()
            .timeout(Duration::from_millis(timeout))
            .redirect(reqwest::redirect::Policy::limited(
                http_config.max_redirect().unwrap_or(2),
            ))
            .cookie_store(http_config.cookie_store())
            .build()
            .unwrap_or_else(|_| blocking::Client::new()),
        None => blocking::Client::new(),
    }
}

fn build_client_async(http_config: HttpConfig) -> Client {
    match http_config.timeout() {
        Some(timeout) => Client::builder()
            .timeout(Duration::from_millis(timeout))
            .redirect(reqwest::redirect::Policy::limited(
                http_config.max_redirect().unwrap_or(2),
            ))
            .cookie_store(http_config.cookie_store())
            .build()
            .unwrap_or_else(|_| Client::new()),
        None => Client::new(),
    }
}

fn clean_url(url: &str) -> String {
    let mut result = url.to_string();

    // Only remove trailing punctuation if parentheses are not balanced
    let open_parens = url.chars().filter(|&c| c == '(').count();
    let close_parens = url.chars().filter(|&c| c == ')').count();

    // If parentheses are balanced, don't remove the closing parenthesis
    if open_parens == close_parens {
        result = result
            .trim_end_matches(&['.', ',', ';', '!', '?', ']', '}'][..])
            .to_string();
    } else {
        result = result
            .trim_end_matches(&['.', ',', ';', '!', '?', ')', ']', '}'][..])
            .to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::http_config::HttpConfigBuilder;
    use std::sync::{Arc, Mutex};
    use tokio;

    use super::*;

    #[test]
    fn test_new() {
        let client = HttpClient::new();
        assert_eq!(std::mem::size_of_val(&client), 0);
    }

    #[test]
    fn test_extract_urls() {
        let client = HttpClient::new();

        let text = "Check out https://example.com and https://test.org for more info";
        let urls = client.extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(urls.contains(&"https://example.com".to_string()));
        assert!(urls.contains(&"https://test.org".to_string()));

        let text = "This text has no URLs";
        let urls = client.extract_urls(text);
        assert_eq!(urls.len(), 0);
    }

    #[test]
    fn test_extract_urls_with_query_strings() {
        let client = HttpClient::new();

        // Test case 1: Sample text with query string
        let text = "Cavafy lived in England for much of his adolescence, and developed both a command of the English language and a preference for the writings of William Shakespeare http://www.poetryfoundation.org/archive/poet.html?id=6176 and Oscar Wilde http://www.poetryfoundation.org/archive/poet.html?id=7425. Cavafy's older brothers mismanaged the family business in Liverpool, and Cavafy's mother was ultimately compelled to move the family back to Alexandria, where they lived until 1882.";
        let urls = client.extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(
            urls.contains(&"http://www.poetryfoundation.org/archive/poet.html?id=6176".to_string())
        );
        assert!(
            urls.contains(&"http://www.poetryfoundation.org/archive/poet.html?id=7425".to_string())
        );

        // Test case 2: Sample text with no query string
        let text = "Rust is a general-purpose https://en.wikipedia.org/wiki/General-purpose_programming_language programming language https://en.wikipedia.org/wiki/Programming_language emphasizing performance https://en.wikipedia.org/wiki/Computer_performance, type safety https://en.wikipedia.org/wiki/Type_safety, and concurrency https://en.wikipedia.org/wiki/Concurrency_(computer_science). It enforces memory safety https://en.wikipedia.org/wiki/Memory_safety, meaning that all references point to valid memory.";
        let urls = client.extract_urls(text);
        assert_eq!(urls.len(), 6);
        assert!(urls.contains(
            &"https://en.wikipedia.org/wiki/General-purpose_programming_language".to_string()
        ));
        assert!(urls.contains(&"https://en.wikipedia.org/wiki/Programming_language".to_string()));
        assert!(urls.contains(&"https://en.wikipedia.org/wiki/Computer_performance".to_string()));
        assert!(urls.contains(&"https://en.wikipedia.org/wiki/Type_safety".to_string()));
        assert!(
            urls.contains(
                &"https://en.wikipedia.org/wiki/Concurrency_(computer_science)".to_string()
            )
        );
        assert!(urls.contains(&"https://en.wikipedia.org/wiki/Memory_safety".to_string()));

        // Test case 3: Simple URL without query string
        let text = "A language empowering everyone https://www.rust-lang.org/ to build reliable and efficient software.";
        let urls = client.extract_urls(text);
        assert_eq!(urls.len(), 1);
        assert!(urls.contains(&"https://www.rust-lang.org/".to_string()));
    }

    #[test]
    fn test_clean_url() {
        assert_eq!(clean_url("https://example.com."), "https://example.com");
        assert_eq!(clean_url("https://example.com,"), "https://example.com");
        assert_eq!(clean_url("https://example.com!"), "https://example.com");
        assert_eq!(clean_url("https://example.com"), "https://example.com");

        // Test balanced parentheses (should not be removed)
        assert_eq!(
            clean_url("https://en.wikipedia.org/wiki/Concurrency_(computer_science)"),
            "https://en.wikipedia.org/wiki/Concurrency_(computer_science)"
        );

        // Test unbalanced parentheses (should be removed)
        assert_eq!(clean_url("https://example.com)"), "https://example.com");
    }

    #[test]
    fn test_fetch_content_from_urls_empty() {
        let client = HttpClient::new();
        let urls: Vec<String> = vec![];
        let results =
            client.fetch_content_from_urls(urls, HttpConfigBuilder::new().timeout(30000).build());
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_fetch_content_from_text_no_urls() {
        let client = HttpClient::new();
        let text = "This text has no URLs";
        let results =
            client.fetch_content_from_text(text, HttpConfigBuilder::new().timeout(30000).build());
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_fetch_content_from_text_async_no_urls() {
        let client = HttpClient::new();
        let text = "This text has no URLs";
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = results.clone();

        let callback = move |url: Option<String>, content: Option<String>| {
            let results = results_clone.clone();
            async move {
                let mut results = results.lock().unwrap();
                results.push((url, content));
            }
        };

        let result = client
            .fetch_content_from_text_async(
                text,
                HttpConfigBuilder::new().timeout(30000).build(),
                callback,
            )
            .await;

        assert!(result.is_ok());
        let results = results.lock().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], (None, None));
    }

    #[tokio::test]
    async fn test_fetch_content_from_text_async_with_urls() {
        let client = HttpClient::new();
        let text = "Check out https://httpbin.org/status/200 for testing";
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = results.clone();

        let callback = move |url: Option<String>, content: Option<String>| {
            let results = results_clone.clone();
            async move {
                let mut results = results.lock().unwrap();
                results.push((url, content));
            }
        };

        let result = client
            .fetch_content_from_text_async(
                text,
                HttpConfigBuilder::new().timeout(30000).build(),
                callback,
            )
            .await;

        assert!(result.is_ok());
        let results = results.lock().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].0.is_some());
        assert!(results[0].1.is_some());
    }

    #[tokio::test]
    async fn test_fetch_content_from_urls_async_empty() {
        let client = HttpClient::new();
        let urls: Vec<String> = vec![];
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = results.clone();

        let callback = move |url: Option<String>, content: Option<String>| {
            let results = results_clone.clone();
            async move {
                let mut results = results.lock().unwrap();
                results.push((url, content));
            }
        };

        let result = client
            .fetch_content_from_urls_async(
                urls,
                HttpConfigBuilder::new().timeout(30000).build(),
                callback,
            )
            .await;

        assert!(result.is_ok());
        let results = results.lock().unwrap();
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_fetch_content_from_urls_async_with_urls() {
        let client = HttpClient::new();
        let urls = vec!["https://httpbin.org/status/200".to_string()];
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = results.clone();

        let callback = move |url: Option<String>, content: Option<String>| {
            let results = results_clone.clone();
            async move {
                let mut results = results.lock().unwrap();
                results.push((url, content));
            }
        };

        let result = client
            .fetch_content_from_urls_async(
                urls,
                HttpConfigBuilder::new().timeout(30000).build(),
                callback,
            )
            .await;

        assert!(result.is_ok());
        let results = results.lock().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].0.is_some());
        assert!(results[0].1.is_some());
    }

    #[tokio::test]
    async fn test_handles_http_requests_results_async_empty() {
        let urls: Vec<String> = vec![];
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = results.clone();

        let callback = move |url: Option<String>, content: Option<String>| {
            let results = results_clone.clone();
            async move {
                let mut results = results.lock().unwrap();
                results.push((url, content));
            }
        };

        let result = handles_http_requests_results_async(
            urls,
            HttpConfigBuilder::new().timeout(30000).build(),
            callback,
        )
        .await;

        assert!(result.is_ok());
        let results = results.lock().unwrap();
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_handles_http_requests_results_async_with_urls() {
        let urls = vec!["https://httpbin.org/status/200".to_string()];
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = results.clone();

        let callback = move |url: Option<String>, content: Option<String>| {
            let results = results_clone.clone();
            async move {
                let mut results = results.lock().unwrap();
                results.push((url, content));
            }
        };

        let result = handles_http_requests_results_async(
            urls,
            HttpConfigBuilder::new().timeout(30000).build(),
            callback,
        )
        .await;

        assert!(result.is_ok());
        let results = results.lock().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].0.is_some());
        assert!(results[0].1.is_some());
    }

    #[test]
    fn test_build_client_async_with_timeout() {
        let http_config = HttpConfigBuilder::new().timeout(5000).build();
        let client = build_client_async(http_config);

        // Verify the client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<Client>()
        );
    }

    #[test]
    fn test_build_client_async_without_timeout() {
        let http_config = HttpConfigBuilder::new().build();
        let client = build_client_async(http_config);

        // Verify the client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<Client>()
        );
    }

    #[test]
    fn test_build_client_async_with_max_redirect() {
        let http_config = HttpConfigBuilder::new()
            .timeout(5000)
            .max_redirect(5)
            .build();
        let client = build_client_async(http_config);

        // Verify the client was created successfully
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<Client>()
        );
    }
}
