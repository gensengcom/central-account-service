#[derive(clap::Parser)]
pub struct Config {
    /// The entire Postgres database URL.
    #[clap(long, env)]
    pub database_url: String,

    /// The HMAC signing and verification key used for login tokens.
    #[clap(long, env)]
    pub hmac_key: String,
}
