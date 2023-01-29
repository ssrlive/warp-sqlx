use std::env;

mod db;
mod model;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    let _db = db::init_db().await?;
    Ok(())
}
