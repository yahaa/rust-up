#[macro_use]
extern crate log;

mod secret;

use kube::Client;

use chrono::Local;
use std::io::Write;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::Builder::from_env(env_logger::Env::default())
        .format(|buf, record| {
            let level = { buf.default_styled_level(record.level()) };
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                &record.args()
            )
        })
        .init();

    let client = Client::try_default().await?;

    tokio::spawn(secret::watch_ns(
        client.clone(),
        "default",
        "docker-registry",
    ));

    tokio::spawn(secret::watch_config_secret(
        client,
        "default",
        "docker-registry",
    ));

    signal(SignalKind::terminate())?.recv().await;

    info!("recv SIGTERM, graceful shutdown...");

    Ok(())
}
