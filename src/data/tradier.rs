use secrecy::SecretString;
use std::borrow::Cow;
use std::env;

/// Configuration for the Tradier REST API endpoint and access token.
///
/// `TradierRestApiConfig` allows setting an endpoint as either a static reference (`&'static str`)
/// or a dynamically allocated string (`String`). The configuration is flexible, with the endpoint
/// defaulting to a static value and an access token securely stored using `SecretString`.
///
/// # Examples
///
/// ```rust
/// use blue_jacket::data::tradier::TradierRestApiConfig;
/// use std::env;
///
/// // Set the environment variable for demonstration purposes
/// env::set_var("TRADIER_API_ACCESS_TOKEN", "test_token");
///
/// # tokio_test::block_on(async {
/// // Load the configuration from environment variables
/// let config = TradierRestApiConfig::load_from_env().await.expect("caller to handle errors");
/// assert_eq!(config.endpoint, "https://sandbox.tradier.com/v1/");
/// # });
/// ```
#[derive(Debug)]
pub struct TradierRestApiConfig {
    pub endpoint: Cow<'static, str>,
    pub access_token: SecretString,
}

impl TradierRestApiConfig {
    /// Loads the configuration from environment variables with default endpoint.
    ///
    /// This method expects `TRADIER_API_ACCESS_TOKEN` to be set in the environment and defaults the
    /// endpoint to `https://sandbox.tradier.com/v1/`.
    ///
    /// # Errors
    ///
    /// Returns an error if `TRADIER_API_ACCESS_TOKEN` is not found in the environment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blue_jacket::data::tradier::TradierRestApiConfig;
    /// use std::env;
    ///
    /// env::set_var("TRADIER_API_ACCESS_TOKEN", "test_token");
    ///
    /// # tokio_test::block_on(async {
    /// let config = TradierRestApiConfig::load_from_env().await.expect("caller to handle errors");
    /// assert_eq!(config.endpoint, "https://sandbox.tradier.com/v1/");
    /// # });
    /// ```
    pub async fn load_from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok(); // Load from .env in development

        let access_token = env::var("TRADIER_API_ACCESS_TOKEN")?;
        Ok(Self::new(
            Cow::Borrowed("https://sandbox.tradier.com/v1/"),
            SecretString::new(access_token.into()),
        ))
    }

    /// Creates a new `TradierRestApiConfig` with the given endpoint and access token.
    ///
    /// This constructor is intended for use in scenarios where the configuration might be manually set,
    /// bypassing the need for environment variables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use blue_jacket::data::tradier::TradierRestApiConfig;
    /// use secrecy::SecretString;
    /// use std::borrow::Cow;
    ///
    /// let config = TradierRestApiConfig::new(
    ///     Cow::Borrowed("https://api.tradier.com/v1/"),
    ///     SecretString::new("test_token".into()),
    /// );
    /// assert_eq!(config.endpoint, "https://api.tradier.com/v1/");
    /// ```
    pub fn new(endpoint: Cow<'static, str>, access_token: SecretString) -> Self {
        Self { endpoint, access_token }
    }
}

#[cfg(test)]
mod tests {
    use super::TradierRestApiConfig;
    use secrecy::{SecretString, ExposeSecret};
    use std::borrow::Cow;
    use std::env;

    #[tokio::test]
    async fn new_config_with_custom_endpoint() {
        // Directly create a new config without relying on environment variables
        let config = TradierRestApiConfig::new(
            Cow::Borrowed("https://api.tradier.com/v1/"),
            SecretString::new("test_token".into()),
        );

        // Assert the custom endpoint is set correctly
        assert_eq!(config.endpoint, "https://api.tradier.com/v1/");
        assert_eq!(config.access_token.expose_secret(), "test_token");
    }

    #[tokio::test]
    async fn dynamic_endpoint_conversion() {
        // Set up environment variable and load the default configuration
        env::set_var("TRADIER_API_ACCESS_TOKEN", "test_token");
        let mut config = TradierRestApiConfig::load_from_env()
            .await
            .expect("Failed to load config");

        // Update endpoint to a dynamically allocated string
        let dynamic_endpoint = "https://api.tradier.com/v1/".to_string();
        config.endpoint = Cow::Owned(dynamic_endpoint.clone());

        // Assert the endpoint matches the new dynamic string
        assert_eq!(config.endpoint, dynamic_endpoint);

        // Clean up environment variable
        env::remove_var("TRADIER_API_ACCESS_TOKEN");
    }
}
