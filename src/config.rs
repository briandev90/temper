use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub etherscan_key: Option<String>,
    pub api_key: Option<String>,
    pub max_request_size: u64,
}

pub fn config() -> Config {
    dotenv().ok();

    load_config()
}

fn load_config() -> Config {
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16.");
    let etherscan_key = std::env::var("ETHERSCAN_KEY")
        .ok()
        .filter(|k| !k.is_empty());
    let api_key = std::env::var("API_KEY").ok().filter(|k| !k.is_empty());
    let max_request_size = std::env::var("MAX_REQUEST_SIZE")
        .unwrap_or("16".to_string())
        .parse::<u64>()
        .expect("MAX_REQUEST_SIZE must be a valid u64")
        * 1024;

    Config {
        port,
        etherscan_key,
        api_key,
        max_request_size,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "PORT must be a valid u16.")]
    fn test_config_port_number() {
        temp_env::with_var("PORT", Some("not a number"), || {
            super::load_config();
        });
    }

    #[test]
    fn test_config_etherscan_key() {
        temp_env::with_vars([("ETHERSCAN_KEY", Some("a"))], || {
            let config = super::load_config();
            assert_eq!(config.etherscan_key, Some("a".to_string()));
        });

        temp_env::with_vars([("ETHERSCAN_KEY", Some(""))], || {
            let config = super::load_config();
            assert_eq!(config.etherscan_key, None);
        });

        temp_env::with_vars_unset([("ETHERSCAN_KEY")], || {
            let config = super::load_config();
            assert_eq!(config.etherscan_key, None);
        });
    }

    #[test]
    fn test_config_api_key() {
        temp_env::with_vars([("API_KEY", Some("a"))], || {
            let config = super::load_config();
            assert_eq!(config.api_key, Some("a".to_string()));
        });

        temp_env::with_vars([("API_KEY", Some(""))], || {
            let config = super::load_config();
            assert_eq!(config.api_key, None);
        });

        temp_env::with_vars_unset([("API_KEY")], || {
            let config = super::load_config();
            assert_eq!(config.api_key, None);
        });
    }
}
