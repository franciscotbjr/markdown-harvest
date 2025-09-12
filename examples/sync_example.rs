use markdown_harvest::{MarkdownHarvester, HttpConfig};
use std::io::{self, Write};

fn main() {
    println!("🦀 Markdown Harvest - Synchronous Processing Example");
    println!("===================================================");
    println!();
    println!("This example demonstrates the synchronous get_hyperlinks_content function.");
    println!("URLs will be processed sequentially, returning all results together.");
    println!();

    // Get user input
    let text = get_user_input();

    // Configure HTTP settings
    let http_config = HttpConfig::builder()
        .timeout(30000)         // 30 second timeout
        .max_redirect(3)        // Allow up to 3 redirects
        .cookie_store(true)     // Enable cookie storage
        .build();

    println!("\n🔄 Processing URLs synchronously...");
    println!("⏳ Please wait while all URLs are processed sequentially...");
    println!();

    let start_time = std::time::Instant::now();
    
    // Process URLs synchronously
    let results = MarkdownHarvester::get_hyperlinks_content(text, http_config);
    
    let duration = start_time.elapsed();

    // Display results
    display_results(&results, duration);
}

fn get_user_input() -> String {
    println!("Enter text containing URLs to extract content from:");
    println!("Example: Check out https://example.com and https://httpbin.org/json");
    println!("Or try: Visit https://www.rust-lang.org for more information");
    print!("Your text: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error reading input");
    
    text
}

fn display_results(results: &[(String, String)], duration: std::time::Duration) {
    println!("⏱️  Synchronous processing completed in {:.2}ms", duration.as_millis());
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

        println!("📝 Markdown content:");
        println!("{}", preview);
        println!("{}", "─".repeat(60));
    }

    println!();
    println!("✅ Synchronous processing example completed!");
    println!();
    println!("💡 Key characteristics of synchronous processing:");
    println!("   • URLs are processed one by one in sequence");
    println!("   • All results are returned together after processing completes");
    println!("   • Simple and straightforward - good for basic use cases");
    println!("   • Memory efficient for smaller numbers of URLs");
}