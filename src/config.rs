#[derive(Clone, Debug)]
pub struct Config
{
    pub host: String,
    pub port: u16,
    pub static_dir: String,
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

        Config { host, port, static_dir }
    }

    pub fn listen_addr(&self) -> String
    {
        format!("{}:{}", self.host, self.port)
    }
}
