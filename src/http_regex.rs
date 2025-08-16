use once_cell::sync::Lazy;
use regex::Regex;

pub static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"https?://[a-zA-Z0-9._/%+()-]+(?:/[a-zA-Z0-9._/%+()-]*)*(?:\?[a-zA-Z0-9._/%+()=&-]*)?",
    )
    .unwrap()
});
