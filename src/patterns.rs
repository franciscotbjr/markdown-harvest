// Applied to temove images, iframes, and other non-textual elements
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

// Applied to remove navigation, header, footer, sidebar and advertising elements
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

// Applied to select only content-relevant elements and extract their inner HTML
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

// Applied to find revelevant content
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

// Applied to do additional cleanup before markdown conversion - remove remaining unwanted elements
pub fn additional_cleanup() -> [&'static str; 3] {
    [
        r"(?i)<a[^>]*class=[^>]*(?:avatar|wp-image|button|btn)[^>]*>[\s\S]*?</a>",
        r"(?i)<span[^>]*class=[^>]*(?:avatar|wp-image|social|share)[^>]*>[\s\S]*?</span>",
        r"(?i)<div[^>]*style=[^>]*(?:display:\s*none|visibility:\s*hidden)[^>]*>[\s\S]*?</div>",
    ]
}

// Applied to remove common advertising/navigation text patterns but preserve line structure
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
