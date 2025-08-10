use rand::prelude::*;

/// Represents different browser user agent strings for web scraping.
///
/// This enum provides a collection of realistic user agent strings from various
/// browsers and operating systems. Using different user agents helps avoid being
/// blocked by websites that restrict automated access.
///
/// # Examples
///
/// ```rust
/// use markdown_harvest::UserAgent;
///
/// // Get a specific user agent
/// let chrome_windows = UserAgent::WindowsChrome;
/// println!("User-Agent: {}", chrome_windows.to_string());
///
/// // Get a random user agent for better diversity
/// let random_agent = UserAgent::random();
/// println!("Random User-Agent: {}", random_agent.to_string());
/// ```
#[derive(Debug, Clone, Copy)]
pub enum UserAgent {
    // Windows
    /// Google Chrome browser on Windows 10/11
    WindowsChrome,
    /// Mozilla Firefox browser on Windows 10/11
    WindowsFirefox,
    /// Microsoft Edge browser on Windows 10/11
    WindowsEdge,

    // macOS
    /// Google Chrome browser on macOS
    MacOSChrome,
    /// Safari browser on macOS
    MacOSSafari,
    /// Mozilla Firefox browser on macOS
    MacOSFirefox,

    // Linux
    /// Google Chrome browser on Linux
    LinuxChrome,
    /// Mozilla Firefox browser on Linux
    LinuxFirefox,

    // Mobile Android
    /// Google Chrome browser on Android devices
    AndroidChrome,
    /// Mozilla Firefox browser on Android devices
    AndroidFirefox,

    // Mobile iOS
    /// Safari browser on iOS devices (iPhone/iPad)
    IOSSafari,
    /// Google Chrome browser on iOS devices (iPhone/iPad)
    IOSChrome,
}

impl UserAgent {
    /// Converts the UserAgent enum variant to its corresponding user agent string.
    ///
    /// Each variant returns a realistic, up-to-date user agent string that mimics
    /// real browsers. These strings include browser version numbers, operating system
    /// details, and rendering engine information.
    ///
    /// # Returns
    ///
    /// A `String` containing the complete user agent string for HTTP headers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_harvest::UserAgent;
    ///
    /// let chrome = UserAgent::WindowsChrome;
    /// let user_agent_string = chrome.to_string();
    /// assert!(user_agent_string.contains("Chrome"));
    /// assert!(user_agent_string.contains("Windows"));
    ///
    /// let firefox = UserAgent::LinuxFirefox;
    /// let user_agent_string = firefox.to_string();
    /// assert!(user_agent_string.contains("Firefox"));
    /// assert!(user_agent_string.contains("Linux"));
    /// ```
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

    /// Returns a random user agent for better web scraping diversity.
    ///
    /// This method selects a random user agent from all available variants to help
    /// avoid detection and blocking by websites. Different user agents simulate
    /// requests from various browsers and operating systems.
    ///
    /// # Returns
    ///
    /// A randomly selected `UserAgent` variant. If random selection fails
    /// (which should never happen), defaults to `UserAgent::LinuxFirefox`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use markdown_harvest::UserAgent;
    ///
    /// // Get different random user agents
    /// let agent1 = UserAgent::random();
    /// let agent2 = UserAgent::random();
    ///
    /// // They might be different (but could be the same due to randomness)
    /// println!("First random agent: {}", agent1.to_string());
    /// println!("Second random agent: {}", agent2.to_string());
    ///
    /// // Use in HTTP request
    /// let random_agent = UserAgent::random();
    /// let user_agent_header = random_agent.to_string();
    /// // Use user_agent_header in your HTTP client...
    /// ```
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
