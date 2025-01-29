use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use central_account_service::config::Config;
use central_account_service::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    sqlx::migrate!().run(&db).await?;

    http::serve(config, db).await?;

    Ok(())
}
