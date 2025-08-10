/// Returns regex patterns for removing media elements from HTML content.
///
/// This function provides patterns to identify and remove non-textual elements
/// like images, videos, iframes, and other media that don't contribute to the
/// readable text content. These elements are typically not useful for text
/// extraction and Markdown conversion.
///
/// # Returns
///
/// An array of regex pattern strings that match various media elements.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::media_elements;
/// use regex::Regex;
///
/// let patterns = media_elements();
/// assert_eq!(patterns.len(), 9);
///
/// // Example usage in HTML cleaning
/// let mut html = r#"<div>Text content <img src="image.jpg"> more text</div>"#.to_string();
/// for pattern in patterns.iter() {
///     let regex = Regex::new(pattern).unwrap();
///     html = regex.replace_all(&html, "").to_string();
/// }
/// assert!(!html.contains("<img"));
/// ```
pub fn media_elements() -> [&'static str; 9] {
    [
        r"(?i)<img[^>]*>",
        r"(?i)<iframe[^>]*>[\s\S]*?</iframe>",
        r"(?i)<iframe[^>]*/>",
        r"(?i)<video[^>]*>[\s\S]*?</video>",
        r"(?i)<audio[^>]*>[\s\S]*?</audio>",
        r"(?i)<canvas[^>]*>[\s\S]*?</canvas>",
        r"(?i)<svg[^>]*>[\s\S]*?</svg>",
        r"(?i)<embed[^>]*>",
        r"(?i)<object[^>]*>[\s\S]*?</object>",
    ]
}

/// Returns regex patterns for removing unwanted structural elements from HTML.
///
/// This function provides patterns to identify and remove navigation bars, headers,
/// footers, sidebars, advertisements, and other structural elements that don't
/// contain the main content. These elements often contain navigation links,
/// promotional content, or metadata that shouldn't be included in the extracted text.
///
/// # Returns
///
/// An array of regex pattern strings that match unwanted structural elements.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::unwanted_elements;
/// use regex::Regex;
///
/// let patterns = unwanted_elements();
/// assert_eq!(patterns.len(), 7);
///
/// // Example usage
/// let mut html = r#"<nav>Navigation</nav><div>Main content</div><footer>Footer</footer>"#.to_string();
/// for pattern in patterns.iter() {
///     let regex = Regex::new(pattern).unwrap();
///     html = regex.replace_all(&html, "").to_string();
/// }
/// assert!(!html.contains("<nav>"));
/// assert!(!html.contains("<footer>"));
/// assert!(html.contains("Main content"));
/// ```
pub fn unwanted_elements() -> [&'static str; 7] {
    [
        r"(?i)<nav[^>]*>[\s\S]*?</nav>",
        r"(?i)<header[^>]*>[\s\S]*?</header>",
        r"(?i)<footer[^>]*>[\s\S]*?</footer>",
        r"(?i)<aside[^>]*>[\s\S]*?</aside>",
        r"(?i)<div[^>]*class=[^>]*(?:nav|menu|sidebar|advertisement|ad|sponsor|cookie|privacy|social|share|comment|related|avatar|wp-image)[^>]*>[\s\S]*?</div>",
        r"(?i)<div[^>]*id=[^>]*(?:nav|menu|sidebar|advertisement|ad|sponsor|cookie|privacy|social|share|comment|related)[^>]*>[\s\S]*?</div>",
        r"(?i)<section[^>]*class=[^>]*(?:nav|menu|sidebar|advertisement|ad|sponsor|cookie|privacy|social|share|comment|related)[^>]*>[\s\S]*?</section>",
    ]
}

/// Returns CSS selectors for identifying main content areas in HTML.
///
/// This function provides CSS selector strings that target elements likely to
/// contain the main content of a webpage. These selectors help identify article
/// content, main sections, and other content-rich areas while avoiding
/// navigation and sidebar content.
///
/// # Returns
///
/// An array of CSS selector strings for content identification.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::content_selectors;
/// use scraper::{Html, Selector};
///
/// let selectors = content_selectors();
/// assert_eq!(selectors.len(), 7);
/// assert!(selectors.contains(&"article"));
/// assert!(selectors.contains(&"main"));
///
/// // Example usage with scraper
/// let html = Html::parse_document(r#"<article><h1>Title</h1><p>Content</p></article>"#);
/// let article_selector = Selector::parse("article").unwrap();
/// let articles: Vec<_> = html.select(&article_selector).collect();
/// assert_eq!(articles.len(), 1);
/// ```
pub fn content_selectors() -> [&'static str; 7] {
    [
        "article",
        "main",
        "[role='main']",
        ".content",
        ".article",
        ".post",
        ".entry",
    ]
}

/// Returns CSS selectors for identifying text content elements in HTML.
///
/// This function provides CSS selector strings that target specific HTML elements
/// containing readable text content such as headings, paragraphs, lists, quotes,
/// and tables. These selectors are used as a fallback when main content areas
/// cannot be identified through content selectors.
///
/// # Returns
///
/// An array of CSS selector strings for text elements.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::text_selectors;
/// use scraper::{Html, Selector};
///
/// let selectors = text_selectors();
/// assert_eq!(selectors.len(), 6);
/// assert!(selectors.contains(&"p"));
/// assert!(selectors.contains(&"h1, h2, h3, h4, h5, h6"));
///
/// // Example usage
/// let html = Html::parse_document(r#"<div><h1>Title</h1><p>Paragraph</p><ul><li>Item</li></ul></div>"#);
/// let p_selector = Selector::parse("p").unwrap();
/// let paragraphs: Vec<_> = html.select(&p_selector).collect();
/// assert_eq!(paragraphs.len(), 1);
/// ```
pub fn text_selectors() -> [&'static str; 6] {
    [
        "h1, h2, h3, h4, h5, h6",
        "p",
        "ul, ol",
        "blockquote",
        "pre",
        "table",
    ]
}

/// Returns regex patterns for final cleanup of HTML elements before Markdown conversion.
///
/// This function provides patterns for removing specific unwanted elements that
/// might have survived the initial cleaning passes. These patterns target elements
/// with specific classes or styles that are typically used for non-content purposes
/// like user avatars, social sharing buttons, and hidden elements.
///
/// # Returns
///
/// An array of regex pattern strings for final cleanup.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::additional_cleanup;
/// use regex::Regex;
///
/// let patterns = additional_cleanup();
/// assert_eq!(patterns.len(), 3);
///
/// // Example usage
/// let mut html = r#"<div>Content</div><a class="avatar">Avatar</a>"#.to_string();
/// for pattern in patterns.iter() {
///     let regex = Regex::new(pattern).unwrap();
///     html = regex.replace_all(&html, "").to_string();
/// }
/// assert!(!html.contains("Avatar"));
/// assert!(html.contains("Content"));
/// ```
pub fn additional_cleanup() -> [&'static str; 3] {
    [
        r"(?i)<a[^>]*class=[^>]*(?:avatar|wp-image|button|btn)[^>]*>[\s\S]*?</a>",
        r"(?i)<span[^>]*class=[^>]*(?:avatar|wp-image|social|share)[^>]*>[\s\S]*?</span>",
        r"(?i)<div[^>]*style=[^>]*(?:display:\s*none|visibility:\s*hidden)[^>]*>[\s\S]*?</div>",
    ]
}

/// Returns regex patterns for removing unwanted text patterns from content.
///
/// This function provides patterns to identify and remove common advertising,
/// navigation, and promotional text that often appears in web content. These
/// patterns help clean up the final text by removing phrases like "subscribe",
/// "follow us", "advertisement", and other non-content text while preserving
/// the main article content.
///
/// # Returns
///
/// An array of regex pattern strings for text cleanup.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::unwanted_text_patterns;
/// use regex::Regex;
///
/// let patterns = unwanted_text_patterns();
/// assert_eq!(patterns.len(), 7);
///
/// // Example usage
/// let mut text = "This is content. Subscribe to our newsletter. More content here.".to_string();
/// for pattern in patterns.iter() {
///     let regex = Regex::new(pattern).unwrap();
///     text = regex.replace_all(&text, "").to_string();
/// }
/// assert!(text.contains("This is content"));
/// assert!(text.contains("More content"));
/// assert!(!text.contains("Subscribe"));
/// ```
pub fn unwanted_text_patterns() -> [&'static str; 7] {
    [
        r"(?i)\b(advertisement|sponsored|cookie policy|privacy policy|terms of service|subscribe|newsletter|follow us|share this|related articles|recommended|créditos|tópicos|inscreva-se|mantenha-se informado|acesso livre|editor/a)\b",
        r"(?i)\bclick here\b",
        r"(?i)\bread more\b",
        r"(?i)\bsee also\b",
        r"(?i)\bver tópicos\b",
        r"(?i)\bimagem do banner\b",
        r"(?i)\bfoto:\b.*$",
    ]
}
