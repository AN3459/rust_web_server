// use serde_derive::Deserialize;
use lazy_static::lazy_static;

// #[derive(Debug, Deserialize)]
// pub struct Config {
//     pub running_env: String,
//     pub app: AppConfig,
//     pub db: DbConfig,
// }

// #[derive(Debug, Deserialize)]
// pub struct AppConfig {
//     pub port: u16,
// }

// #[derive(Debug, Deserialize)]
// pub struct DbConfig {
//     pub host: String,
//     pub port: u16,
//     pub user: String,
//     pub password: String,
//     pub database: String,
//     pub connection_timeout: u64,
//     pub max_query_execution_time: u64,
// }

// impl Config {
//     pub fn new() -> Self {
//         Config {
//             running_env: std::env::var("RUNNING_ENV").unwrap_or_else(|_| "dev".to_string()),
//             app: AppConfig { port: std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse()
//             .expect("Failed to parse") },
//             db: DbConfig {
//                 host: std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "test".to_string()),
//                 port: std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()).parse()
//                 .expect("Failed to parse"),
//                 user: std::env::var("POSTGRES_USER").unwrap_or_else(|_| "test".to_string()),
//                 password: std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "test".to_string()),
//                 database: std::env::var("POSTGRES_DB").unwrap_or_else(|_| "test".to_string()),
//                 connection_timeout: std::env::var("POSTGRES_CONNECTION_TIMEOUT").unwrap_or_else(|_| "5000".to_string()).parse()
//                 .expect("Failed to parse"),
//                 max_query_execution_time: std::env::var("POSTGRES_MAX_QUERY_EXECUTION_TIME").unwrap_or_else(|_| "10000".to_string()).parse()
//                 .expect("Failed to parse"),
//             },
//         }
//     }
// }

const PARSE_ERROR_MSG: &str = "Failed to parse";

// runing
lazy_static! {
    pub static ref RUNNING_ENV: String = get_env_var_or_default("RUNNING_ENV", "dev");
}

// app
lazy_static! {
    pub static ref PORT: u16 =
        get_env_var_or_default("PORT", "3000").parse().expect(PARSE_ERROR_MSG);
}

// db
lazy_static! {
    pub static ref POSTGRES_HOST: String =
        get_env_var_or_default("POSTGRES_HOST", "http://172.18.0.2:5432");
    pub static ref POSTGRES_PORT: u16 =
        get_env_var_or_default("PORT", "3000").parse().expect(PARSE_ERROR_MSG);
    pub static ref POSTGRES_USER: String = get_env_var_or_default("POSTGRES_USER", "test");
    pub static ref POSTGRES_PASSWORD: String = get_env_var_or_default("POSTGRES_PASSWORD", "test");
    pub static ref POSTGRES_DB: String = get_env_var_or_default("POSTGRES_DB", "test");
    pub static ref POSTGRES_CONNECTION_TIMEOUT: u64 =
        get_env_var_or_default("POSTGRES_CONNECTION_TIMEOUT", "5000")
            .parse()
            .expect(PARSE_ERROR_MSG);
    pub static ref POSTGRES_MAX_QUERY_EXECUTION_TIME: u64 =
        get_env_var_or_default("POSTGRES_MAX_QUERY_EXECUTION_TIME", "10000")
            .parse()
            .expect(PARSE_ERROR_MSG);
}

fn get_env_var_or_default(var_name: &str, default_value: &str) -> String {
    if let Ok(val) = std::env::var(var_name) {
        val
    } else {
        String::from(default_value)
    }
}
