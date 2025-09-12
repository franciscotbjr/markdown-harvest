use markdown_harvest::{MarkdownHarvester, HttpConfig};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Markdown Harvest - Asynchronous Chunking Example");
    println!("===================================================");
    println!();
    println!("This example demonstrates the asynchronous get_hyperlinks_content_as_chunks_async function.");
    println!("URLs will be processed in parallel and content split into semantic chunks for RAG systems.");
    println!();

    // Get user input
    let text = get_user_input();

    // Get chunk configuration
    let (chunk_size, chunk_overlap) = get_chunk_config();

    // Configure HTTP settings
    let http_config = HttpConfig::builder()
        .timeout(30000)         // 30 second timeout
        .max_redirect(3)        // Allow up to 3 redirects
        .cookie_store(true)     // Enable cookie storage
        .build();

    println!("\n⚡ Processing URLs and creating chunks asynchronously...");
    println!("📦 Chunk size: {} characters", chunk_size);
    if let Some(overlap) = chunk_overlap {
        println!("🔗 Chunk overlap: {} characters", overlap);
    } else {
        println!("🔗 No chunk overlap");
    }
    println!("🚀 URLs will be processed in parallel - chunks appear as they complete:");
    println!();

    let start_time = std::time::Instant::now();
    let processed_count = Arc::new(Mutex::new(0));
    let total_chunks = Arc::new(Mutex::new(0));
    let processed_count_clone = processed_count.clone();
    let total_chunks_clone = total_chunks.clone();

    // Define the callback for handling chunked results as they arrive
    let callback = move |url: Option<String>, chunks: Option<Vec<String>>| {
        let processed_count = processed_count_clone.clone();
        let total_chunks = total_chunks_clone.clone();
        async move {
            match (url, chunks) {
                (Some(url), Some(chunks)) => {
                    let mut count = processed_count.lock().unwrap();
                    *count += 1;
                    let current_count = *count;
                    drop(count);

                    let mut total = total_chunks.lock().unwrap();
                    *total += chunks.len();
                    let current_total = *total;
                    drop(total);

                    println!("✅ Result #{}: {}", current_count, url);
                    println!("📦 Chunks created: {} (Total so far: {})", chunks.len(), current_total);
                    println!();

                    // Display each chunk with preview
                    for (chunk_idx, chunk) in chunks.iter().enumerate() {
                        println!("   📝 Chunk #{}: {} characters", chunk_idx + 1, chunk.len());
                        
                        // Show chunk preview
                        let preview = if chunk.chars().count() > 120 {
                            let truncated: String = chunk.chars().take(80).collect();
                            format!("{}...", truncated)
                        } else {
                            chunk.clone()
                        };

                        println!("   Content: {}", preview);
                        println!();
                    }
                    
                    println!("{}", "─".repeat(80));
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

    // Process URLs and create chunks asynchronously
    MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
        text, 
        http_config, 
        chunk_size, 
        chunk_overlap,
        callback
    ).await?;

    let duration = start_time.elapsed();
    let final_count = *processed_count.lock().unwrap();
    let final_total_chunks = *total_chunks.lock().unwrap();

    println!("⏱️  Asynchronous chunking completed in {:.2}ms", duration.as_millis());
    println!("📊 Total URLs processed: {}", final_count);
    println!("📦 Total chunks created: {}", final_total_chunks);
    println!("✅ Asynchronous chunking example completed!");
    println!();
    println!("💡 Configuration used:");
    println!("   📦 Chunk size: {} characters", chunk_size);
    if let Some(overlap) = chunk_overlap {
        println!("   🔗 Chunk overlap: {} characters ({:.1}%)", 
                overlap, (overlap as f64 / chunk_size as f64) * 100.0);
    } else {
        println!("   🔗 No chunk overlap");
    }
    println!();
    println!("🚀 Async Chunking Benefits:");
    println!("   • Parallel processing for faster completion with multiple URLs");
    println!("   • Real-time streaming of chunks as they're created");
    println!("   • Immediate processing of chunks for RAG pipeline integration");
    println!("   • Optimal for high-throughput applications and real-time systems");

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

fn get_chunk_config() -> (usize, Option<usize>) {
    println!("📦 Chunk Configuration:");
    println!("Choose a chunk size (recommended sizes):");
    println!("1. 🤖 OpenAI ada-002: 1000 characters");
    println!("2. 🤖 OpenAI text-embedding-3-large: 1500 characters");
    println!("3. 🤖 Cohere embed-multilingual: 2000 characters");
    println!("4. 🤖 Custom size");
    print!("Enter your choice (1-4): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input");

    let chunk_size = match choice.trim() {
        "1" => 1000,
        "2" => 1500,
        "3" => 2000,
        "4" => {
            print!("Enter custom chunk size: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut size = String::new();
            io::stdin().read_line(&mut size).expect("Error reading input");
            size.trim().parse().unwrap_or(1000)
        }
        _ => {
            println!("Invalid choice, using default: 1000");
            1000
        }
    };

    println!("\n🔗 Chunk Overlap Configuration:");
    println!("Do you want chunk overlap for better context preservation?");
    println!("1. No overlap (faster processing)");
    println!("2. 10% overlap (recommended for most cases)");
    println!("3. 20% overlap (better context, larger storage)");
    println!("4. Custom overlap");
    print!("Enter your choice (1-4): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut overlap_choice = String::new();
    io::stdin().read_line(&mut overlap_choice).expect("Error reading input");

    let chunk_overlap = match overlap_choice.trim() {
        "1" => None,
        "2" => Some(chunk_size / 10), // 10% overlap
        "3" => Some(chunk_size / 5),  // 20% overlap
        "4" => {
            print!("Enter custom overlap size: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut overlap = String::new();
            io::stdin().read_line(&mut overlap).expect("Error reading input");
            overlap.trim().parse().ok()
        }
        _ => {
            println!("Invalid choice, using no overlap");
            None
        }
    };

    (chunk_size, chunk_overlap)
}