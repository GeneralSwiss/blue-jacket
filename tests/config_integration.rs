use blue_jacket::data::tradier::TradierRestApiConfig;
use secrecy::ExposeSecret;
use std::env;

#[tokio::test]
async fn load_from_env_success() {
    // Set environment variables needed for the test
    env::set_var("TRADIER_API_ACCESS_TOKEN", "integration_test_token");

    // Load configuration from environment variables
    let config = TradierRestApiConfig::load_from_env()
        .await
        .expect("Failed to load config");

    // Assert that the default endpoint is the expected sandbox URL
    assert_eq!(config.endpoint, "https://sandbox.tradier.com/v1/");
    assert_eq!(config.access_token.expose_secret(), "integration_test_token");

    // Clean up the environment variable after the test
    env::remove_var("TRADIER_API_ACCESS_TOKEN");
}