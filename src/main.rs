mod content_processor;
mod http_client;
mod http_config;
mod http_regex;
mod markdown_harvester;
mod patterns;
mod user_agent;

use http_config::HttpConfig;
use markdown_harvester::MarkdownHarvester;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

fn main() {
    println!("🦀 Markdown Harvest - URL Content Extractor");
    println!("===========================================");
    println!();

    // Get user input
    let text = get_user_input();

    // Show available options
    println!("\nChoose processing mode:");
    println!("1. 🔄 Synchronous (sequential processing)");
    println!("2. ⚡ Asynchronous (parallel processing)");
    print!("Enter your choice (1 or 2): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error reading input");

    let http_config = HttpConfig::builder()
        .timeout(30000)
        .max_redirect(3)
        .cookie_store(true)
        .build();

    match choice.trim() {
        "1" => {
            println!("\n🔄 Running SYNCHRONOUS example...");
            run_synchronous_example(&text, http_config);
        }
        "2" => {
            println!("\n⚡ Running ASYNCHRONOUS example...");
            run_asynchronous_example(&text, http_config);
        }
        _ => {
            println!("\n❌ Invalid choice. Running both examples for demonstration:");
            println!("\n🔄 SYNCHRONOUS Example:");
            run_synchronous_example(&text, http_config.clone());
            println!("\n⚡ ASYNCHRONOUS Example:");
            run_asynchronous_example(&text, http_config);
        }
    }
}

fn get_user_input() -> String {
    println!("Enter text containing URLs to extract content from:");
    println!("Example: Check out https://example.com and https://httpbin.org/json");
    print!("Your text: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error reading input");
    text
}

/// SYNCHRONOUS EXAMPLE - Traditional blocking approach
/// Perfect for simple use cases and sequential processing
fn run_synchronous_example(text: &str, http_config: HttpConfig) {
    println!("📚 How to use SYNCHRONOUS processing:");
    println!("```rust");
    println!("use markdown_harvest::{{MarkdownHarvester, HttpConfig}};");
    println!();
    println!("let http_config = HttpConfig::builder()");
    println!("    .timeout(30000)");
    println!("    .max_redirect(3)");
    println!("    .cookie_store(true)");
    println!("    .build();");
    println!();
    println!("let results = MarkdownHarvester::get_hyperlinks_content(text, http_config);");
    println!("```");
    println!();

    let start_time = std::time::Instant::now();
    let results = MarkdownHarvester::get_hyperlinks_content(text.to_string(), http_config);
    let duration = start_time.elapsed();

    display_results(&results, duration, "SYNCHRONOUS");
}

/// ASYNCHRONOUS EXAMPLE - Non-blocking parallel approach  
/// Best for performance when processing multiple URLs
fn run_asynchronous_example(text: &str, http_config: HttpConfig) {
    println!("📚 How to use ASYNCHRONOUS processing:");
    println!("```rust");
    println!("use markdown_harvest::{{MarkdownHarvester, HttpConfig}};");
    println!("use std::sync::{{Arc, Mutex}};");
    println!();
    println!("#[tokio::main]");
    println!("async fn main() {{");
    println!("    let results = Arc::new(Mutex::new(Vec::new()));");
    println!("    let results_clone = results.clone();");
    println!();
    println!("    let callback = move |url: Option<String>, content: Option<String>| {{");
    println!("        let results = results_clone.clone();");
    println!("        async move {{");
    println!("            if let (Some(url), Some(content)) = (url, content) {{");
    println!("                let mut results = results.lock().unwrap();");
    println!("                results.push((url, content));");
    println!("            }}");
    println!("        }}");
    println!("    }};");
    println!();
    println!(
        "    MarkdownHarvester::get_hyperlinks_content_async(text, http_config, callback).await?;"
    );
    println!("}}");
    println!("```");
    println!();

    // Create a tokio runtime for the async example
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    println!("🚀 Starting asynchronous processing...");
    println!("⏳ Processing URLs in parallel - results will appear as they complete:");
    println!();

    let start_time = std::time::Instant::now();
    let processed_count = Arc::new(Mutex::new(0));
    let processed_count_clone = processed_count.clone();

    rt.block_on(async {
        let callback = move |url: Option<String>, content: Option<String>| {
            let processed_count = processed_count_clone.clone();
            async move {
                match (url, content) {
                    (Some(url), Some(content)) => {
                        let mut count = processed_count.lock().unwrap();
                        *count += 1;
                        let current_count = *count;
                        drop(count); // Release lock early

                        println!("✅ Result #{}: {}", current_count, url);

                        // Show content preview
                        let preview = if content.chars().count() > 200 {
                            let truncated: String = content.chars().take(150).collect();
                            format!(
                                "{}...\n\n📏 [Content truncated - {} total characters]",
                                truncated,
                                content.chars().count()
                            )
                        } else {
                            content.clone()
                        };

                        println!("📝 Markdown content:");
                        println!("{}", preview);
                        println!("{}", "─".repeat(60));
                        println!();
                    }
                    (None, None) => {
                        println!("ℹ️  No URLs found in the provided text");
                        println!();
                    }
                    _ => unreachable!(),
                }
            }
        };

        let _ = MarkdownHarvester::get_hyperlinks_content_async(
            text.to_string(),
            http_config,
            callback,
        )
        .await;
    });

    let duration = start_time.elapsed();
    let final_count = *processed_count.lock().unwrap();

    println!(
        "⏱️  ASYNCHRONOUS processing completed in {:.2}ms",
        duration.as_millis()
    );
    println!("📊 Total URLs processed: {}", final_count);
    println!("✅ ASYNCHRONOUS processing demonstration completed!");
    println!();
}

fn display_results(results: &[(String, String)], duration: std::time::Duration, mode: &str) {
    println!(
        "⏱️  {} processing completed in {:.2}ms",
        mode,
        duration.as_millis()
    );
    println!("📊 Summary: {} URL(s) processed", results.len());
    println!();

    if results.is_empty() {
        println!("ℹ️  No URLs found in the provided text.");
        println!("💡 Try entering text with URLs like: https://example.com");
        return;
    }

    for (i, (url, content)) in results.iter().enumerate() {
        println!("📄 Result #{}: {}", i + 1, url);

        // Show content preview
        let preview = if content.chars().count() > 300 {
            let truncated: String = content.chars().take(200).collect();
            format!(
                "{}...\n\n[Content truncated - {} total characters]",
                truncated,
                content.chars().count()
            )
        } else {
            content.clone()
        };

        println!("📝 Markdown content preview:");
        println!("{}", preview);
        println!("{}", "─".repeat(60));
    }

    println!();
    println!("✅ {} processing demonstration completed!", mode);
}
