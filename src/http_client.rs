use crate::{http_config::HttpConfig, user_agent::UserAgent};
use regex::Regex;
use reqwest::blocking::Client;
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

    fn extract_urls(&self, text: &str) -> Vec<String> {
        let url_regex = Regex::new(r"https?://[a-zA-Z0-9._/%+-]+(?:/[a-zA-Z0-9._/%+-]*)*").unwrap();

        url_regex
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
}

fn handles_http_requests_results(
    urls: Vec<String>,
    http_config: HttpConfig,
) -> Vec<(String, String)> {
    let client = match http_config.timeout() {
        Some(timeout) => Client::builder()
            .timeout(Duration::from_millis(timeout))
            .redirect(reqwest::redirect::Policy::limited(
                http_config.max_redirect().unwrap_or(2),
            ))
            .cookie_store(http_config.cookie_store())
            .build()
            .unwrap_or_else(|_| Client::new()),
        None => Client::new(),
    };
    let mut results = Vec::new();

    for url in &urls {
        let user_agent = UserAgent::random();
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

fn clean_url(url: &str) -> String {
    // Remove common punctuation at the end of URLs
    url.trim_end_matches(&['.', ',', ';', '!', '?', ')', ']', '}'][..])
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::http_config::HttpConfigBuilder;

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
    fn test_clean_url() {
        assert_eq!(clean_url("https://example.com."), "https://example.com");
        assert_eq!(clean_url("https://example.com,"), "https://example.com");
        assert_eq!(clean_url("https://example.com!"), "https://example.com");
        assert_eq!(clean_url("https://example.com"), "https://example.com");
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
}
