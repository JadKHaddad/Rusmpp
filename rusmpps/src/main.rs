use clap::Parser;
use rusmpps::{
    args::Args,
    config::Config,
    server::{Server, ServerParameters},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmpps=debug")
        .init();

    dotenvy::dotenv().ok();

    let args = Args::parse();

    let config = Config::from_yaml_file(args.config_file).unwrap_or_else(|err| {
        tracing::error!("Failed to load config: {}", err);
        tracing::warn!("Using default configuration");

        Config::default()
    });

    tracing::info!(?config);

    let parameters = ServerParameters {
        clients: vec![],
        enquire_link_interval: config.enquire_link_interval,
        enquire_link_response_timeout: config.enquire_link_response_timeout,
        enquire_link_response_delay: config.enquire_link_response_delay,
        session_timeout: config.session_timeout,
        bind_delay: config.bind_delay,
        response_delay: config.response_delay,
        socket_addr: config.socket_addr,
    };

    let server = Server::new(parameters);

    tracing::info!("Starting server");

    tokio::select! {
        result = server.run() => {
            result?;
        }
        _ = shutdown_signal() => {

        }
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");

        tracing::info!("CTRL+C received");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;

        tracing::info!("SIGTERM received");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down");
}
