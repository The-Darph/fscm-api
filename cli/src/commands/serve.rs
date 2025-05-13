use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::fmt;
use tracing_subscriber::util::SubscriberInitExt;
use clap::{Arg, ArgMatches, Command};
use clap::value_parser;
use std::net::SocketAddr;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::sync::Arc;
use crate::settings::Settings;
use crate::state::ApplicationState;

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(
    matches: &ArgMatches, 
    settings: &Settings
) -> anyhow::Result<()> {
    let port: u16 = *matches.get_one("port").unwrap_or(&8080);

    start_tokio(port, settings)?;

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            // Logging
            let subscriber = tracing_subscriber::registry()
                .with(LevelFilter::from_level(Level::TRACE))
                .with(fmt::Layer::default());

            subscriber.init();

            let state = Arc::new(ApplicationState::new(settings)?);
            let router = crate::api::configure(state)
                .layer(TraceLayer::new_for_http());

            // Application startup (load config & start server)
            let state = Arc::new(ApplicationState::new(settings)?);
            let router = crate::api::configure(state);

            let addr = SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 
                port
            );

            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router.into_make_service()).await?;

            Ok::<(), anyhow::Error>(())
        })?;

    Ok(())
}
