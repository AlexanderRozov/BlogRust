use blog::Config;

#[test]
fn test_config_from_env_defaults() {
    // Test that config can be created with defaults
    let config = Config::from_env();
    
    assert!(!config.database_url.is_empty());
    assert!(!config.session_secret.is_empty());
    assert!(config.port > 0);
}

#[test]
fn test_config_port_default() {
    std::env::remove_var("PORT");
    let config = Config::from_env();
    assert_eq!(config.port, 3000);
}

