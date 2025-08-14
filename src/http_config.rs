#[derive(Default, Clone, Copy)]
pub struct HttpConfig {
    timeout: Option<u64>,
    max_redirect: Option<usize>,
    cookie_store: bool,
}

#[derive(Default)]
pub struct HttpConfigBuilder {
    timeout: Option<u64>,
    max_redirect: Option<usize>,
    cookie_store: bool,
}

impl HttpConfigBuilder {
    pub fn new() -> Self {
        Self {
            timeout: None,
            max_redirect: None,
            cookie_store: false,
        }
    }

    pub fn timeout(mut self, ms: u64) -> Self {
        self.timeout = Some(ms);
        self
    }

    pub fn max_redirect(mut self, max_redirect: usize) -> Self {
        self.max_redirect = Some(max_redirect);
        self
    }

    pub fn cookie_store(mut self, cookie_store: bool) -> Self {
        self.cookie_store = cookie_store;
        self
    }

    pub fn build(self) -> HttpConfig {
        HttpConfig {
            timeout: self.timeout,
            max_redirect: self.max_redirect,
            cookie_store: self.cookie_store,
        }
    }
}

impl HttpConfig {
    fn new(timeout: Option<u64>, max_redirect: Option<usize>, cookie_store: bool) -> Self {
        Self {
            timeout,
            max_redirect,
            cookie_store,
        }
    }

    pub fn builder() -> HttpConfigBuilder {
        HttpConfigBuilder::new()
    }

    pub fn timeout(&self) -> Option<u64> {
        self.timeout
    }

    pub fn max_redirect(&self) -> Option<usize> {
        self.max_redirect
    }

    pub fn cookie_store(&self) -> bool {
        self.cookie_store
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_builder_new() {
        let builder = HttpConfigBuilder::new();
        assert_eq!(builder.timeout, None);
        assert_eq!(builder.max_redirect, None);
        assert_eq!(builder.cookie_store, false);
    }

    #[test]
    fn test_http_config_builder_default() {
        let builder = HttpConfigBuilder::default();
        assert_eq!(builder.timeout, None);
        assert_eq!(builder.max_redirect, None);
        assert_eq!(builder.cookie_store, false);
    }

    #[test]
    fn test_http_config_builder_timeout() {
        let builder = HttpConfigBuilder::new().timeout(5000);
        assert_eq!(builder.timeout, Some(5000));
        assert_eq!(builder.max_redirect, None);
        assert_eq!(builder.cookie_store, false);
    }

    #[test]
    fn test_http_config_builder_max_redirect() {
        let builder = HttpConfigBuilder::new().max_redirect(10);
        assert_eq!(builder.timeout, None);
        assert_eq!(builder.max_redirect, Some(10));
        assert_eq!(builder.cookie_store, false);
    }

    #[test]
    fn test_http_config_builder_cookie_store() {
        let builder = HttpConfigBuilder::new().cookie_store(true);
        assert_eq!(builder.timeout, None);
        assert_eq!(builder.max_redirect, None);
        assert_eq!(builder.cookie_store, true);
    }

    #[test]
    fn test_http_config_builder_fluent_api() {
        let builder = HttpConfigBuilder::new()
            .timeout(3000)
            .max_redirect(5)
            .cookie_store(true);

        assert_eq!(builder.timeout, Some(3000));
        assert_eq!(builder.max_redirect, Some(5));
        assert_eq!(builder.cookie_store, true);
    }

    #[test]
    fn test_http_config_builder_build() {
        let config = HttpConfigBuilder::new()
            .timeout(2500)
            .max_redirect(8)
            .cookie_store(false)
            .build();

        assert_eq!(config.timeout(), Some(2500));
        assert_eq!(config.max_redirect(), Some(8));
        assert_eq!(config.cookie_store(), false);
    }

    #[test]
    fn test_http_config_builder_build_empty() {
        let config = HttpConfigBuilder::new().build();

        assert_eq!(config.timeout(), None);
        assert_eq!(config.max_redirect(), None);
        assert_eq!(config.cookie_store(), false);
    }

    #[test]
    fn test_http_config_default() {
        let config = HttpConfig::default();

        assert_eq!(config.timeout(), None);
        assert_eq!(config.max_redirect(), None);
        assert_eq!(config.cookie_store(), false);
    }

    #[test]
    fn test_http_config_builder_static_method() {
        let config = HttpConfig::builder()
            .timeout(1000)
            .max_redirect(3)
            .cookie_store(true)
            .build();

        assert_eq!(config.timeout(), Some(1000));
        assert_eq!(config.max_redirect(), Some(3));
        assert_eq!(config.cookie_store(), true);
    }

    #[test]
    fn test_http_config_getters() {
        let config = HttpConfig {
            timeout: Some(4000),
            max_redirect: Some(7),
            cookie_store: true,
        };

        assert_eq!(config.timeout(), Some(4000));
        assert_eq!(config.max_redirect(), Some(7));
        assert_eq!(config.cookie_store(), true);
    }

    #[test]
    fn test_http_config_clone() {
        let original = HttpConfig::builder()
            .timeout(1500)
            .max_redirect(4)
            .cookie_store(true)
            .build();

        let cloned = original.clone();

        assert_eq!(original.timeout(), cloned.timeout());
        assert_eq!(original.max_redirect(), cloned.max_redirect());
        assert_eq!(original.cookie_store(), cloned.cookie_store());
    }

    #[test]
    fn test_http_config_copy() {
        let original = HttpConfig::builder()
            .timeout(2000)
            .max_redirect(6)
            .cookie_store(false)
            .build();

        let copied = original;

        assert_eq!(original.timeout(), copied.timeout());
        assert_eq!(original.max_redirect(), copied.max_redirect());
        assert_eq!(original.cookie_store(), copied.cookie_store());
    }

    #[test]
    fn test_http_config_builder_chaining_order() {
        // Test different chaining orders produce same result
        let config1 = HttpConfig::builder()
            .timeout(1000)
            .max_redirect(5)
            .cookie_store(true)
            .build();

        let config2 = HttpConfig::builder()
            .cookie_store(true)
            .timeout(1000)
            .max_redirect(5)
            .build();

        assert_eq!(config1.timeout(), config2.timeout());
        assert_eq!(config1.max_redirect(), config2.max_redirect());
        assert_eq!(config1.cookie_store(), config2.cookie_store());
    }

    #[test]
    fn test_http_config_builder_overwrite() {
        // Test that later values overwrite earlier ones
        let config = HttpConfig::builder()
            .timeout(1000)
            .timeout(2000) // This should overwrite the previous timeout
            .max_redirect(3)
            .max_redirect(6) // This should overwrite the previous max_redirect
            .cookie_store(false)
            .cookie_store(true) // This should overwrite the previous cookie_store
            .build();

        assert_eq!(config.timeout(), Some(2000));
        assert_eq!(config.max_redirect(), Some(6));
        assert_eq!(config.cookie_store(), true);
    }

    #[test]
    fn test_http_config_edge_values() {
        // Test edge values
        let config = HttpConfig::builder()
            .timeout(0) // Minimum timeout
            .max_redirect(0) // Minimum redirects
            .cookie_store(false)
            .build();

        assert_eq!(config.timeout(), Some(0));
        assert_eq!(config.max_redirect(), Some(0));
        assert_eq!(config.cookie_store(), false);

        let config2 = HttpConfig::builder()
            .timeout(u64::MAX) // Maximum timeout
            .max_redirect(usize::MAX) // Maximum redirects
            .cookie_store(true)
            .build();

        assert_eq!(config2.timeout(), Some(u64::MAX));
        assert_eq!(config2.max_redirect(), Some(usize::MAX));
        assert_eq!(config2.cookie_store(), true);
    }

    #[test]
    fn test_http_config_partial_configuration() {
        // Test partial configurations
        let config1 = HttpConfig::builder().timeout(1000).build();
        assert_eq!(config1.timeout(), Some(1000));
        assert_eq!(config1.max_redirect(), None);
        assert_eq!(config1.cookie_store(), false);

        let config2 = HttpConfig::builder().max_redirect(5).build();
        assert_eq!(config2.timeout(), None);
        assert_eq!(config2.max_redirect(), Some(5));
        assert_eq!(config2.cookie_store(), false);

        let config3 = HttpConfig::builder().cookie_store(true).build();
        assert_eq!(config3.timeout(), None);
        assert_eq!(config3.max_redirect(), None);
        assert_eq!(config3.cookie_store(), true);
    }
}
