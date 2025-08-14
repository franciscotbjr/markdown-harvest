mod content_processor;
mod http_client;
mod http_config;
mod markdown_harvester;
mod patterns;
mod user_agent;

use http_config::HttpConfig;
use markdown_harvester::MarkdownHarvester;
use std::io::stdin;

fn main() {
    // Read text from standard input
    println!("\nEnter the message:");
    let mut text = String::new();
    stdin().read_line(&mut text).expect("Error reading input");

    let http_config = HttpConfig::builder()
        .timeout(30000)
        .max_redirect(3)
        .cookie_store(true)
        .build();

    let results = MarkdownHarvester::get_hyperlinks_content(text, http_config);

    // Display summary of stored results
    println!("\nSummary: {} URL(s) processed", results.len());
    for (url, content) in &results {
        println!("✓ {}", url);
        println!("📄 Markdown content preview:");
        let preview = if content.chars().count() > 200 {
            let truncated: String = content.chars().take(200).collect();
            format!("{}...", truncated)
        } else {
            content.clone()
        };
        println!("{}", preview);
        println!("---");
    }
}
