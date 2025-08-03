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

    server.run().await?;

    Ok(())
}
