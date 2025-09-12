use markdown_harvest::{MarkdownHarvester, HttpConfig};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Markdown Harvest - Asynchronous Processing Example");
    println!("====================================================");
    println!();
    println!("This example demonstrates the asynchronous get_hyperlinks_content_async function.");
    println!("URLs will be processed in parallel, with results streaming as they complete.");
    println!();

    // Get user input
    let text = get_user_input();

    // Configure HTTP settings
    let http_config = HttpConfig::builder()
        .timeout(30000)         // 30 second timeout
        .max_redirect(3)        // Allow up to 3 redirects
        .cookie_store(true)     // Enable cookie storage
        .build();

    println!("\n⚡ Processing URLs asynchronously...");
    println!("🚀 URLs will be processed in parallel - results appear as they complete:");
    println!();

    let start_time = std::time::Instant::now();
    let processed_count = Arc::new(Mutex::new(0));
    let processed_count_clone = processed_count.clone();

    // Define the callback for handling results as they arrive
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
                    println!("💡 Try entering text with URLs like: https://example.com");
                    println!();
                }
                _ => unreachable!(),
            }
        }
    };

    // Process URLs asynchronously
    MarkdownHarvester::get_hyperlinks_content_async(text, http_config, callback).await?;

    let duration = start_time.elapsed();
    let final_count = *processed_count.lock().unwrap();

    println!("⏱️  Asynchronous processing completed in {:.2}ms", duration.as_millis());
    println!("📊 Total URLs processed: {}", final_count);
    println!("✅ Asynchronous processing example completed!");
    println!();
    println!("💡 Key characteristics of asynchronous processing:");
    println!("   • URLs are processed concurrently in parallel");
    println!("   • Results stream in real-time as each URL completes");
    println!("   • Better performance for multiple URLs");
    println!("   • Ideal for real-time applications and high throughput");

    Ok(())
}

fn get_user_input() -> String {
    println!("Enter text containing URLs to extract content from:");
    println!("Example: Check out https://example.com and https://httpbin.org/json");
    println!("Or try: Visit https://www.rust-lang.org and https://github.com");
    print!("Your text: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error reading input");
    
    text
}