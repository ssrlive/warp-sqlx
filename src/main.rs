use std::{env, net::SocketAddr, sync::Arc};

mod db;
mod model;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    let db = db::init_db().await?;
    let apis = web::product_rest_filter("api", Arc::new(db));
    let addr = "127.0.0.1:3030".parse::<SocketAddr>().map_err(|e| anyhow::anyhow!(e))?;
    warp::serve(apis).run(addr).await;
    Ok(())
}
