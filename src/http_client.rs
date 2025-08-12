use crate::user_agent::UserAgent;
use regex::Regex;
use reqwest::blocking::Client;


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

    /// Extracts URLs from text and fetches their content in one operation using blocking HTTP.
    pub fn fetch_content_from_text(&self, text: &str) -> Vec<(String, String)> {
        let urls = self.extract_urls(text);
        if urls.is_empty() {
            return Vec::new();
        }
        self.fetch_content_from_urls(urls)
    }

    fn extract_urls(&self, text: &str) -> Vec<String> {
        let url_regex = Regex::new(r"https?://[a-zA-Z0-9._/%+-]+(?:/[a-zA-Z0-9._/%+-]*)*").unwrap();

        url_regex
            .find_iter(text)
            .map(|m| clean_url(m.as_str()))
            .collect()
    }

    /// Fetches HTML content from a list of URLs using blocking HTTP.
    fn fetch_content_from_urls(&self, urls: Vec<String>) -> Vec<(String, String)> {
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
                    results.push((url.to_string(), html_content));
                    println!("✓ Successfully fetched content from URL: {}", url);
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
        let results = client.fetch_content_from_urls(urls);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_fetch_content_from_text_no_urls() {
        let client = HttpClient::new();
        let text = "This text has no URLs";
        let results = client.fetch_content_from_text(text);
        assert_eq!(results.len(), 0);
    }
}