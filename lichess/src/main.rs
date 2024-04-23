use server::LichessServer;

pub mod incoming_events;
pub mod incoming_game_state;
pub mod player;
pub mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = LichessServer::new()?;
    server.run().await?;
    Ok(())
}
