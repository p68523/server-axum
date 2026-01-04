mod app;
mod config;
mod error;
mod models;
mod routes;
mod state;
mod ws;

use crate::config::Config;

#[tokio::main]
async fn main()
{
    init_tracing();

    let config = Config::from_env();
    let app = app::build_app(config.clone());

    let addr = config.listen_addr();
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn init_tracing()
{
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
