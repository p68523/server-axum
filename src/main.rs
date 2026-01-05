// main.rs

mod app;
mod config;
mod error;
mod models;
mod routes;
mod state;
mod ws;

use std::time::Duration;

use crate::config::Config;

#[tokio::main]
async fn main()
{
    init_tracing();

    let config = Config::from_env();
    let app = crate::app::build_app(config.clone());
    let addr = config.listen_addr();
    tracing::info!("listening on {}", addr);

    let listener = match tokio::net::TcpListener::bind(addr.as_str()).await
    {
        Ok(v) => v,
        Err(e) =>
        {
            tracing::error!("failed to bind {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let server = axum::serve(listener, app).with_graceful_shutdown(async move
    {
        let _ = shutdown_rx.await;
    });

    let server_handle = tokio::spawn(async move
    {
        server.await
    });

    shutdown_signal().await;
    tracing::info!("shutdown signal received");

    let shutdown_timeout = Duration::from_millis(config.shutdown_timeout_ms);

    match tokio::time::timeout(shutdown_timeout, server_handle).await
    {
        Ok(join_result) =>
        {
            match join_result
            {
                Ok(Ok(())) =>
                {
                    tracing::info!("graceful shutdown completed");
                }
                Ok(Err(e)) =>
                {
                    tracing::error!("server error: {}", e);
                    std::process::exit(1);
                }
                Err(e) =>
                {
                    tracing::error!("server task join error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(_) =>
        {
            tracing::warn!("graceful shutdown timed out, forcing exit");
            std::process::exit(1);
        }
    }
}

fn init_tracing()
{
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

async fn shutdown_signal()
{
    let ctrl_c = async
    {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async
    {
        use tokio::signal::unix::{signal, SignalKind};

        match signal(SignalKind::terminate())
        {
            Ok(mut sigterm) =>
            {
                sigterm.recv().await;
            }
            Err(_) =>
            {
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = async
    {
        std::future::pending::<()>().await;
    };

    tokio::select!
    {
        _ = ctrl_c =>
        {
            tracing::info!("received Ctrl+C");
        }
        _ = terminate =>
        {
            tracing::info!("received SIGTERM");
        }
    }
}