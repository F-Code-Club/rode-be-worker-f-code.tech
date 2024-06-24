mod app_state;
mod config;
mod update_score;

use std::net::SocketAddr;

use app_state::AppState;
use config::server::SERVER_PORT;
use tokio::net::TcpListener;
use tokio::time::interval;
use update_score::update_score_tables;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), *SERVER_PORT)).await?;
    println!("Listening on port: 0.0.0.0:{:?}", *SERVER_PORT);

    let mut interval = interval(std::time::Duration::from_secs(5));

    loop {
        interval.tick().await;

        update_score_tables(&AppState::new().await?.database).await?;
    }
}
