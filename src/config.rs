// config.rs

#[derive(Clone, Debug)]
pub struct Config
{
    pub host: String,
    pub port: u16,
    pub static_dir: String,

    pub cors_allowed_origins: Vec<String>,
    pub http_request_timeout_ms: u64,
    pub http_max_body_bytes: usize,
    pub shutdown_timeout_ms: u64,
}

impl Config
{
    pub fn from_env() -> Config
    {
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(3030);

        let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "static".to_string());
        let cors_allowed_origins = read_csv_env("CORS_ALLOWED_ORIGINS");
        let http_request_timeout_ms = read_u64_env("HTTP_REQUEST_TIMEOUT_MS", 10_000);
        let http_max_body_bytes = read_usize_env("HTTP_MAX_BODY_BYTES", 1 * 1024 * 1024); // 1MB
        let shutdown_timeout_ms = read_u64_env("SHUTDOWN_TIMEOUT_MS", 10_000);

        Config
        {
            host,
            port,
            static_dir,
            cors_allowed_origins,
            http_request_timeout_ms,
            http_max_body_bytes,
            shutdown_timeout_ms,
        }
    }

    pub fn listen_addr(&self) -> String
    {
        format!("{}:{}", self.host, self.port)
    }
}

fn read_csv_env(key: &str) -> Vec<String>
{
    let v = match std::env::var(key)
    {
        Ok(v) => v,
        Err(_) => String::new(),
    };

    v.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn read_u64_env(key: &str, default_value: u64) -> u64
{
    match std::env::var(key)
    {
        Ok(v) =>
        {
            match v.parse::<u64>()
            {
                Ok(n) => n,
                Err(_) => default_value,
            }
        }
        Err(_) => default_value,
    }
}

fn read_usize_env(key: &str, default_value: usize) -> usize
{
    match std::env::var(key)
    {
        Ok(v) =>
        {
            match v.parse::<usize>()
            {
                Ok(n) => n,
                Err(_) => default_value,
            }
        }
        Err(_) => default_value,
    }
}
