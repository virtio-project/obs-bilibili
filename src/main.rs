use crate::web::spawn_server;

mod error;
mod web;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    spawn_server().await?;
    Ok(())
}
