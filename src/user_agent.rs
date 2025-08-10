use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum UserAgent {
    // Windows
    WindowsChrome,
    WindowsFirefox,
    WindowsEdge,

    // macOS
    MacOSChrome,
    MacOSSafari,
    MacOSFirefox,

    // Linux
    LinuxChrome,
    LinuxFirefox,

    // Mobile Android
    AndroidChrome,
    AndroidFirefox,

    // Mobile iOS
    IOSSafari,
    IOSChrome,
}

impl UserAgent {
    pub fn to_string(&self) -> String {
        match self {
            // Windows User Agents
            UserAgent::WindowsChrome => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            UserAgent::WindowsFirefox => "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            UserAgent::WindowsEdge => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0".to_string(),
            // macOS User Agents
            UserAgent::MacOSChrome => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            UserAgent::MacOSSafari => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15".to_string(),
            UserAgent::MacOSFirefox => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            // Linux User Agents
            UserAgent::LinuxChrome => "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            UserAgent::LinuxFirefox => "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            // Android User Agents
            UserAgent::AndroidChrome => "Mozilla/5.0 (Linux; Android 14; SM-G991B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36".to_string(),
            UserAgent::AndroidFirefox => "Mozilla/5.0 (Mobile; rv:121.0) Gecko/121.0 Firefox/121.0".to_string(),
            // iOS User Agents
            UserAgent::IOSSafari => "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Mobile/15E148 Safari/604.1".to_string(),
            UserAgent::IOSChrome => "Mozilla/5.0 (iPhone; CPU iPhone OS 17_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/120.0.0.0 Mobile/15E148 Safari/604.1".to_string(),
        }
    }

    /// Returns a random user agent for better web scraping diversity
    pub fn random() -> UserAgent {
        let agents = [
            UserAgent::WindowsChrome,
            UserAgent::WindowsFirefox,
            UserAgent::WindowsEdge,
            UserAgent::MacOSChrome,
            UserAgent::MacOSSafari,
            UserAgent::MacOSFirefox,
            UserAgent::LinuxChrome,
            UserAgent::LinuxFirefox,
            UserAgent::AndroidChrome,
            UserAgent::AndroidFirefox,
            UserAgent::IOSSafari,
            UserAgent::IOSChrome,
        ];

        *agents
            .choose(&mut rand::rng())
            .unwrap_or(&UserAgent::LinuxFirefox)
    }
}
